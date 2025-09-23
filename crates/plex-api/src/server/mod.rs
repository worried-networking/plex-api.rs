pub mod library;
pub(crate) mod prefs;
pub mod transcode;

use self::{
    library::{metadata_items, FromMetadata, Item, Library, MediaItem, MetadataItem},
    prefs::Preferences,
    transcode::{
        session::{transcode_session_stats, TranscodeSession, TranscodeSessionsMediaContainer},
        transcode_artwork, ArtTranscodeOptions,
    },
};
#[cfg(not(feature = "tests_deny_unknown_fields"))]
use crate::media_container::server::library::LibraryType;
use crate::{
    http_client::HttpClient,
    isahc_compat::StatusCodeExt,
    media_container::{
        server::{library::ContentDirectory, MediaProviderFeature, Server as ServerMediaContainer},
        MediaContainerWrapper,
    },
    myplex::MyPlex,
    transcode::download_queue::DownloadQueue,
    url::{
        SERVER_MEDIA_PROVIDERS, SERVER_MYPLEX_ACCOUNT, SERVER_MYPLEX_CLAIM, SERVER_SCROBBLE,
        SERVER_TIMELINE, SERVER_TRANSCODE_SESSIONS, SERVER_UNSCROBBLE,
    },
    Error, HttpClientBuilder, Result,
};
use futures::AsyncWrite;
use http::{StatusCode, Uri};
use isahc::AsyncReadResponseExt;
use std::{
    collections::HashMap,
    fmt::{self, Debug},
};

struct Query {
    params: HashMap<String, String>,
}

impl Query {
    fn new() -> Self {
        Self {
            params: HashMap::new(),
        }
    }

    fn append(mut self, other: HashMap<String, String>) -> Self {
        self.params.extend(other);
        self
    }

    fn param<N: Into<String>, V: Into<String>>(mut self, name: N, value: V) -> Self {
        self.params.insert(name.into(), value.into());
        self
    }
}

impl From<Query> for HashMap<String, String> {
    fn from(query: Query) -> Self {
        query.params
    }
}

impl fmt::Display for Query {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad(&serde_urlencoded::to_string(&self.params).unwrap())
    }
}

#[derive(Debug, Clone)]
pub struct Server {
    client: HttpClient,
    pub myplex_api_url: Uri,
    pub media_container: ServerMediaContainer,
}

impl Server {
    async fn build(client: HttpClient, myplex_api_url: Uri) -> Result<Self> {
        let media_container_wrapper: MediaContainerWrapper<ServerMediaContainer> =
            client.get(SERVER_MEDIA_PROVIDERS).json().await?;

        Ok(Self {
            media_container: media_container_wrapper.media_container,
            client,
            myplex_api_url,
        })
    }

    #[tracing::instrument(level = "debug", skip(client))]
    pub async fn new<U>(url: U, client: HttpClient) -> Result<Self>
    where
        U: Debug,
        Uri: TryFrom<U>,
        <Uri as TryFrom<U>>::Error: Into<http::Error>,
    {
        let myplex_api_url = client.api_url.clone();
        Self::build(
            HttpClientBuilder::from(client).set_api_url(url).build()?,
            myplex_api_url,
        )
        .await
    }

    fn content(&self) -> Option<&Vec<ContentDirectory>> {
        if let Some(provider) = self
            .media_container
            .media_providers
            .iter()
            .find(|p| p.identifier == "com.plexapp.plugins.library")
        {
            for feature in &provider.features {
                if let MediaProviderFeature::Content {
                    key: _,
                    ref directory,
                } = feature
                {
                    return Some(directory);
                }
            }
        }

        None
    }

    pub fn libraries(&self) -> Vec<Library> {
        if let Some(content) = self.content() {
            content
                .iter()
                .filter_map(|d| match d {
                    ContentDirectory::Media(lib) => match lib.library_type {
                        #[cfg(not(feature = "tests_deny_unknown_fields"))]
                        LibraryType::Unknown => None,
                        _ => Some(Library::new(self.client.clone(), *lib.clone())),
                    },
                    _ => None,
                })
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Given the path to some item's artwork (`art` or `thumb` properties for
    /// example but many other types of images will work) this will request a
    /// scaled version of that image be written to the passed writer as a JPEG.
    /// The image will always maintain its aspect ratio.
    #[tracing::instrument(
        name = "Server::transcode_artwork",
        level = "debug",
        skip(self, writer)
    )]
    pub async fn transcode_artwork<W>(
        &self,
        art: &str,
        width: u32,
        height: u32,
        options: ArtTranscodeOptions,
        writer: W,
    ) -> Result<()>
    where
        W: AsyncWrite + Unpin,
    {
        transcode_artwork(&self.client, art, width, height, options, writer).await
    }

    /// Retrieves a list of the current transcode sessions.
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn transcode_sessions(&self) -> Result<Vec<TranscodeSession>> {
        let wrapper: MediaContainerWrapper<TranscodeSessionsMediaContainer> =
            self.client.get(SERVER_TRANSCODE_SESSIONS).json().await?;

        Ok(wrapper
            .media_container
            .transcode_sessions
            .into_iter()
            .map(move |stats| TranscodeSession::from_stats(self.client.clone(), stats))
            .collect())
    }

    /// Retrieves the transcode session with the passed ID.
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn transcode_session(&self, session_id: &str) -> Result<TranscodeSession> {
        let stats = transcode_session_stats(&self.client, session_id).await?;
        Ok(TranscodeSession::from_stats(self.client.clone(), stats))
    }

    /// Allows retrieving media, playlists, collections and other items using
    /// their rating key.
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn item_by_id(&self, rating_key: &str) -> Result<Item> {
        let path = format!("/library/metadata/{rating_key}?includeConcerts=1&includeExtras=1&includePopularLeaves=1&includePreferences=1&includeReviews=1&includeOnDeck=1&includeChapters=1&includeStations=1&includeExternalMedia=1&asyncAugmentMetadata=1&asyncCheckFiles=1&asyncRefreshAnalysis=1&asyncRefreshLocalMediaAgent=1&includeMarkers=1");

        match metadata_items(&self.client, &path).await {
            Ok(items) => items.into_iter().next().ok_or(Error::ItemNotFound),
            Err(Error::UnexpectedApiResponse {
                status_code,
                content,
            }) => {
                // A 404 error indicates the item does not exist.
                if status_code == 404 {
                    Err(Error::ItemNotFound)
                } else {
                    Err(Error::UnexpectedApiResponse {
                        status_code,
                        content,
                    })
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Marks a media item as fully watched increasing its view count by one.
    pub async fn mark_watched<M: MediaItem + FromMetadata>(&self, item: &M) -> Result<M> {
        let rating_key = item.rating_key();
        let path =
            format!("{SERVER_SCROBBLE}?identifier=com.plexapp.plugins.library&key={rating_key}");

        self.client.get(path).consume().await?;

        let item = self.item_by_id(rating_key).await?;
        Ok(M::from_metadata(
            self.client.clone(),
            item.metadata().clone(),
        ))
    }

    /// Marks a media item as unwatched.
    pub async fn mark_unwatched<M: MediaItem + FromMetadata>(&self, item: &M) -> Result<M> {
        let rating_key = item.rating_key();
        let path =
            format!("{SERVER_UNSCROBBLE}?identifier=com.plexapp.plugins.library&key={rating_key}");

        self.client.get(path).consume().await?;

        let item = self.item_by_id(rating_key).await?;
        Ok(M::from_metadata(
            self.client.clone(),
            item.metadata().clone(),
        ))
    }

    /// Sets a media item's playback position in milliseconds. The server currently ignores any
    /// positions equal to or less than 60000ms. The time sets the time the item was last viewed.
    pub async fn update_timeline<M: MediaItem + FromMetadata>(
        &self,
        item: &M,
        position: u64,
    ) -> Result<M> {
        let rating_key = item.rating_key();
        let query = Query::new()
            .param("key", format!("/library/metadata/{rating_key}"))
            .param("ratingKey", rating_key)
            .param("offline", "1")
            .param("state", "playing")
            .param("time", position.to_string());

        self.client
            .get(format!("{SERVER_TIMELINE}?{query}"))
            .consume()
            .await?;

        let item = self.item_by_id(rating_key).await?;
        Ok(M::from_metadata(
            self.client.clone(),
            item.metadata().clone(),
        ))
    }

    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn refresh(self) -> Result<Self> {
        Self::build(self.client, self.myplex_api_url).await
    }

    pub fn myplex(&self) -> Result<MyPlex> {
        self.myplex_with_api_url(self.myplex_api_url.clone())
    }

    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn claim(self, claim_token: &str) -> Result<Self> {
        let url = format!(
            "{}?{}",
            SERVER_MYPLEX_CLAIM,
            serde_urlencoded::to_string([("token", claim_token)])?
        );
        let mut response = self.client.post(url).send().await?;

        if response.status().as_http_status() == StatusCode::OK {
            response.consume().await?;
            self.refresh().await
        } else {
            Err(crate::Error::from_response(response).await)
        }
    }

    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn unclaim(self) -> Result<Self> {
        let mut response = self.client.delete(SERVER_MYPLEX_ACCOUNT).send().await?;

        if response.status().as_http_status() == StatusCode::OK {
            response.consume().await?;
            self.refresh().await
        } else {
            Err(crate::Error::from_response(response).await)
        }
    }

    pub fn myplex_with_api_url<U>(&self, api_url: U) -> Result<MyPlex>
    where
        Uri: TryFrom<U>,
        <Uri as TryFrom<U>>::Error: Into<http::Error>,
    {
        Ok(MyPlex::new(
            HttpClientBuilder::from(self.client.clone())
                .set_api_url(api_url)
                .build()?,
        ))
    }

    pub fn client(&self) -> &HttpClient {
        &self.client
    }

    pub async fn preferences<'a>(&self) -> Result<Preferences<'a>> {
        Preferences::new(&self.client).await
    }

    pub fn machine_identifier(&self) -> &str {
        &self.media_container.machine_identifier
    }

    pub async fn download_queue(&self) -> Result<DownloadQueue> {
        DownloadQueue::get_or_create(self.client.clone()).await
    }
}
