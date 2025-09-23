use std::{fmt, ops::RangeBounds, str::FromStr};

use content_disposition::parse_content_disposition;
use futures::AsyncWrite;
use http::StatusCode;
use isahc::{
    http::header::CONTENT_DISPOSITION, http::header::CONTENT_LENGTH, AsyncReadResponseExt,
};
use serde::Deserialize;
use serde_json::Value;

use crate::{
    isahc_compat::StatusCodeExt,
    media_container::{
        server::library::{ContainerFormat, Metadata, Protocol},
        MediaContainerWrapper,
    },
    transcode::{
        get_transcode_params, session_id, Context, DecisionResult, TranscodeOptions,
        TranscodeSessionStats,
    },
    url::{
        DOWNLOAD_QUEUE_ADD, DOWNLOAD_QUEUE_CREATE, DOWNLOAD_QUEUE_DOWNLOAD, DOWNLOAD_QUEUE_ITEM,
        DOWNLOAD_QUEUE_LIST,
    },
    Error, HttpClient, Result,
};

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
enum QueueStatus {
    Deciding,
    Waiting,
    Processing,
    Done,
    Error,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum QueueItemStatus {
    /// The server is deciding whether to transcode this item or not.
    Deciding,
    /// The item is waiting in the queue to be transcoded.
    Waiting,
    /// The item is currently being transcoded.
    Processing,
    /// The item is available for download. Either transcoding is complete or was not required.
    Available,
    /// An error occurred.
    Error,
    /// The transcoded item has timed out and is no longer available.
    Expired,
}

#[derive(Deserialize)]
#[allow(dead_code)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "tests_deny_unknown_fields", serde(deny_unknown_fields))]
struct QueueSpec {
    id: u32,
    owner: Option<u32>,
    client_identifier: Option<String>,
    item_count: u32,
    status: QueueStatus,
}

#[derive(Deserialize)]
struct DownloadQueueContainer {
    #[serde(rename = "DownloadQueue", default)]
    queues: Vec<QueueSpec>,
}

#[derive(Deserialize)]
struct QueueAddedItem {
    key: String,
    id: u32,
}

#[derive(Deserialize)]
struct QueueAddedContainer {
    #[serde(rename = "AddedQueueItems", default)]
    items: Vec<QueueAddedItem>,
}

#[derive(Clone, Debug)]
/// A download queue on the server.
///
/// Each server maintains one download queue per user per device.
pub struct DownloadQueue {
    client: HttpClient,
    id: u32,
}

impl PartialEq for DownloadQueue {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.client.x_plex_client_identifier == other.client.x_plex_client_identifier
    }
}

impl DownloadQueue {
    pub(crate) async fn get_or_create(client: HttpClient) -> Result<Self> {
        let wrapper: MediaContainerWrapper<DownloadQueueContainer> =
            client.post(DOWNLOAD_QUEUE_CREATE).json().await?;

        if let Some(queue) = wrapper.media_container.queues.first() {
            Ok(Self {
                client,
                id: queue.id,
            })
        } else {
            Err(Error::ItemNotFound)
        }
    }

    /// Lists the items in this download queue.
    pub async fn items(&self) -> Result<Vec<QueueItem>> {
        Ok(self
            .client
            .get(DOWNLOAD_QUEUE_LIST.replace("{queueId}", &self.id.to_string()))
            .json::<MediaContainerWrapper<QueueItemContainer>>()
            .await?
            .media_container
            .items
            .into_iter()
            .map(|item| QueueItem {
                client: self.client.clone(),
                state: item,
            })
            .collect())
    }

    /// Gets a specific item in this download queue by its ID.
    pub async fn item(&self, id: u32) -> Result<QueueItem> {
        let state = QueueItemState::fetch(&self.client, self.id, id).await?;

        Ok(QueueItem {
            client: self.client.clone(),
            state,
        })
    }

    /// Adds a media item to this download queue with the given transcode options.
    ///
    /// Adding the same media with the same options will return the existing item in the queue.
    /// You can pass either the main item (in which case the server selects which media to use and
    /// combines all parts) or specific media or a specific part.
    pub(crate) async fn add_item<O: TranscodeOptions>(
        &self,
        metadata: &Metadata,
        media_index: Option<usize>,
        part_index: Option<usize>,
        options: O,
    ) -> Result<QueueItem> {
        let id = session_id();
        let key = &metadata.key;

        let params = get_transcode_params(
            &id,
            Context::Static,
            Protocol::Http,
            media_index,
            part_index,
            options,
        )?
        .param("keys", &metadata.key)
        .param("path", &metadata.key);

        let wrapper: MediaContainerWrapper<QueueAddedContainer> = self
            .client
            .post(format!(
                "{}?{params}",
                DOWNLOAD_QUEUE_ADD.replace("{queueId}", &self.id.to_string())
            ))
            .json()
            .await?;

        if let Some(item) = wrapper.media_container.items.iter().find(|i| &i.key == key) {
            let item = QueueItemState::fetch(&self.client, self.id, item.id).await?;

            Ok(QueueItem {
                client: self.client.clone(),
                state: item,
            })
        } else {
            Err(Error::ItemNotFound)
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "tests_deny_unknown_fields", serde(deny_unknown_fields))]
struct QueueItemState {
    id: u32,
    queue_id: u32,
    key: String,
    status: QueueItemStatus,
    error: Option<String>,
    // The API docs says this is the transcode session object. I've never seen it as anything other
    // than null though.
    transcode: Option<Value>,
    #[serde(rename = "DecisionResult")]
    decision_result: DecisionResult,
    #[serde(rename = "TranscodeSession")]
    session_stats: Option<TranscodeSessionStats>,
}

impl QueueItemState {
    async fn fetch(client: &HttpClient, queue_id: u32, id: u32) -> Result<Self> {
        let items = client
            .get(
                DOWNLOAD_QUEUE_ITEM
                    .replace("{queueId}", &queue_id.to_string())
                    .replace("{itemId}", &id.to_string()),
            )
            .json::<MediaContainerWrapper<QueueItemContainer>>()
            .await?
            .media_container
            .items;

        items.into_iter().next().ok_or_else(|| Error::ItemNotFound)
    }
}

#[derive(Deserialize)]
struct QueueItemContainer {
    #[serde(rename = "DownloadQueueItem", default)]
    items: Vec<QueueItemState>,
}

/// An item in a download queue.
pub struct QueueItem {
    client: HttpClient,
    state: QueueItemState,
}

impl fmt::Debug for QueueItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.state.fmt(f)
    }
}

impl QueueItem {
    pub fn id(&self) -> u32 {
        self.state.id
    }

    pub fn queue(&self) -> DownloadQueue {
        DownloadQueue {
            client: self.client.clone(),
            id: self.state.queue_id,
        }
    }

    pub fn key(&self) -> &str {
        &self.state.key
    }

    pub fn status(&self) -> QueueItemStatus {
        self.state.status.clone()
    }

    /// If this item is currently being transcoded this will return the current
    /// transcode stats.
    pub fn stats(&self) -> Option<TranscodeSessionStats> {
        self.state.session_stats.clone()
    }

    /// If the status is an error this may reveal more information.
    pub fn error(&self) -> Option<&str> {
        self.state.error.as_deref()
    }

    /// Returns true if this item was or is being transcoded. If false then
    /// downloading will just download the original media file.
    pub fn is_transcode(&self) -> bool {
        self.state.decision_result.direct_play_decision_code != Some(1000)
    }

    /// Returns the container format of the file that will be downloaded.
    ///
    /// This will fail if the item is not available.
    pub async fn container(&self) -> Result<ContainerFormat> {
        // The API doesn't appear to expose the container format in a
        // a particularly nice way. If the item is in the middle of transcoding
        // then it is available in the transcode stats, but if already complete
        // the stats are no longer exposed. However the content-disposition
        // header of the download endpoint does include the filename complete
        // with correct extension for the container so we can use that.

        let path = DOWNLOAD_QUEUE_DOWNLOAD
            .replace("{queueId}", &self.state.queue_id.to_string())
            .replace("{itemId}", &self.state.id.to_string());

        let response = self.client.head(path).send().await?;
        match response.status().as_http_status() {
            StatusCode::OK => {
                if let Some(val) = response.headers().get(CONTENT_DISPOSITION) {
                    if let Ok(st) = val.to_str() {
                        if let Some((_, Some(ext))) = parse_content_disposition(st).filename() {
                            match ContainerFormat::from_str(&ext) {
                                Ok(c) => Ok(c),
                                Err(_) => Err(Error::UnknownContainerFormat(ext.to_string())),
                            }
                        } else {
                            Err(Error::InvalidHeaderValue)
                        }
                    } else {
                        Err(Error::InvalidHeaderValue)
                    }
                } else {
                    Err(Error::InvalidHeaderValue)
                }
            }
            StatusCode::SERVICE_UNAVAILABLE => Err(Error::TranscodeIncomplete),
            _ => Err(crate::Error::from_response(response).await),
        }
    }

    /// Returns the expected length of the download.
    ///
    /// This will fail if the item is not available.
    pub async fn content_length(&self) -> Result<Option<u64>> {
        let path = DOWNLOAD_QUEUE_DOWNLOAD
            .replace("{queueId}", &self.state.queue_id.to_string())
            .replace("{itemId}", &self.state.id.to_string());

        let response = self.client.head(path).send().await?;
        match response.status().as_http_status() {
            StatusCode::OK => {
                if let Some(val) = response.headers().get(CONTENT_LENGTH) {
                    if let Ok(st) = val.to_str() {
                        Ok(st.parse::<u64>().ok())
                    } else {
                        Ok(None)
                    }
                } else {
                    Ok(None)
                }
            }
            StatusCode::SERVICE_UNAVAILABLE => Err(Error::TranscodeIncomplete),
            _ => Err(crate::Error::from_response(response).await),
        }
    }

    /// Updates the state of this item by re-fetching it from the server.
    pub async fn update(&mut self) -> Result<()> {
        let state = QueueItemState::fetch(&self.client, self.state.queue_id, self.state.id).await?;
        self.state = state;
        Ok(())
    }

    /// Downloads the item to the provided writer.
    ///
    /// This will fail if the item is not available.
    pub async fn download<W, R>(&self, writer: W, range: R) -> Result
    where
        W: AsyncWrite + Unpin,
        R: RangeBounds<u64>,
    {
        let path = DOWNLOAD_QUEUE_DOWNLOAD
            .replace("{queueId}", &self.state.queue_id.to_string())
            .replace("{itemId}", &self.state.id.to_string());

        let start = match range.start_bound() {
            std::ops::Bound::Included(v) => *v,
            std::ops::Bound::Excluded(v) => v + 1,
            std::ops::Bound::Unbounded => 0,
        };

        let end = match range.end_bound() {
            std::ops::Bound::Included(v) => Some(*v),
            std::ops::Bound::Excluded(v) => Some(v - 1),
            std::ops::Bound::Unbounded => None,
        };

        let mut builder = self.client.get(path).timeout(None);
        if start != 0 || end.is_some() {
            // We're requesting part of the file.
            let end = end.map(|v| v.to_string()).unwrap_or_default();
            builder = builder.header("Range", format!("bytes={start}-{end}"))
        }

        let mut response = builder.send().await?;
        match response.status().as_http_status() {
            StatusCode::OK | StatusCode::PARTIAL_CONTENT => {
                response.copy_to(writer).await?;
                Ok(())
            }
            StatusCode::SERVICE_UNAVAILABLE => Err(Error::TranscodeIncomplete),
            _ => Err(crate::Error::from_response(response).await),
        }
    }

    /// Deletes this item from the download queue.
    pub async fn delete(self) -> Result<()> {
        self.client
            .delete(
                DOWNLOAD_QUEUE_ITEM
                    .replace("{queueId}", &self.state.queue_id.to_string())
                    .replace("{itemId}", &self.state.id.to_string()),
            )
            .send()
            .await?;

        Ok(())
    }
}
