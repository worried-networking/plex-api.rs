use futures::AsyncWrite;
use http::StatusCode;
use isahc::AsyncReadResponseExt;
use serde::Deserialize;

use crate::{
    isahc_compat::StatusCodeExt,
    media_container::{
        server::{
            library::{
                AudioCodec, AudioStream, ContainerFormat, Decision, Media as MediaMetadata,
                Metadata, Protocol, Stream, VideoCodec, VideoStream,
            },
            Feature,
        },
        MediaContainer, MediaContainerWrapper,
    },
    server::Query,
    transcode::{
        bs, get_transcode_params, session_id, Context, DecisionResult, TranscodeOptions,
        TranscodeSessionStats,
    },
    url::{
        SERVER_TRANSCODE_DECISION, SERVER_TRANSCODE_DOWNLOAD, SERVER_TRANSCODE_SESSIONS,
        SERVER_TRANSCODE_STOP,
    },
    Error, HttpClient, Result,
};

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct TranscodeSessionsMediaContainer {
    #[serde(default, rename = "TranscodeSession")]
    pub(crate) transcode_sessions: Vec<TranscodeSessionStats>,
}

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "tests_deny_unknown_fields", serde(deny_unknown_fields))]
struct TranscodeDecisionMediaContainer {
    #[serde(flatten)]
    decision_result: DecisionResult,

    allow_sync: String,
    #[serde(rename = "librarySectionID")]
    library_section_id: Option<String>,
    library_section_title: Option<String>,
    #[serde(rename = "librarySectionUUID")]
    library_section_uuid: Option<String>,
    media_tag_prefix: Option<String>,
    media_tag_version: Option<String>,
    resource_session: Option<String>,

    #[serde(flatten)]
    media_container: MediaContainer,

    #[serde(default, rename = "Metadata")]
    metadata: Vec<Metadata>,
}

async fn transcode_decision(client: &HttpClient, params: &Query) -> Result<MediaMetadata> {
    let path = format!("{SERVER_TRANSCODE_DECISION}?{params}");

    let mut response = client
        .get(path)
        .header("Accept", "application/json")
        .send()
        .await?;

    let text = match response.status().as_http_status() {
        StatusCode::OK => response.text().await?,
        _ => return Err(crate::Error::from_response(response).await),
    };

    let wrapper: MediaContainerWrapper<TranscodeDecisionMediaContainer> =
        serde_json::from_str(&text)?;

    if wrapper
        .media_container
        .decision_result
        .general_decision_code
        == Some(2011)
        && wrapper
            .media_container
            .decision_result
            .general_decision_text
            == Some("Downloads not allowed".to_string())
    {
        return Err(Error::SubscriptionFeatureNotAvailable(Feature::SyncV3));
    }

    if wrapper
        .media_container
        .decision_result
        .direct_play_decision_code
        == Some(1000)
    {
        return Err(Error::TranscodeRefused);
    }

    wrapper
        .media_container
        .metadata
        .into_iter()
        .next()
        .and_then(|m| m.media)
        .and_then(|m| m.into_iter().find(|m| m.selected == Some(true)))
        .ok_or_else(|| {
            if let Some(text) = wrapper
                .media_container
                .decision_result
                .transcode_decision_text
            {
                Error::TranscodeError(text)
            } else {
                Error::UnexpectedApiResponse {
                    status_code: response.status().as_u16(),
                    content: text,
                }
            }
        })
}

pub(crate) async fn create_transcode_session<O: TranscodeOptions>(
    client: &HttpClient,
    item_metadata: &Metadata,
    context: Context,
    target_protocol: Protocol,
    media_index: Option<usize>,
    part_index: Option<usize>,
    options: O,
) -> Result<TranscodeSession> {
    let id = session_id();

    let mut params = get_transcode_params(
        &id,
        context,
        target_protocol,
        media_index,
        part_index,
        options,
    )?
    .param("path", item_metadata.key.clone());

    if context == Context::Static {
        params = params.param("offlineTranscode", bs(true));
    }

    let media_data = transcode_decision(client, &params).await?;

    if target_protocol != media_data.protocol.unwrap_or(Protocol::Http) {
        return Err(Error::TranscodeError(
            "Server returned an invalid protocol.".to_string(),
        ));
    }

    TranscodeSession::from_metadata(
        id,
        client.clone(),
        media_data,
        context == Context::Static,
        params,
    )
}

pub(crate) async fn transcode_session_stats(
    client: &HttpClient,
    session_id: &str,
) -> Result<TranscodeSessionStats> {
    let wrapper: MediaContainerWrapper<TranscodeSessionsMediaContainer> = match client
        .get(format!("{SERVER_TRANSCODE_SESSIONS}/{session_id}"))
        .json()
        .await
    {
        Ok(w) => w,
        Err(Error::UnexpectedApiResponse {
            status_code: 404,
            content: _,
        }) => {
            return Err(crate::Error::ItemNotFound);
        }
        Err(e) => return Err(e),
    };
    wrapper
        .media_container
        .transcode_sessions
        .first()
        .cloned()
        .ok_or(crate::Error::ItemNotFound)
}

#[derive(Clone, Copy)]
pub enum TranscodeStatus {
    Complete,
    Error,
    Transcoding {
        // The server's estimate of how many seconds are left until complete.
        remaining: Option<u32>,
        // Percent complete (0-100).
        progress: f32,
    },
}

pub struct TranscodeSession {
    id: String,
    client: HttpClient,
    offline: bool,
    protocol: Protocol,
    container: ContainerFormat,
    video_transcode: Option<(Decision, VideoCodec)>,
    audio_transcode: Option<(Decision, AudioCodec)>,
    params: Query,
}

impl TranscodeSession {
    pub(crate) fn from_stats(client: HttpClient, stats: TranscodeSessionStats) -> Self {
        Self {
            client,
            // Once the transcode session is started we only need the session ID
            // to download.
            params: Query::new().param("session", &stats.key),
            offline: stats.offline_transcode,
            container: stats.container,
            protocol: stats.protocol,
            video_transcode: stats.video_decision.zip(stats.video_codec),
            audio_transcode: stats.audio_decision.zip(stats.audio_codec),
            id: stats.key,
        }
    }

    fn from_metadata(
        id: String,
        client: HttpClient,
        media_data: MediaMetadata,
        offline: bool,
        params: Query,
    ) -> Result<Self> {
        let part_data = media_data
            .parts
            .iter()
            .find(|p| p.selected == Some(true))
            .ok_or_else(|| {
                Error::TranscodeError("Server returned unexpected response".to_string())
            })?;

        let streams = part_data.streams.as_ref().ok_or_else(|| {
            Error::TranscodeError("Server returned unexpected response".to_string())
        })?;

        let video_streams = streams
            .iter()
            .filter_map(|s| match s {
                Stream::Video(s) => Some(s),
                _ => None,
            })
            .collect::<Vec<&VideoStream>>();

        let video_transcode = video_streams
            .iter()
            .find(|s| s.selected == Some(true))
            .or_else(|| video_streams.first())
            .map(|s| (s.decision.unwrap(), s.codec));

        let audio_streams = streams
            .iter()
            .filter_map(|s| match s {
                Stream::Audio(s) => Some(s),
                _ => None,
            })
            .collect::<Vec<&AudioStream>>();

        let audio_transcode = audio_streams
            .iter()
            .find(|s| s.selected == Some(true))
            .or_else(|| audio_streams.first())
            .map(|s| (s.decision.unwrap(), s.codec));

        Ok(Self {
            id,
            client,
            offline,
            params,
            container: media_data.container.unwrap(),
            protocol: media_data.protocol.unwrap_or(Protocol::Http),
            video_transcode,
            audio_transcode,
        })
    }

    /// The session ID allows for re-retrieving this session at a later date.
    pub fn session_id(&self) -> &str {
        &self.id
    }

    pub fn is_offline(&self) -> bool {
        self.offline
    }

    /// The selected protocol.
    pub fn protocol(&self) -> Protocol {
        self.protocol
    }

    /// The selected container.
    pub fn container(&self) -> ContainerFormat {
        self.container
    }

    // The target video codec and the transcode decision.
    pub fn video_transcode(&self) -> Option<(Decision, VideoCodec)> {
        self.video_transcode
    }

    // The target audio codec and the transcode decision.
    pub fn audio_transcode(&self) -> Option<(Decision, AudioCodec)> {
        self.audio_transcode
    }

    /// Downloads the transcoded data to the provided writer.
    ///
    /// For streaming transcodes (MPEG-DASH or HLS) this will return the
    /// playlist data. This crate doesn't contain any support for processing
    /// these streaming formats and figuring out how to use them is currently
    /// left as an exercise for the caller.
    ///
    /// For offline transcodes it is possible to start downloading before the
    /// transcode is complete. In this case any data already transcoded is
    /// downloaded and then the connection will remain open and more data will
    /// be delivered to the writer as it becomes available. This can mean
    /// that the HTTP connection is idle for long periods of time waiting for
    /// more data to be transcoded and so the normal timeouts are disabled for
    /// offline transcode downloads.
    ///
    /// Unfortunately there does not appear to be any way to restart downloads
    /// from a specific point in the file. So if the download fails for
    /// any reason you have to start downloading all over again. It may make
    /// more sense to wait until the transcode is complete or nearly complete
    /// before attempting download.
    #[tracing::instrument(level = "debug", skip_all)]
    pub async fn download<W>(&self, writer: W) -> Result<()>
    where
        W: AsyncWrite + Unpin,
    {
        // Strictly speaking it doesn't appear that the requested extension
        // matters but we'll attempt to match other clients anyway.
        let ext = match (self.protocol, self.container) {
            (Protocol::Dash, _) => "mpd".to_string(),
            (Protocol::Hls, _) => "m3u8".to_string(),
            (_, container) => container.to_string(),
        };

        let path = format!(
            "{}?{}",
            SERVER_TRANSCODE_DOWNLOAD.replace("{extension}", &ext),
            self.params
        );

        let mut builder = self.client.get(path);
        if self.offline {
            builder = builder.timeout(None)
        }
        let mut response = builder.send().await?;

        match response.status().as_http_status() {
            StatusCode::OK => {
                response.copy_to(writer).await?;
                Ok(())
            }
            _ => Err(crate::Error::from_response(response).await),
        }
    }

    #[tracing::instrument(level = "debug", skip_all)]
    pub async fn status(&self) -> Result<TranscodeStatus> {
        let stats = self.stats().await?;

        if stats.error {
            Ok(TranscodeStatus::Error)
        } else if stats.complete {
            Ok(TranscodeStatus::Complete)
        } else {
            Ok(TranscodeStatus::Transcoding {
                remaining: stats.remaining,
                progress: stats.progress,
            })
        }
    }

    /// Retrieves the current transcode stats.
    #[tracing::instrument(level = "debug", skip_all)]
    pub async fn stats(&self) -> Result<TranscodeSessionStats> {
        transcode_session_stats(&self.client, &self.id).await
    }

    /// Cancels the transcode and removes any transcoded data from the server.
    ///
    /// NB! Be careful with cancelling sessions too often! Cancelling a few transcoding
    /// sessions in a short succession, or cancelling a session shortly after it was
    /// initiated might crash the Plex server. At least the one running inside a Linux
    /// Docker Container.
    #[tracing::instrument(level = "debug", skip_all)]
    pub async fn cancel(self) -> Result<()> {
        let mut response = self
            .client
            .get(format!("{SERVER_TRANSCODE_STOP}?session={}", self.id))
            .send()
            .await?;

        match response.status().as_http_status() {
            // Sometimes the server will respond not found but still cancel the
            // session.
            StatusCode::OK | StatusCode::NOT_FOUND => Ok(response.consume().await?),
            _ => Err(crate::Error::from_response(response).await),
        }
    }
}
