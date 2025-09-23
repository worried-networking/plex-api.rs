mod fixtures;

mod offline {
    use std::collections::HashMap;

    use super::fixtures::offline::{server::*, Mocked};
    use httpmock::{prelude::HttpMockRequest, Method::GET};
    use plex_api::{
        library::{Movie, Transcodable},
        media_container::server::library::{AudioCodec, VideoCodec},
        Server,
    };

    // Expands a profile query parameter into the list of settings.
    fn expand_profile(req: &HttpMockRequest) -> HashMap<String, Vec<HashMap<String, String>>> {
        let param = req
            .query_params()
            .into_iter()
            .filter_map(|(n, v)| {
                if n == "X-Plex-Client-Profile-Extra" {
                    Some(v)
                } else {
                    None
                }
            })
            .next()
            .unwrap();

        let mut settings: HashMap<String, Vec<HashMap<String, String>>> = HashMap::new();
        for setting in param.split('+') {
            if setting.ends_with(')') {
                if let Some(idx) = setting.find('(') {
                    let setting_name = setting[0..idx].to_string();
                    let params: HashMap<String, String> = setting[idx + 1..setting.len() - 1]
                        // Split up the parameters
                        .split('&')
                        .filter_map(|v| {
                            // Split into name=value
                            v.find('=')
                                .map(|index| (v[0..index].to_string(), v[index + 1..].to_string()))
                        })
                        .collect();

                    if let Some(list) = settings.get_mut(&setting_name) {
                        list.push(params);
                    } else {
                        settings.insert(setting_name, vec![params]);
                    }
                }
            }
        }

        settings
    }

    fn assert_setting_count(
        settings: &HashMap<String, Vec<HashMap<String, String>>>,
        name: &str,
        expected: usize,
    ) {
        if let Some(s) = settings.get(name) {
            assert_eq!(s.len(), expected);
        } else {
            assert_eq!(0, expected);
        }
    }

    fn assert_setting(
        settings: &HashMap<String, Vec<HashMap<String, String>>>,
        name: &str,
        values: &[(&str, &str)],
    ) {
        let settings = if let Some(s) = settings.get(name) {
            s
        } else {
            panic!("Failed to find match for {values:#?} in []")
        };

        for setting in settings {
            if setting.len() != values.len() {
                continue;
            }

            let mut matched = true;
            for (name, value) in values {
                if setting.get(*name) != Some(&value.to_string()) {
                    matched = false;
                }
            }

            if matched {
                return;
            }
        }

        panic!("Failed to find match for {values:#?} in {settings:#?}")
    }

    #[plex_api_test_helper::offline_test]
    async fn download_queue(#[future] server_authenticated: Mocked<Server>) {
        let (server, mock_server) = server_authenticated.split();

        // Test getting/creating the queue
        let mut m = mock_server.mock(|when, then| {
            when.method(httpmock::Method::POST).path("/downloadQueue");
            then.status(200)
                .header("content-type", "text/json")
                .body_from_file("tests/mocks/download_queue/queue_created.json");
        });

        let queue = server.download_queue().await.unwrap();
        m.assert();
        m.delete();

        // Test listing items when queue is empty
        let mut m = mock_server.mock(|when, then| {
            when.method(GET).path("/downloadQueue/1/items");
            then.status(200)
                .header("content-type", "text/json")
                .body_from_file("tests/mocks/download_queue/empty_items.json");
        });

        let items = queue.items().await.unwrap();
        assert_eq!(items.len(), 0);
        m.assert();
        m.delete();

        // Test listing items when queue has one item
        let mut m = mock_server.mock(|when, then| {
            when.method(GET).path("/downloadQueue/1/items");
            then.status(200)
                .header("content-type", "text/json")
                .body_from_file("tests/mocks/download_queue/items_with_one.json");
        });

        let items = queue.items().await.unwrap();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].id(), 123);
        assert_eq!(items[0].key(), "/library/metadata/159637");
        m.assert();
        m.delete();

        // Test getting a specific item
        let mut m = mock_server.mock(|when, then| {
            when.method(GET).path("/downloadQueue/1/items/123");
            then.status(200)
                .header("content-type", "text/json")
                .body_from_file("tests/mocks/download_queue/item_waiting.json");
        });

        let item = queue.item(123).await.unwrap();
        assert_eq!(item.id(), 123);
        assert_eq!(item.key(), "/library/metadata/159637");
        m.assert();
        m.delete();
    }

    #[plex_api_test_helper::offline_test]
    async fn queue_item(#[future] server_authenticated: Mocked<Server>) {
        let (server, mock_server) = server_authenticated.split();

        let mut m = mock_server.mock(|when, then| {
            when.method(GET).path("/library/metadata/159637");
            then.status(200)
                .header("content-type", "text/json")
                .body_from_file("tests/mocks/transcode/metadata_159637.json");
        });

        let item: Movie = server
            .item_by_id("159637")
            .await
            .unwrap()
            .try_into()
            .unwrap();
        m.assert();
        m.delete();

        // Mock the queue creation
        let mut m = mock_server.mock(|when, then| {
            when.method(httpmock::Method::POST).path("/downloadQueue");
            then.status(200)
                .header("content-type", "text/json")
                .body_from_file("tests/mocks/download_queue/queue_created.json");
        });

        // Mock adding the item to the queue
        let mut m2 = mock_server.mock(|when, then| {
            when.method(httpmock::Method::POST)
                .path("/downloadQueue/1/add")
                .query_param_exists("session")
                .query_param_exists("transcodeSessionId")
                .query_param("transcodeType", "video")
                .query_param("path", "/library/metadata/159637")
                .query_param("keys", "/library/metadata/159637")
                .query_param_exists("mediaIndex")
                .query_param_exists("partIndex")
                .query_param_exists("directPlay")
                .query_param_exists("directStream")
                .query_param_exists("directStreamAudio")
                .query_param("context", "static")
                .query_param("maxVideoBitrate", "2000")
                .query_param("videoBitrate", "2000")
                .query_param("videoResolution", "1280x720")
                .query_param_exists("subtitles")
                .query_param_exists("subtitleSize")
                .query_param_exists("X-Plex-Client-Profile-Extra")
                .is_true(|req| {
                    // Verify that the transcode options are correctly passed via the profile
                    let settings = expand_profile(req);

                    // Verify we have only one transcode target (for Mp4) and one direct play profile
                    assert_setting_count(&settings, "add-transcode-target", 1);
                    assert_setting_count(&settings, "add-direct-play-profile", 1);

                    // Verify MP4 transcode target
                    assert_setting(
                        &settings,
                        "add-transcode-target",
                        &[
                            ("type", "videoProfile"),
                            ("context", "static"),
                            ("protocol", "http"),
                            ("container", "mp4"),
                            ("videoCodec", "h264"),
                            ("audioCodec", "aac"),
                            ("subtitleCodec", ""),
                            ("replace", "true"),
                        ],
                    );

                    // Verify direct play profile
                    assert_setting(
                        &settings,
                        "add-direct-play-profile",
                        &[
                            ("type", "videoProfile"),
                            ("container", "mp4"),
                            ("videoCodec", "h264"),
                            ("audioCodec", "aac"),
                            ("subtitleCodec", ""),
                            ("replace", "true"),
                        ],
                    );

                    true
                });
            then.status(200)
                .header("content-type", "text/json")
                .body_from_file("tests/mocks/download_queue/add_item_response.json");
        });

        // Mock fetching the item in Deciding state
        let mut m3 = mock_server.mock(|when, then| {
            when.method(GET).path("/downloadQueue/1/items/123");
            then.status(200)
                .header("content-type", "text/json")
                .body_from_file("tests/mocks/download_queue/item_deciding.json");
        });

        let mut queue_item = item
            .queue_download(
                plex_api::transcode::VideoTranscodeOptions {
                    bitrate: 2000,
                    width: 1280,
                    height: 720,
                    containers: vec![
                        plex_api::media_container::server::library::ContainerFormat::Mp4,
                    ],
                    video_codecs: vec![VideoCodec::H264],
                    audio_codecs: vec![AudioCodec::Aac],
                    ..Default::default()
                },
                None,
            )
            .await
            .unwrap();

        m.assert();
        m.delete();
        m2.assert();
        m2.delete();
        m3.assert();
        m3.delete();

        // Verify initial state is Deciding
        assert!(matches!(
            queue_item.status(),
            plex_api::transcode::QueueItemStatus::Deciding
        ));
        assert_eq!(queue_item.id(), 123);
        assert_eq!(queue_item.key(), "/library/metadata/159637");

        // Update to Waiting state
        let mut m = mock_server.mock(|when, then| {
            when.method(GET).path("/downloadQueue/1/items/123");
            then.status(200)
                .header("content-type", "text/json")
                .body_from_file("tests/mocks/download_queue/item_waiting.json");
        });

        queue_item.update().await.unwrap();
        m.assert();
        m.delete();

        assert!(matches!(
            queue_item.status(),
            plex_api::transcode::QueueItemStatus::Waiting
        ));

        // Update to Processing state
        let mut m = mock_server.mock(|when, then| {
            when.method(GET).path("/downloadQueue/1/items/123");
            then.status(200)
                .header("content-type", "text/json")
                .body_from_file("tests/mocks/download_queue/item_processing.json");
        });

        queue_item.update().await.unwrap();
        m.assert();
        m.delete();

        assert!(matches!(
            queue_item.status(),
            plex_api::transcode::QueueItemStatus::Processing
        ));
        assert!(queue_item.is_transcode());
        let stats = queue_item.stats().unwrap();
        assert_eq!(stats.progress, 25.5_f32);
        assert_eq!(stats.speed, Some(2.5_f32));

        // Update to Available state
        let mut m = mock_server.mock(|when, then| {
            when.method(GET).path("/downloadQueue/1/items/123");
            then.status(200)
                .header("content-type", "text/json")
                .body_from_file("tests/mocks/download_queue/item_available.json");
        });

        queue_item.update().await.unwrap();
        m.assert();
        m.delete();

        assert!(matches!(
            queue_item.status(),
            plex_api::transcode::QueueItemStatus::Available
        ));
    }
}

mod online {
    use plex_api::{
        media_container::server::library::{
            AudioCodec, ContainerFormat, Decision, Protocol, VideoCodec,
        },
        transcode::{QueueItem, QueueItemStatus},
        Server,
    };
    use std::time::Duration;
    use tokio::time::sleep;

    /// Waits for an item to start transcoding.
    async fn wait_for_transcode_start(item: &mut QueueItem) {
        let mut count = 0;
        loop {
            if !matches!(
                item.status(),
                QueueItemStatus::Deciding | QueueItemStatus::Waiting
            ) {
                break;
            }
            sleep(Duration::from_millis(250)).await;
            item.update().await.unwrap();
            count += 1;

            if count > 480 {
                panic!("Waited too long for transcode to start");
            }
        }
    }

    /// Waits for an item to become available.
    async fn wait_for_available(item: &mut QueueItem) {
        let mut count = 0;
        loop {
            if matches!(item.status(), QueueItemStatus::Available) {
                break;
            }
            sleep(Duration::from_millis(250)).await;
            item.update().await.unwrap();
            count += 1;

            if count > 480 {
                panic!("Waited too long for item to become available");
            }
        }
    }

    /// Checks the item was correct.
    fn verify_transcoded_item(
        item: &QueueItem,
        container: ContainerFormat,
        audio: (Decision, AudioCodec),
        video: Option<(Decision, VideoCodec)>,
    ) {
        assert!(item.is_transcode());

        match item.status() {
            QueueItemStatus::Waiting => {
                // It may have taken too long to start transcoding. This isn't
                // an error, but it is unfortunate as we then can't check the
                // transcode options.
            }
            QueueItemStatus::Available => {
                // If the server is too fast the item may have completed
                // already. This isn't an error, but it is unfortunate as we
                // then can't check the transcode options.
            }
            QueueItemStatus::Processing => {
                let stats = item
                    .stats()
                    .expect("Stats should be available when processing");

                assert_eq!(stats.protocol, Protocol::Http);
                assert_eq!(stats.container, container);
                assert_eq!(stats.audio_decision, Some(audio.0));
                assert_eq!(stats.audio_codec, Some(audio.1));
                assert_eq!(stats.video_decision, video.map(|v| v.0));
                assert_eq!(stats.video_codec, video.map(|v| v.1));
            }
            status => panic!("Unexpected status: {status:?}"),
        }
    }

    mod movie {
        use super::{super::fixtures::online::server::server, *};
        use mp4::{AvcProfile, MediaType, Mp4Reader, TrackType};
        use plex_api::{
            library::{MediaItem, MetadataItem, Movie, Transcodable},
            transcode::VideoTranscodeOptions,
        };
        use std::io::Cursor;

        #[plex_api_test_helper::online_test_claimed_server]
        async fn queue_transcode(
            #[future]
            #[with("Generic".to_owned())]
            server: Server,
        ) {
            let queue = server.download_queue().await.unwrap();

            let movie: Movie = server.item_by_id("57").await.unwrap().try_into().unwrap();
            assert_eq!(movie.title(), "Sintel");

            let media = &movie.media()[0];
            let part = &media.parts()[0];

            let mut item = part
                .queue_download(
                    // These settings will force transcoding as the original has
                    // higher bitrate and has a different audio codec.
                    VideoTranscodeOptions {
                        bitrate: 110,
                        containers: vec![ContainerFormat::Mp4],
                        video_codecs: vec![VideoCodec::H264],
                        audio_codecs: vec![AudioCodec::Mp3],
                        ..Default::default()
                    },
                    Some(&queue),
                )
                .await
                .unwrap();

            assert_eq!(item.queue(), queue);

            assert!(matches!(item.status(), QueueItemStatus::Deciding));

            wait_for_transcode_start(&mut item).await;

            assert!(item.is_transcode());

            verify_transcoded_item(
                &item,
                ContainerFormat::Mp4,
                (Decision::Transcode, AudioCodec::Mp3),
                Some((Decision::Transcode, VideoCodec::H264)),
            );

            item.delete().await.unwrap();

            assert!(queue.items().await.unwrap().is_empty());

            let movie: Movie = server.item_by_id("55").await.unwrap().try_into().unwrap();
            assert_eq!(movie.title(), "Big Buck Bunny");

            let media = &movie.media()[0];
            assert_eq!(media.parts().len(), 2);

            let mut item = movie
                .queue_download(
                    // These settings will force transcoding as the original has
                    // higher bitrate and has a different audio codec.
                    VideoTranscodeOptions {
                        bitrate: 110,
                        containers: vec![ContainerFormat::Mp4],
                        video_codecs: vec![VideoCodec::H264],
                        audio_codecs: vec![AudioCodec::Mp3],
                        ..Default::default()
                    },
                    None,
                )
                .await
                .unwrap();

            assert!(matches!(item.status(), QueueItemStatus::Deciding));

            wait_for_transcode_start(&mut item).await;

            assert!(item.is_transcode());

            verify_transcoded_item(
                &item,
                ContainerFormat::Mp4,
                (Decision::Transcode, AudioCodec::Mp3),
                Some((Decision::Transcode, VideoCodec::H264)),
            );

            item.delete().await.unwrap();

            assert!(queue.items().await.unwrap().is_empty());
        }

        #[plex_api_test_helper::online_test_claimed_server]
        async fn queue_change_container(
            #[future]
            #[with("Generic".to_owned())]
            server: Server,
        ) {
            let queue = server.download_queue().await.unwrap();

            let movie: Movie = server.item_by_id("57").await.unwrap().try_into().unwrap();
            assert_eq!(movie.title(), "Sintel");

            let media = &movie.media()[0];
            let part = &media.parts()[0];

            let mut item = part
                .queue_download(
                    // These settings should allow for direct streaming of the video
                    // and audio but into a different container format.
                    VideoTranscodeOptions {
                        bitrate: 200000000,
                        width: 1280,
                        height: 720,
                        containers: vec![ContainerFormat::Mp4],
                        video_codecs: vec![VideoCodec::H264],
                        audio_codecs: vec![AudioCodec::Aac],
                        ..Default::default()
                    },
                    Some(&queue),
                )
                .await
                .unwrap();

            assert_eq!(item.queue(), queue);

            assert!(matches!(item.status(), QueueItemStatus::Deciding));

            wait_for_transcode_start(&mut item).await;

            assert!(item.is_transcode());

            verify_transcoded_item(
                &item,
                ContainerFormat::Mp4,
                (Decision::Copy, AudioCodec::Aac),
                Some((Decision::Copy, VideoCodec::H264)),
            );

            // As this transcode is just copying the existing streams into a new
            // container format it should complete quickly allowing us to download
            // the transcoded file.

            wait_for_available(&mut item).await;

            let mut buf = Vec::<u8>::new();
            item.download(&mut buf, ..).await.unwrap();

            item.delete().await.unwrap();

            assert!(queue.items().await.unwrap().is_empty());

            // Verify that the file is a valid MP4 container and the tracks are
            // expected.
            let len = buf.len();
            let cursor = Cursor::new(buf);
            let mp4 = Mp4Reader::read_header(cursor, len as u64).unwrap();

            let mut videos = mp4
                .tracks()
                .values()
                .filter(|t| matches!(t.track_type(), Ok(TrackType::Video)));

            let video = videos.next().unwrap();
            assert!(matches!(video.media_type(), Ok(MediaType::H264)));
            assert_eq!(video.width(), 1280);
            assert_eq!(video.height(), 720);
            // Allow some slop in the durations
            assert!(
                video
                    .duration()
                    .as_millis()
                    .abs_diff(part.duration().unwrap() as u128)
                    < 200,
            );
            assert!(matches!(video.video_profile(), Ok(AvcProfile::AvcHigh)));
            assert!(videos.next().is_none());

            let mut audios = mp4
                .tracks()
                .values()
                .filter(|t| matches!(t.track_type(), Ok(TrackType::Audio)));

            let audio = audios.next().unwrap();
            assert_eq!(audio.media_type().unwrap(), MediaType::AAC);
            assert!(audios.next().is_none());

            let movie: Movie = server.item_by_id("55").await.unwrap().try_into().unwrap();
            assert_eq!(movie.title(), "Big Buck Bunny");

            let media = &movie.media()[0];
            assert_eq!(media.parts().len(), 2);

            let mut item = movie
                .queue_download(
                    // These settings should allow for direct streaming of the video
                    // and audio but into a different container format.
                    VideoTranscodeOptions {
                        bitrate: 200000000,
                        width: 1280,
                        height: 720,
                        containers: vec![ContainerFormat::Mp4],
                        video_codecs: vec![VideoCodec::H264],
                        audio_codecs: vec![AudioCodec::Aac],
                        ..Default::default()
                    },
                    None,
                )
                .await
                .unwrap();

            assert!(matches!(item.status(), QueueItemStatus::Deciding));

            wait_for_transcode_start(&mut item).await;

            assert!(item.is_transcode());

            verify_transcoded_item(
                &item,
                ContainerFormat::Mp4,
                (Decision::Transcode, AudioCodec::Aac),
                Some((Decision::Transcode, VideoCodec::H264)),
            );

            // As this transcode is just copying the existing streams into a new
            // container format it should complete quickly allowing us to download
            // the transcoded file.

            wait_for_available(&mut item).await;

            let mut buf = Vec::<u8>::new();
            item.download(&mut buf, ..).await.unwrap();

            item.delete().await.unwrap();

            assert!(queue.items().await.unwrap().is_empty());

            // Verify that the file is a valid MP4 container and the tracks are
            // expected.
            let len = buf.len();
            let cursor = Cursor::new(buf);
            let mp4 = Mp4Reader::read_header(cursor, len as u64).unwrap();

            let mut videos = mp4
                .tracks()
                .values()
                .filter(|t| matches!(t.track_type(), Ok(TrackType::Video)));

            let video = videos.next().unwrap();
            assert!(matches!(video.media_type(), Ok(MediaType::H264)));
            assert_eq!(video.width(), 1280);
            assert_eq!(video.height(), 720);
            // Allow some slop in the durations
            assert!(
                video
                    .duration()
                    .as_millis()
                    .abs_diff(media.duration().unwrap() as u128)
                    < 200,
            );
            assert_eq!(video.video_profile().unwrap(), AvcProfile::AvcMain);
            assert!(videos.next().is_none());

            let mut audios = mp4
                .tracks()
                .values()
                .filter(|t| matches!(t.track_type(), Ok(TrackType::Audio)));

            let audio = audios.next().unwrap();
            assert!(matches!(audio.media_type(), Ok(MediaType::AAC)));
            assert!(audios.next().is_none());
        }

        #[plex_api_test_helper::online_test_claimed_server]
        async fn queue_direct_play(
            #[future]
            #[with("Generic".to_owned())]
            server: Server,
        ) {
            let queue = server.download_queue().await.unwrap();

            let movie: Movie = server.item_by_id("57").await.unwrap().try_into().unwrap();
            assert_eq!(movie.title(), "Sintel");

            let media = &movie.media()[0];
            let part = &media.parts()[0];

            let mut item = part
                .queue_download(
                    // Here we ask to transcode into a format the movie is already
                    // in so the server denies the request.
                    VideoTranscodeOptions {
                        bitrate: 200000000,
                        width: 1280,
                        height: 720,
                        containers: vec![ContainerFormat::Mkv],
                        video_codecs: vec![VideoCodec::H264],
                        audio_codecs: vec![AudioCodec::Aac],
                        ..Default::default()
                    },
                    Some(&queue),
                )
                .await
                .unwrap();

            assert_eq!(item.queue(), queue);

            assert!(matches!(item.status(), QueueItemStatus::Deciding));

            wait_for_transcode_start(&mut item).await;

            assert!(!item.is_transcode());

            assert_eq!(item.status(), QueueItemStatus::Available);

            let mut buf = Vec::<u8>::new();
            item.download(&mut buf, ..).await.unwrap();

            item.delete().await.unwrap();

            assert!(queue.items().await.unwrap().is_empty());

            assert_eq!(buf.len(), part.len().unwrap() as usize);
        }
    }

    mod music {
        use super::{super::fixtures::online::server::server, *};
        use plex_api::{
            library::{MediaItem, MetadataItem, Track, Transcodable},
            transcode::MusicTranscodeOptions,
        };

        #[plex_api_test_helper::online_test_claimed_server]
        async fn queue_transcode(
            #[future]
            #[with("Generic".to_owned())]
            server: Server,
        ) {
            let queue = server.download_queue().await.unwrap();

            let track: Track = server.item_by_id("158").await.unwrap().try_into().unwrap();
            assert_eq!(track.title(), "Try It Out (Neon mix)");

            let media = &track.media()[0];
            let part = &media.parts()[0];

            assert!(queue.items().await.unwrap().is_empty());

            let mut item = part
                .queue_download(
                    // These settings will force transcoding as the original is too
                    // high a bitrate and has a different audio codec.
                    MusicTranscodeOptions {
                        bitrate: 92,
                        containers: vec![ContainerFormat::Mp3],
                        codecs: vec![AudioCodec::Mp3],
                        ..Default::default()
                    },
                    None,
                )
                .await
                .unwrap();

            assert_eq!(item.queue(), queue);

            assert!(matches!(item.status(), QueueItemStatus::Deciding));

            wait_for_transcode_start(&mut item).await;

            assert!(item.is_transcode());

            verify_transcoded_item(
                &item,
                ContainerFormat::Mp3,
                (Decision::Transcode, AudioCodec::Mp3),
                None,
            );

            // Audio transcoding should be reasonably fast...

            wait_for_available(&mut item).await;

            let mut buf = Vec::<u8>::new();
            item.download(&mut buf, ..).await.unwrap();

            item.delete().await.unwrap();

            assert!(queue.items().await.unwrap().is_empty());

            // Check a few unlikely to change properties about the stream.
            let metadata = mp3_metadata::read_from_slice(&buf).unwrap();
            assert_eq!(metadata.duration.as_secs(), 5);
            let frame = metadata.frames.first().unwrap();
            assert_eq!(frame.layer, mp3_metadata::Layer::Layer3);
            assert_eq!(frame.chan_type, mp3_metadata::ChannelType::SingleChannel);
        }

        #[plex_api_test_helper::online_test_claimed_server]
        async fn queue_direct_play(
            #[future]
            #[with("Generic".to_owned())]
            server: Server,
        ) {
            let queue = server.download_queue().await.unwrap();

            let track: Track = server.item_by_id("158").await.unwrap().try_into().unwrap();
            assert_eq!(track.title(), "Try It Out (Neon mix)");

            let media = &track.media()[0];
            let part = &media.parts()[0];

            let mut item = part
                .queue_download(
                    // Here we ask to transcode into a format the music is already
                    // in so the server denies the request.
                    MusicTranscodeOptions {
                        bitrate: 200000000,
                        containers: vec![ContainerFormat::Aac],
                        codecs: vec![AudioCodec::Aac],
                        ..Default::default()
                    },
                    Some(&queue),
                )
                .await
                .unwrap();

            assert_eq!(item.queue(), queue);

            assert!(matches!(item.status(), QueueItemStatus::Deciding));

            wait_for_transcode_start(&mut item).await;

            assert!(!item.is_transcode());

            assert_eq!(item.status(), QueueItemStatus::Available);

            let mut buf = Vec::<u8>::new();
            item.download(&mut buf, ..).await.unwrap();

            item.delete().await.unwrap();

            assert!(queue.items().await.unwrap().is_empty());

            assert_eq!(buf.len(), part.len().unwrap() as usize);
        }
    }
}
