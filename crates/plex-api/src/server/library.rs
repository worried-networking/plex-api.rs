use std::{future::Future, marker::PhantomData, ops::RangeBounds};

use enum_dispatch::enum_dispatch;
use futures::AsyncWrite;
use http::StatusCode;
use http_adapter::Body;

use crate::{
    media_container::{
        server::library::{
            CollectionMetadataSubtype, LibraryType, Media as MediaMetadata, Metadata,
            MetadataMediaContainer, MetadataType, Part as PartMetadata, PlaylistMetadataType,
            Protocol, SearchType, ServerLibrary,
        },
        MediaContainerWrapper,
    },
    transcode::{MusicTranscodeOptions, TranscodeSession, VideoTranscodeOptions},
    Error, HttpClient, Result,
};

use super::transcode::{create_transcode_session, Context, TranscodeOptions};

pub trait FromMetadata {
    /// Creates an item given the http configuration and item metadata. No
    /// validation is performed that the metadata is correct.
    fn from_metadata(client: HttpClient, metadata: Metadata) -> Self;
}

/// Implements MetadataItem for the given struct which must only contain `client`
/// and `metadata` fields.
macro_rules! derive_from_metadata {
    ($typ:ident) => {
        impl FromMetadata for $typ {
            fn from_metadata(client: HttpClient, metadata: Metadata) -> Self {
                Self { client, metadata }
            }
        }
    };
}

/// Functionality shared across different items types in the Plex library.
#[enum_dispatch]
pub trait MetadataItem {
    /// Returns the Plex metadata for this item.
    fn metadata(&self) -> &Metadata;
    /// Returns the http client for this item.
    fn client(&self) -> &HttpClient;

    /// Returns the rating key for this item.
    ///
    /// This can be used to re-retrieve the item at a later time through the
    /// Server::item_by_id function.
    fn rating_key(&self) -> &str {
        self.metadata().rating_key.as_str()
    }

    /// Returns the title of this item.
    fn title(&self) -> &str {
        &self.metadata().title
    }
}

/// Implements MetadataItem for the given struct which must contain `client`
/// and `metadata` fields.
macro_rules! derive_metadata_item {
    ($typ:ident) => {
        impl MetadataItem for $typ {
            fn metadata(&self) -> &Metadata {
                &self.metadata
            }

            fn client(&self) -> &HttpClient {
                &self.client
            }
        }
    };
    ($typ:ident<$gen:ident>) => {
        impl<$gen> MetadataItem for $typ<$gen> {
            fn metadata(&self) -> &Metadata {
                &self.metadata
            }

            fn client(&self) -> &HttpClient {
                &self.client
            }
        }
    };
}

/// Retrieves a list of metadata items given the lookup key.
#[tracing::instrument(level = "trace", skip(client))]
pub(crate) async fn metadata_items<T>(client: &HttpClient, path: &str) -> Result<Vec<T>>
where
    T: FromMetadata,
{
    let wrapper: MediaContainerWrapper<MetadataMediaContainer> = client.get(path).json().await?;

    let media = wrapper
        .media_container
        .metadata
        .into_iter()
        .map(|metadata| {
            T::from_metadata(
                client.clone(),
                Metadata {
                    library_section_id: metadata
                        .library_section_id
                        .or(wrapper.media_container.library_section_id),
                    library_section_title: metadata
                        .library_section_title
                        .or(wrapper.media_container.library_section_title.clone()),
                    ..metadata
                },
            )
        })
        .collect();
    Ok(media)
}

/// Attempts to retrieve the parent of this item.
#[tracing::instrument(level = "trace", skip_all, fields(item.rating_key = item.rating_key()))]
async fn parent<T, P>(item: &T, client: &HttpClient) -> Result<Option<P>>
where
    T: MetadataItem,
    P: FromMetadata,
{
    if let Some(ref parent_key) = item.metadata().parent.parent_key {
        Ok(metadata_items(client, parent_key).await?.into_iter().next())
    } else {
        Ok(None)
    }
}

/// Retrieves the metadata items from a pivot from a library.
#[tracing::instrument(level = "trace", skip(client, directory), fields(directory.key = directory.key))]
async fn pivot_items<M>(
    client: &HttpClient,
    directory: &ServerLibrary,
    context: &str,
) -> Result<Vec<M>>
where
    M: FromMetadata,
{
    if let Some(pivot) = directory.pivots.iter().find(|p| p.context == context) {
        metadata_items(client, &pivot.key).await
    } else {
        Ok(Vec::new())
    }
}

/// A single media format for a `MediaItem`.
#[derive(Debug, Clone)]
pub struct Media<'a, M: MediaItem> {
    _options: PhantomData<M>,
    client: &'a HttpClient,
    media_index: usize,
    media: &'a MediaMetadata,
    parent_metadata: &'a Metadata,
}

impl<'a, M: MediaItem> Media<'a, M> {
    /// The different parts that make up this media. They should be played in
    /// order.
    pub fn parts(&'_ self) -> Vec<Part<'_, M>> {
        self.media
            .parts
            .iter()
            .enumerate()
            .map(|(index, part)| Part {
                _options: self._options,
                client: self.client,
                media_index: self.media_index,
                part_index: index,
                parent_metadata: self.parent_metadata,
                part,
            })
            .collect()
    }

    pub fn duration(&self) -> Option<u64> {
        self.media.duration
    }

    /// The internal metadata for the media.
    pub fn metadata(&self) -> &MediaMetadata {
        self.media
    }
}

impl<'a, M: MediaItemWithTranscoding + MediaItem + Sync> MediaItemWithTranscoding for Media<'a, M> {
    type Options = M::Options;

    #[tracing::instrument(level = "debug", skip_all)]
    async fn create_download_session(&self, options: M::Options) -> Result<TranscodeSession> {
        create_transcode_session(
            self.client,
            self.parent_metadata,
            Context::Static,
            Protocol::Http,
            Some(self.media_index),
            None,
            options,
        )
        .await
    }

    #[tracing::instrument(level = "debug", skip_all)]
    async fn create_streaming_session(
        &self,
        protocol: Protocol,
        options: M::Options,
    ) -> Result<TranscodeSession> {
        create_transcode_session(
            self.client,
            self.parent_metadata,
            Context::Streaming,
            protocol,
            Some(self.media_index),
            None,
            options,
        )
        .await
    }
}

/// One part of a `Media`.
#[derive(Debug, Clone)]
pub struct Part<'a, M: MediaItem> {
    _options: PhantomData<M>,
    pub(crate) client: &'a HttpClient,
    pub media_index: usize,
    pub part_index: usize,
    part: &'a PartMetadata,
    parent_metadata: &'a Metadata,
}

impl<'a, M: MediaItem> Part<'a, M> {
    /// The length of this file on disk in bytes.
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> Option<u64> {
        self.part.size
    }

    pub fn duration(&self) -> Option<u64> {
        self.part.duration
    }

    /// Downloads the original media file for this part writing the data into
    /// the provided writer. A range of bytes within the file can be requested
    /// allowing for resumable transfers.
    ///
    /// Configured timeout value will be ignored during downloading.
    #[tracing::instrument(level = "debug", skip_all)]
    pub async fn download<W, R>(&self, writer: W, range: R) -> Result
    where
        W: AsyncWrite + Unpin,
        R: RangeBounds<u64>,
    {
        let path = format!("{}?download=1", self.part.key.as_ref().unwrap());

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
        if start != 0 || (end.is_some() && end != self.part.size) {
            // We're requesting part of the file.
            let end = end.map(|v| v.to_string()).unwrap_or_default();
            builder = builder.header("Range", format!("bytes={start}-{end}"))
        }

        let response = builder.send().await?;
        match response.status() {
            StatusCode::OK | StatusCode::PARTIAL_CONTENT => {
                let body = response.into_body();
                body.copy_to(writer).await?;
                Ok(())
            }
            _ => Err(crate::Error::from_response(response).await),
        }
    }

    /// The internal metadata for the media.
    pub fn metadata(&self) -> &PartMetadata {
        self.part
    }
}

impl<'a, M: MediaItemWithTranscoding + MediaItem + Sync> MediaItemWithTranscoding for Part<'a, M> {
    type Options = M::Options;

    #[tracing::instrument(level = "debug", skip_all)]
    async fn create_download_session(&self, options: M::Options) -> Result<TranscodeSession> {
        create_transcode_session(
            self.client,
            self.parent_metadata,
            Context::Static,
            Protocol::Http,
            Some(self.media_index),
            Some(self.part_index),
            options,
        )
        .await
    }

    #[tracing::instrument(level = "debug", skip_all)]
    async fn create_streaming_session(
        &self,
        protocol: Protocol,
        options: M::Options,
    ) -> Result<TranscodeSession> {
        create_transcode_session(
            self.client,
            self.parent_metadata,
            Context::Streaming,
            protocol,
            Some(self.media_index),
            Some(self.part_index),
            options,
        )
        .await
    }
}

/// Represents some playable media. In Plex each playable item can be available
/// in a number of different formats which in turn can be made up of a number of
/// different parts.
pub trait MediaItem: MetadataItem + Sized {
    /// The different media formats that this item is available in.
    fn media(&'_ self) -> Vec<Media<'_, Self>> {
        let metadata = self.metadata();
        if let Some(ref media) = metadata.media {
            media
                .iter()
                .enumerate()
                .map(|(index, media)| Media {
                    _options: PhantomData,
                    client: self.client(),
                    media_index: index,
                    parent_metadata: metadata,
                    media,
                })
                .collect()
        } else {
            Vec::new()
        }
    }
}

pub trait MediaItemWithTranscoding {
    type Options: TranscodeOptions + Send;

    /// Starts an offline transcode using the provided options.
    ///
    /// The server may refuse to transcode if the options suggest that the
    /// original media file can be played back directly.
    ///
    /// Can't be called on media other than Movie, Episode or Track.
    fn create_download_session(
        &self,
        options: Self::Options,
    ) -> impl Future<Output = Result<TranscodeSession>> + Send;

    /// Starts a streaming transcode using of the given media part using the
    /// streaming protocol and provided options.
    ///
    /// Can't be called on media other than Movie, Episode or Track.
    fn create_streaming_session(
        &self,
        protocol: Protocol,
        options: Self::Options,
    ) -> impl Future<Output = Result<TranscodeSession>> + Send;
}

/// A video that can be included in a video playlist.
#[enum_dispatch(MetadataItem)]
#[derive(Debug, Clone)]
pub enum Video {
    Movie,
    Episode,
}

impl FromMetadata for Video {
    fn from_metadata(client: HttpClient, metadata: Metadata) -> Self {
        if let Some(MetadataType::Episode) = metadata.metadata_type {
            Episode::from_metadata(client, metadata).into()
        } else {
            Movie::from_metadata(client, metadata).into()
        }
    }
}

impl MediaItem for Video {}
impl MediaItemWithTranscoding for Video {
    type Options = VideoTranscodeOptions;

    #[tracing::instrument(level = "debug", skip_all)]
    async fn create_download_session(&self, options: Self::Options) -> Result<TranscodeSession> {
        create_transcode_session(
            self.client(),
            self.metadata(),
            Context::Static,
            Protocol::Http,
            None,
            None,
            options,
        )
        .await
    }

    #[tracing::instrument(level = "debug", skip_all)]
    async fn create_streaming_session(
        &self,
        protocol: Protocol,
        options: Self::Options,
    ) -> Result<TranscodeSession> {
        create_transcode_session(
            self.client(),
            self.metadata(),
            Context::Streaming,
            protocol,
            None,
            None,
            options,
        )
        .await
    }
}

#[derive(Debug, Clone)]
pub struct Playlist<M> {
    _items: PhantomData<M>,
    client: HttpClient,
    metadata: Metadata,
}

impl<M> FromMetadata for Playlist<M> {
    fn from_metadata(client: HttpClient, metadata: Metadata) -> Self {
        Self {
            _items: PhantomData,
            client,
            metadata,
        }
    }
}

derive_metadata_item!(Playlist<M>);

impl<M> Playlist<M>
where
    M: FromMetadata,
{
    #[tracing::instrument(level = "debug", skip_all)]
    pub async fn children(&self) -> Result<Vec<M>> {
        metadata_items(&self.client, &self.metadata.key).await
    }
}

#[derive(Debug, Clone)]
pub struct Collection<M> {
    _items: PhantomData<M>,
    client: HttpClient,
    metadata: Metadata,
}

impl<M> FromMetadata for Collection<M> {
    fn from_metadata(client: HttpClient, metadata: Metadata) -> Self {
        Self {
            _items: PhantomData,
            client,
            metadata,
        }
    }
}

derive_metadata_item!(Collection<M>);

impl<M> Collection<M>
where
    M: FromMetadata,
{
    #[tracing::instrument(level = "debug", skip_all)]
    pub async fn children(&self) -> Result<Vec<M>> {
        metadata_items(&self.client, &self.metadata.key).await
    }
}

#[derive(Debug, Clone)]
pub struct Movie {
    client: HttpClient,
    metadata: Metadata,
}

derive_from_metadata!(Movie);
derive_metadata_item!(Movie);

impl MediaItem for Movie {}
impl MediaItemWithTranscoding for Movie {
    type Options = VideoTranscodeOptions;

    #[tracing::instrument(level = "debug", skip_all)]
    async fn create_download_session(&self, options: Self::Options) -> Result<TranscodeSession> {
        create_transcode_session(
            self.client(),
            self.metadata(),
            Context::Static,
            Protocol::Http,
            None,
            None,
            options,
        )
        .await
    }

    #[tracing::instrument(level = "debug", skip_all)]
    async fn create_streaming_session(
        &self,
        protocol: Protocol,
        options: Self::Options,
    ) -> Result<TranscodeSession> {
        create_transcode_session(
            self.client(),
            self.metadata(),
            Context::Streaming,
            protocol,
            None,
            None,
            options,
        )
        .await
    }
}

#[derive(Debug, Clone)]
pub struct Show {
    client: HttpClient,
    metadata: Metadata,
}

derive_from_metadata!(Show);
derive_metadata_item!(Show);

impl Show {
    /// Retrieves all of the seasons of this show.
    #[tracing::instrument(level = "debug", skip_all, fields(self.metadata.key = self.metadata.key))]
    pub async fn seasons(&self) -> Result<Vec<Season>> {
        metadata_items(&self.client, &self.metadata.key).await
    }

    /// Retrieves all of the episodes in all seasons of this show.
    #[tracing::instrument(level = "debug", skip_all, fields(self.metadata.rating_key = self.metadata.rating_key))]
    pub async fn episodes(&self) -> Result<Vec<Episode>> {
        let path = format!("/library/metadata/{}/allLeaves", self.metadata.rating_key);
        metadata_items(&self.client, &path).await
    }
}

#[derive(Debug, Clone)]
pub struct Season {
    client: HttpClient,
    metadata: Metadata,
}

derive_from_metadata!(Season);
derive_metadata_item!(Season);

impl Season {
    pub fn season_number(&self) -> Option<u32> {
        self.metadata.index
    }

    /// Retrieves all of the episodes in this season.
    #[tracing::instrument(level = "debug", skip_all, fields(self.metadata.key = self.metadata.key))]
    pub async fn episodes(&self) -> Result<Vec<Episode>> {
        metadata_items(&self.client, &self.metadata.key).await
    }

    /// Retrieves the show that this season is from.
    #[tracing::instrument(level = "debug", skip_all, fields(self.metadata.key = self.metadata.key))]
    pub async fn show(&self) -> Result<Option<Show>> {
        parent(self, &self.client).await
    }
}

#[derive(Debug, Clone)]
pub struct Episode {
    client: HttpClient,
    metadata: Metadata,
}

derive_from_metadata!(Episode);
derive_metadata_item!(Episode);

impl MediaItem for Episode {}
impl MediaItemWithTranscoding for Episode {
    type Options = VideoTranscodeOptions;

    #[tracing::instrument(level = "debug", skip_all)]
    async fn create_download_session(&self, options: Self::Options) -> Result<TranscodeSession> {
        create_transcode_session(
            self.client(),
            self.metadata(),
            Context::Static,
            Protocol::Http,
            None,
            None,
            options,
        )
        .await
    }

    #[tracing::instrument(level = "debug", skip_all)]
    async fn create_streaming_session(
        &self,
        protocol: Protocol,
        options: Self::Options,
    ) -> Result<TranscodeSession> {
        create_transcode_session(
            self.client(),
            self.metadata(),
            Context::Streaming,
            protocol,
            None,
            None,
            options,
        )
        .await
    }
}

impl Episode {
    /// Returns the number of this season within the show.
    pub fn season_number(&self) -> Option<u32> {
        self.metadata.parent.parent_index
    }

    /// Returns the number of this episode within the season.
    pub fn episode_number(&self) -> Option<u32> {
        self.metadata.index
    }

    /// Retrieves the season that this episode is from.
    #[tracing::instrument(level = "debug", skip_all, fields(self.metadata.key = self.metadata.key))]
    pub async fn season(&self) -> Result<Option<Season>> {
        parent(self, &self.client).await
    }
}

#[derive(Debug, Clone)]
pub struct Artist {
    client: HttpClient,
    metadata: Metadata,
}

derive_from_metadata!(Artist);
derive_metadata_item!(Artist);

impl Artist {
    /// Retrieves all of the fully-featured studio albums (skipping Lives, EPs, etc.) by this artist.
    #[tracing::instrument(level = "debug", skip_all, fields(self.metadata.key = self.metadata.key))]
    pub async fn full_studio_albums(&self) -> Result<Vec<MusicAlbum>> {
        metadata_items(&self.client, &self.metadata.key).await
    }

    /// Retrieves all of the albums by this artist.
    #[tracing::instrument(level = "debug", skip_all, fields(self.metadata.key = self.metadata.key))]
    pub async fn albums(&self) -> Result<Vec<MusicAlbum>> {
        let section_id = match &self.metadata.library_section_id {
            Some(id) => id,
            None => return Err(Error::UnexpectedError),
        };

        let albums_search_path = format!(
            "/library/sections/{}/all?type={}&artist.id={}",
            section_id,
            SearchType::Album,
            self.metadata.rating_key
        );
        metadata_items(&self.client, &albums_search_path).await
    }
}

#[derive(Debug, Clone)]
pub struct MusicAlbum {
    client: HttpClient,
    metadata: Metadata,
}

derive_from_metadata!(MusicAlbum);
derive_metadata_item!(MusicAlbum);

impl MusicAlbum {
    /// Retrieves all of the tracks in this album.
    #[tracing::instrument(level = "debug", skip_all, fields(self.metadata.key = self.metadata.key))]
    pub async fn tracks(&self) -> Result<Vec<Track>> {
        metadata_items(&self.client, &self.metadata.key).await
    }

    /// Retrieves the artist for this album.
    #[tracing::instrument(level = "debug", skip_all, fields(self.metadata.key = self.metadata.key))]
    pub async fn artist(&self) -> Result<Option<Artist>> {
        parent(self, &self.client).await
    }
}

#[derive(Debug, Clone)]
pub struct Track {
    client: HttpClient,
    metadata: Metadata,
}

derive_from_metadata!(Track);
derive_metadata_item!(Track);

impl MediaItem for Track {}
impl MediaItemWithTranscoding for Track {
    type Options = MusicTranscodeOptions;

    #[tracing::instrument(level = "debug", skip_all)]
    async fn create_download_session(&self, options: Self::Options) -> Result<TranscodeSession> {
        create_transcode_session(
            self.client(),
            self.metadata(),
            Context::Static,
            Protocol::Http,
            None,
            None,
            options,
        )
        .await
    }

    #[tracing::instrument(level = "debug", skip_all)]
    async fn create_streaming_session(
        &self,
        protocol: Protocol,
        options: Self::Options,
    ) -> Result<TranscodeSession> {
        create_transcode_session(
            self.client(),
            self.metadata(),
            Context::Streaming,
            protocol,
            None,
            None,
            options,
        )
        .await
    }
}

impl Track {
    /// Returns the number of this track within the album.
    pub fn track_number(&self) -> Option<u32> {
        self.metadata.index
    }

    /// Retrieves the album for this track.
    #[tracing::instrument(level = "debug", skip_all, fields(self.metadata.key = self.metadata.key))]
    pub async fn album(&self) -> Result<Option<MusicAlbum>> {
        parent(self, &self.client).await
    }
}

#[derive(Debug, Clone)]
pub struct Photo {
    client: HttpClient,
    metadata: Metadata,
}

derive_from_metadata!(Photo);
derive_metadata_item!(Photo);

impl MediaItem for Photo {}

impl Photo {
    /// Retrieves the album that this photo is in.
    #[tracing::instrument(level = "debug", skip_all, fields(self.metadata.key = self.metadata.key))]
    pub async fn album(&self) -> Result<Option<PhotoAlbum>> {
        parent(self, &self.client).await
    }
}

#[derive(Debug, Clone)]
pub struct PhotoAlbum {
    client: HttpClient,
    metadata: Metadata,
}

derive_from_metadata!(PhotoAlbum);
derive_metadata_item!(PhotoAlbum);

impl PhotoAlbum {
    /// Retrieves all of the albums and photos in this album.
    #[tracing::instrument(level = "debug", skip_all, fields(self.metadata.key = self.metadata.key))]
    pub async fn contents(&self) -> Result<Vec<PhotoAlbumItem>> {
        metadata_items(&self.client, &self.metadata.key).await
    }

    /// Retrieves the album that this album is in.
    #[tracing::instrument(level = "debug", skip_all, fields(self.metadata.key = self.metadata.key))]
    pub async fn album(&self) -> Result<Option<PhotoAlbum>> {
        parent(self, &self.client).await
    }
}

#[enum_dispatch(MetadataItem)]
pub enum PhotoAlbumItem {
    PhotoAlbum,
    Photo,
}

impl FromMetadata for PhotoAlbumItem {
    fn from_metadata(client: HttpClient, metadata: Metadata) -> Self {
        // This isn't a great test but there doesn't seem to be much better.
        if metadata.key.ends_with("/children") {
            PhotoAlbum::from_metadata(client, metadata).into()
        } else {
            Photo::from_metadata(client, metadata).into()
        }
    }
}

#[derive(Debug, Clone)]
pub struct Clip {
    client: HttpClient,
    metadata: Metadata,
}

derive_from_metadata!(Clip);
derive_metadata_item!(Clip);

impl MediaItem for Clip {}

#[derive(Debug, Clone)]
pub struct UnknownItem {
    client: HttpClient,
    metadata: Metadata,
}

derive_from_metadata!(UnknownItem);
derive_metadata_item!(UnknownItem);

#[enum_dispatch(MetadataItem)]
#[derive(Debug, Clone)]
pub enum Item {
    Movie,
    Episode,
    Photo,
    Show,
    Artist,
    MusicAlbum,
    Season,
    Track,
    Clip,
    MovieCollection(Collection<Movie>),
    ShowCollection(Collection<Show>),
    VideoPlaylist(Playlist<Video>),
    PhotoPlaylist(Playlist<Photo>),
    MusicPlaylist(Playlist<Track>),
    UnknownItem,
}

impl MediaItem for Item {}

impl FromMetadata for Item {
    fn from_metadata(client: HttpClient, metadata: Metadata) -> Self {
        if let Some(ref item_type) = metadata.metadata_type {
            match item_type {
                MetadataType::Movie => Movie::from_metadata(client, metadata).into(),
                MetadataType::Episode => Episode::from_metadata(client, metadata).into(),
                MetadataType::Photo => Photo::from_metadata(client, metadata).into(),
                MetadataType::Show => Show::from_metadata(client, metadata).into(),
                MetadataType::Artist => Artist::from_metadata(client, metadata).into(),
                MetadataType::MusicAlbum => MusicAlbum::from_metadata(client, metadata).into(),
                MetadataType::Season => Season::from_metadata(client, metadata).into(),
                MetadataType::Track => Track::from_metadata(client, metadata).into(),
                MetadataType::Clip(_) => Clip::from_metadata(client, metadata).into(),
                MetadataType::Collection(CollectionMetadataSubtype::Movie) => {
                    Collection::<Movie>::from_metadata(client, metadata).into()
                }
                MetadataType::Collection(CollectionMetadataSubtype::Show) => {
                    Collection::<Show>::from_metadata(client, metadata).into()
                }
                MetadataType::Playlist(PlaylistMetadataType::Video) => {
                    Playlist::<Video>::from_metadata(client, metadata).into()
                }
                MetadataType::Playlist(PlaylistMetadataType::Audio) => {
                    Playlist::<Track>::from_metadata(client, metadata).into()
                }
                MetadataType::Playlist(PlaylistMetadataType::Photo) => {
                    Playlist::<Photo>::from_metadata(client, metadata).into()
                }
                #[cfg(not(feature = "tests_deny_unknown_fields"))]
                _ => UnknownItem::from_metadata(client, metadata).into(),
            }
        } else {
            UnknownItem::from_metadata(client, metadata).into()
        }
    }
}

#[derive(Debug, Clone)]
pub struct MovieLibrary {
    client: HttpClient,
    directory: ServerLibrary,
}

impl MovieLibrary {
    /// Returns the title of this library.
    pub fn title(&self) -> &str {
        &self.directory.title
    }

    /// Retrieves all of the movies in this library.
    #[tracing::instrument(level = "debug", skip_all)]
    pub async fn movies(&self) -> Result<Vec<Movie>> {
        pivot_items(&self.client, &self.directory, "content.library").await
    }

    /// Retrieves all of the collections in this library.
    #[tracing::instrument(level = "debug", skip_all)]
    pub async fn collections(&self) -> Result<Vec<Collection<Movie>>> {
        pivot_items(&self.client, &self.directory, "content.collections").await
    }

    /// Retrieves all of the playlists containing movies from this library.
    #[tracing::instrument(level = "debug", skip_all)]
    pub async fn playlists(&self) -> Result<Vec<Playlist<Video>>> {
        pivot_items(&self.client, &self.directory, "content.playlists").await
    }
}

#[derive(Debug, Clone)]
pub struct TVLibrary {
    client: HttpClient,
    directory: ServerLibrary,
}

impl TVLibrary {
    /// Returns the title of this library.
    pub fn title(&self) -> &str {
        &self.directory.title
    }

    /// Retrieves all of the shows in this library.
    #[tracing::instrument(level = "debug", skip_all)]
    pub async fn shows(&self) -> Result<Vec<Show>> {
        pivot_items(&self.client, &self.directory, "content.library").await
    }

    /// Retrieves all of the collections in this library.
    #[tracing::instrument(level = "debug", skip_all)]
    pub async fn collections(&self) -> Result<Vec<Collection<Show>>> {
        pivot_items(&self.client, &self.directory, "content.collections").await
    }

    /// Retrieves all of the playlists containing episodes from this library.
    #[tracing::instrument(level = "debug", skip_all)]
    pub async fn playlists(&self) -> Result<Vec<Playlist<Video>>> {
        pivot_items(&self.client, &self.directory, "content.playlists").await
    }
}

#[derive(Debug, Clone)]
pub struct MusicLibrary {
    client: HttpClient,
    directory: ServerLibrary,
}

impl MusicLibrary {
    /// Returns the title of this library.
    pub fn title(&self) -> &str {
        &self.directory.title
    }

    /// Retrieves all of the artists in this library.
    #[tracing::instrument(level = "debug", skip_all)]
    pub async fn artists(&self) -> Result<Vec<Artist>> {
        pivot_items(&self.client, &self.directory, "content.library").await
    }

    /// Retrieves all of the playlists containing tracks from this library.
    #[tracing::instrument(level = "debug", skip_all)]
    pub async fn playlists(&self) -> Result<Vec<Playlist<Track>>> {
        pivot_items(&self.client, &self.directory, "content.playlists").await
    }
}

#[derive(Debug, Clone)]
pub struct PhotoLibrary {
    client: HttpClient,
    directory: ServerLibrary,
}

impl PhotoLibrary {
    /// Returns the title of this library.
    pub fn title(&self) -> &str {
        &self.directory.title
    }

    /// Retrieves all of the albums in this library.
    #[tracing::instrument(level = "debug", skip_all)]
    pub async fn albums(&self) -> Result<Vec<PhotoAlbum>> {
        pivot_items(&self.client, &self.directory, "content.library").await
    }

    /// Retrieves all of the playlists containing photos from this library.
    #[tracing::instrument(level = "debug", skip_all)]
    pub async fn playlists(&self) -> Result<Vec<Playlist<Photo>>> {
        pivot_items(&self.client, &self.directory, "content.playlists").await
    }
}

#[derive(Debug, Clone)]
pub enum Library {
    Movie(MovieLibrary),
    TV(TVLibrary),
    Music(MusicLibrary),
    Video(MovieLibrary),
    Photo(PhotoLibrary),
}

impl Library {
    pub(super) fn new(client: HttpClient, directory: ServerLibrary) -> Self {
        match directory.library_type {
            LibraryType::Movie => {
                if directory.subtype.as_deref() == Some("clip") {
                    Library::Video(MovieLibrary { client, directory })
                } else {
                    Library::Movie(MovieLibrary { client, directory })
                }
            }
            LibraryType::Show => Library::TV(TVLibrary { client, directory }),
            LibraryType::Artist => Library::Music(MusicLibrary { client, directory }),
            LibraryType::Photo => Library::Photo(PhotoLibrary { client, directory }),
            LibraryType::Mixed => todo!("Mixed library type is not supported yet"),
            LibraryType::Clip => todo!("Clip library type is not supported yet"),
            #[cfg(not(feature = "tests_deny_unknown_fields"))]
            LibraryType::Unknown => panic!("Unknown library type"),
        }
    }

    fn directory(&self) -> &ServerLibrary {
        match self {
            Self::Movie(l) => &l.directory,
            Self::TV(l) => &l.directory,
            Self::Music(l) => &l.directory,
            Self::Video(l) => &l.directory,
            Self::Photo(l) => &l.directory,
        }
    }

    /// Returns the unique ID of this library.
    pub fn id(&self) -> &str {
        &self.directory().id
    }

    /// Returns the title of this library.
    pub fn title(&self) -> &str {
        &self.directory().title
    }

    pub fn library_type(&self) -> &LibraryType {
        &self.directory().library_type
    }
}
