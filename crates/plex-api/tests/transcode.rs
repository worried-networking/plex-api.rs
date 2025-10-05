mod fixtures;

mod offline {
    use std::collections::HashMap;

    use super::fixtures::offline::{server::*, Mocked};
    use httpmock::{prelude::HttpMockRequest, Method::GET};
    use plex_api::{
        media_container::server::library::{
            AudioCodec, ContainerFormat, Decision, Protocol, VideoCodec,
        },
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
    async fn transcode_sessions(#[future] server_authenticated: Mocked<Server>) {
        let (server, mock_server) = server_authenticated.split();

        let mut m = mock_server.mock(|when, then| {
            when.method(GET).path("/transcode/sessions");
            then.status(200)
                .header("content-type", "text/json")
                .body_from_file("tests/mocks/transcode/video_sessions.json");
        });

        let sessions = server.transcode_sessions().await.unwrap();
        m.assert();
        m.delete();

        assert_eq!(sessions.len(), 1);
        let session = &sessions[0];
        assert!(session.is_offline());
        assert_eq!(
            session.session_id(),
            "6c624c15015644a2801002562d2c33e4fdbf54cb"
        );
        assert_eq!(session.container(), ContainerFormat::Mkv);
        assert_eq!(session.protocol(), Protocol::Http);
        assert_eq!(
            session.audio_transcode(),
            Some((Decision::Transcode, AudioCodec::Mp3))
        );
        assert_eq!(
            session.video_transcode(),
            Some((Decision::Transcode, VideoCodec::H264))
        );

        let mut m = mock_server.mock(|when, then| {
            when.method(GET).path("/transcode/sessions");
            then.status(200)
                .header("content-type", "text/json")
                .body_from_file("tests/mocks/transcode/music_sessions.json");
        });

        let sessions = server.transcode_sessions().await.unwrap();
        m.assert();
        m.delete();

        assert_eq!(sessions.len(), 1);
        let session = &sessions[0];
        assert!(!session.is_offline());
        assert_eq!(session.session_id(), "dfghtybntbretybrtyb");
        assert_eq!(session.container(), ContainerFormat::Mp4);
        assert_eq!(session.protocol(), Protocol::Dash);
        assert_eq!(
            session.audio_transcode(),
            Some((Decision::Copy, AudioCodec::Mp3))
        );
        assert_eq!(session.video_transcode(), None);

        let mut m = mock_server.mock(|when, then| {
            when.method(GET)
                .path("/transcode/sessions/dfghtybntbretybrtyb");
            then.status(200)
                .header("content-type", "text/json")
                .body_from_file("tests/mocks/transcode/music_sessions.json");
        });

        let session = server
            .transcode_session("dfghtybntbretybrtyb")
            .await
            .unwrap();
        m.assert();
        m.delete();

        assert!(!session.is_offline());
        assert_eq!(session.session_id(), "dfghtybntbretybrtyb");
        assert_eq!(session.container(), ContainerFormat::Mp4);
        assert_eq!(session.protocol(), Protocol::Dash);
        assert_eq!(
            session.audio_transcode(),
            Some((Decision::Copy, AudioCodec::Mp3))
        );
        assert_eq!(session.video_transcode(), None);

        let mut m = mock_server.mock(|when, then| {
            when.method(GET)
                .path("/transcode/sessions/dfghtybntbretybrtyb");
            then.status(404);
        });

        let error = session.stats().await.err().unwrap();
        m.assert();
        m.delete();

        assert!(matches!(error, plex_api::Error::ItemNotFound));

        let mut m = mock_server.mock(|when, then| {
            when.method(GET).path("/transcode/sessions/gfbrgbrbrfber");
            then.status(200)
                .header("content-type", "text/json")
                .body_from_file("tests/mocks/transcode/empty_sessions.json");
        });

        let error = server
            .transcode_session("gfbrgbrbrfber")
            .await
            .err()
            .unwrap();
        m.assert();
        m.delete();

        assert!(matches!(error, plex_api::Error::ItemNotFound));

        let mut m = mock_server.mock(|when, then| {
            when.method(GET).path("/transcode/sessions/gfbrgbrbrfber");
            then.status(404);
        });

        let error = server
            .transcode_session("gfbrgbrbrfber")
            .await
            .err()
            .unwrap();
        m.assert();
        m.delete();

        assert!(matches!(error, plex_api::Error::ItemNotFound));
    }

    mod movie {
        use super::*;
        use plex_api::{
            library::{MediaItem, MediaItemWithTranscoding, Movie},
            media_container::server::library::SubtitleCodec,
            transcode::{AudioSetting, Constraint, VideoSetting, VideoTranscodeOptions},
        };

        #[plex_api_test_helper::offline_test]
        async fn transcode_profile_params(#[future] server_authenticated: Mocked<Server>) {
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

            let media = &item.media()[0];
            let part = &media.parts()[0];

            let mut m = mock_server.mock(|when, then| {
                when.method(GET)
                    .path("/video/:/transcode/universal/decision")
                    .query_param_exists("session")
                    .query_param_exists("transcodeSessionId")
                    .query_param("transcodeType", "video")
                    .query_param("path", "/library/metadata/159637")
                    .query_param("mediaIndex", "0")
                    .query_param("partIndex", "0")
                    .query_param("directPlay", "0")
                    .query_param("directStream", "1")
                    .query_param("directStreamAudio", "1")
                    .query_param("context", "streaming")
                    .query_param("maxVideoBitrate", "2000")
                    .query_param("videoBitrate", "2000")
                    .query_param("videoResolution", "1280x720")
                    .query_param("subtitles", "burn")
                    .query_param("protocol", "dash")
                    .query_param_exists("X-Plex-Client-Profile-Extra")
                    .is_true(|req| {
                        let settings = expand_profile(req);

                        assert_setting_count(&settings, "add-transcode-target", 1);
                        assert_setting_count(&settings, "add-direct-play-profile", 0);
                        assert_setting_count(&settings, "append-transcode-target-codec", 0);
                        assert_setting_count(&settings, "add-transcode-target-audio-codec", 0);
                        assert_setting_count(&settings, "add-limitation", 0);

                        assert_setting(
                            &settings,
                            "add-transcode-target",
                            &[
                                ("type", "videoProfile"),
                                ("context", "streaming"),
                                ("protocol", "dash"),
                                ("container", "mp4"),
                                ("videoCodec", "h264"),
                                ("audioCodec", "aac,mp3"),
                                ("subtitleCodec", ""),
                                ("replace", "true"),
                            ],
                        );

                        true
                    });
                then.status(200)
                    .header("content-type", "text/json")
                    .body_from_file("tests/mocks/transcode/video_dash_h264_mp3.json");
            });

            part.create_streaming_session(
                Protocol::Dash,
                VideoTranscodeOptions {
                    bitrate: 2000,
                    width: 1280,
                    height: 720,
                    burn_subtitles: true,
                    video_codecs: vec![VideoCodec::H264],
                    audio_codecs: vec![AudioCodec::Aac, AudioCodec::Mp3],
                    ..Default::default()
                },
            )
            .await
            .unwrap();
            m.assert();
            m.delete();

            let media = &item.media()[1];
            let part = &media.parts()[0];

            let mut m = mock_server.mock(|when, then| {
                when.method(GET)
                    .path("/video/:/transcode/universal/decision")
                    .query_param_exists("session")
                    .query_param_exists("transcodeSessionId")
                    .query_param("transcodeType", "video")
                    .query_param("path", "/library/metadata/159637")
                    .query_param("mediaIndex", "1")
                    .query_param("partIndex", "0")
                    .query_param("directPlay", "0")
                    .query_param("directStream", "1")
                    .query_param("directStreamAudio", "1")
                    .query_param("context", "streaming")
                    .query_param("maxVideoBitrate", "1000")
                    .query_param("videoBitrate", "1000")
                    .query_param("videoResolution", "1920x1080")
                    .query_param("protocol", "hls")
                    .query_param_exists("X-Plex-Client-Profile-Extra")
                    .is_true(|req| {
                        let settings = expand_profile(req);

                        assert_setting_count(&settings, "add-transcode-target", 1);
                        assert_setting_count(&settings, "add-direct-play-profile", 0);
                        assert_setting_count(&settings, "append-transcode-target-codec", 0);
                        assert_setting_count(&settings, "add-transcode-target-audio-codec", 0);
                        assert_setting_count(&settings, "add-limitation", 3);

                        assert_setting(
                            &settings,
                            "add-transcode-target",
                            &[
                                ("type", "videoProfile"),
                                ("context", "streaming"),
                                ("protocol", "hls"),
                                ("container", "mpegts"),
                                ("videoCodec", "vp9,vp8"),
                                ("audioCodec", "eac3"),
                                ("subtitleCodec", "ass"),
                                ("replace", "true"),
                            ],
                        );

                        assert_setting(
                            &settings,
                            "add-limitation",
                            &[
                                ("scope", "videoCodec"),
                                ("scopeName", "*"),
                                ("name", "video.bitDepth"),
                                ("type", "upperBound"),
                                ("value", "8"),
                            ],
                        );

                        assert_setting(
                            &settings,
                            "add-limitation",
                            &[
                                ("scope", "videoCodec"),
                                ("scopeName", "vp9"),
                                ("name", "video.profile"),
                                ("type", "match"),
                                ("list", "main|baseline"),
                            ],
                        );

                        assert_setting(
                            &settings,
                            "add-limitation",
                            &[
                                ("scope", "videoAudioCodec"),
                                ("scopeName", "*"),
                                ("name", "audio.channels"),
                                ("type", "upperBound"),
                                ("value", "2"),
                            ],
                        );

                        true
                    });
                then.status(200)
                    .header("content-type", "text/json")
                    .body_from_file("tests/mocks/transcode/video_hls_vp9_pcm.json");
            });

            part.create_streaming_session(
                Protocol::Hls,
                VideoTranscodeOptions {
                    bitrate: 1000,
                    width: 1920,
                    height: 1080,
                    video_codecs: vec![VideoCodec::Vp9, VideoCodec::Vp8],
                    audio_codecs: vec![AudioCodec::Eac3],
                    video_limitations: vec![
                        (VideoSetting::BitDepth, Constraint::Max("8".to_string())).into(),
                        (
                            VideoCodec::Vp9,
                            VideoSetting::Profile,
                            Constraint::Match(vec!["main".to_string(), "baseline".to_string()]),
                        )
                            .into(),
                    ],
                    audio_limitations: vec![(
                        AudioSetting::Channels,
                        Constraint::Max("2".to_string()),
                    )
                        .into()],
                    subtitle_codecs: vec![SubtitleCodec::Ass],
                    ..Default::default()
                },
            )
            .await
            .unwrap();
            m.assert();
            m.delete();

            let media = &item.media()[1];
            let part = &media.parts()[1];

            let mut m = mock_server.mock(|when, then| {
                when.method(GET)
                    .path("/video/:/transcode/universal/decision")
                    .query_param_exists("session")
                    .query_param_exists("transcodeSessionId")
                    .query_param("transcodeType", "video")
                    .query_param("path", "/library/metadata/159637")
                    .query_param("mediaIndex", "1")
                    .query_param("partIndex", "1")
                    .query_param("directPlay", "1")
                    .query_param("directStream", "1")
                    .query_param("directStreamAudio", "1")
                    .query_param("context", "static")
                    .query_param("maxVideoBitrate", "2000")
                    .query_param("videoBitrate", "2000")
                    .query_param("videoResolution", "1280x720")
                    .query_param("subtitles", "burn")
                    .query_param("offlineTranscode", "1")
                    .query_param_exists("X-Plex-Client-Profile-Extra")
                    .is_true(|req| {
                        let settings = expand_profile(req);

                        assert_setting_count(&settings, "add-transcode-target", 2);
                        assert_setting_count(&settings, "add-direct-play-profile", 1);
                        assert_setting_count(&settings, "append-transcode-target-codec", 0);
                        assert_setting_count(&settings, "add-transcode-target-audio-codec", 0);
                        assert_setting_count(&settings, "add-limitation", 0);

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

                        assert_setting(
                            &settings,
                            "add-transcode-target",
                            &[
                                ("type", "videoProfile"),
                                ("context", "static"),
                                ("protocol", "http"),
                                ("container", "mkv"),
                                ("videoCodec", "h264"),
                                ("audioCodec", "aac"),
                                ("subtitleCodec", ""),
                            ],
                        );

                        assert_setting(
                            &settings,
                            "add-direct-play-profile",
                            &[
                                ("type", "videoProfile"),
                                ("container", "mp4,mkv"),
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
                    .body_from_file("tests/mocks/transcode/video_offline_h264_mp3.json");
            });

            part.create_download_session(VideoTranscodeOptions {
                bitrate: 2000,
                width: 1280,
                height: 720,
                burn_subtitles: true,
                video_codecs: vec![VideoCodec::H264],
                audio_codecs: vec![AudioCodec::Aac],
                ..Default::default()
            })
            .await
            .unwrap();
            m.assert();
            m.delete();

            let mut m = mock_server.mock(|when, then| {
                when.method(GET)
                    .path("/video/:/transcode/universal/decision")
                    .query_param_exists("session")
                    .query_param_exists("transcodeSessionId")
                    .query_param("transcodeType", "video")
                    .query_param("path", "/library/metadata/159637")
                    .query_param("mediaIndex", "1")
                    .query_param("partIndex", "-1")
                    .query_param("directPlay", "1")
                    .query_param("directStream", "1")
                    .query_param("directStreamAudio", "1")
                    .query_param("context", "static")
                    .query_param("maxVideoBitrate", "2000")
                    .query_param("videoBitrate", "2000")
                    .query_param("videoResolution", "1280x720")
                    .query_param("subtitles", "burn")
                    .query_param("offlineTranscode", "1")
                    .query_param_exists("X-Plex-Client-Profile-Extra")
                    .is_true(|req| {
                        let settings = expand_profile(req);

                        assert_setting_count(&settings, "add-transcode-target", 2);
                        assert_setting_count(&settings, "add-direct-play-profile", 1);
                        assert_setting_count(&settings, "append-transcode-target-codec", 0);
                        assert_setting_count(&settings, "add-transcode-target-audio-codec", 0);
                        assert_setting_count(&settings, "add-limitation", 0);

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

                        assert_setting(
                            &settings,
                            "add-transcode-target",
                            &[
                                ("type", "videoProfile"),
                                ("context", "static"),
                                ("protocol", "http"),
                                ("container", "mkv"),
                                ("videoCodec", "h264"),
                                ("audioCodec", "aac"),
                                ("subtitleCodec", ""),
                            ],
                        );

                        assert_setting(
                            &settings,
                            "add-direct-play-profile",
                            &[
                                ("type", "videoProfile"),
                                ("container", "mp4,mkv"),
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
                    .body_from_file("tests/mocks/transcode/video_offline_h264_mp3.json");
            });

            media
                .create_download_session(VideoTranscodeOptions {
                    bitrate: 2000,
                    width: 1280,
                    height: 720,
                    burn_subtitles: true,
                    video_codecs: vec![VideoCodec::H264],
                    audio_codecs: vec![AudioCodec::Aac],
                    ..Default::default()
                })
                .await
                .unwrap();
            m.assert();
            m.delete();

            let mut m = mock_server.mock(|when, then| {
                when.method(GET)
                    .path("/video/:/transcode/universal/decision")
                    .query_param_exists("session")
                    .query_param_exists("transcodeSessionId")
                    .query_param("transcodeType", "video")
                    .query_param("path", "/library/metadata/159637")
                    .query_param("mediaIndex", "-1")
                    .query_param("partIndex", "-1")
                    .query_param("directPlay", "1")
                    .query_param("directStream", "1")
                    .query_param("directStreamAudio", "1")
                    .query_param("context", "static")
                    .query_param("maxVideoBitrate", "2000")
                    .query_param("videoBitrate", "2000")
                    .query_param("videoResolution", "1280x720")
                    .query_param("subtitles", "burn")
                    .query_param("offlineTranscode", "1")
                    .query_param_exists("X-Plex-Client-Profile-Extra")
                    .is_true(|req| {
                        let settings = expand_profile(req);

                        assert_setting_count(&settings, "add-transcode-target", 2);
                        assert_setting_count(&settings, "add-direct-play-profile", 1);
                        assert_setting_count(&settings, "append-transcode-target-codec", 0);
                        assert_setting_count(&settings, "add-transcode-target-audio-codec", 0);
                        assert_setting_count(&settings, "add-limitation", 0);

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

                        assert_setting(
                            &settings,
                            "add-transcode-target",
                            &[
                                ("type", "videoProfile"),
                                ("context", "static"),
                                ("protocol", "http"),
                                ("container", "mkv"),
                                ("videoCodec", "h264"),
                                ("audioCodec", "aac"),
                                ("subtitleCodec", ""),
                            ],
                        );

                        assert_setting(
                            &settings,
                            "add-direct-play-profile",
                            &[
                                ("type", "videoProfile"),
                                ("container", "mp4,mkv"),
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
                    .body_from_file("tests/mocks/transcode/video_offline_h264_mp3.json");
            });

            item.create_download_session(VideoTranscodeOptions {
                bitrate: 2000,
                width: 1280,
                height: 720,
                burn_subtitles: true,
                video_codecs: vec![VideoCodec::H264],
                audio_codecs: vec![AudioCodec::Aac],
                ..Default::default()
            })
            .await
            .unwrap();
            m.assert();
            m.delete();
        }

        #[plex_api_test_helper::offline_test]
        async fn transcode_decision(#[future] server_authenticated: Mocked<Server>) {
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

            let media = &item.media()[0];
            let part = &media.parts()[0];

            let mut m = mock_server.mock(|when, then| {
                when.method(GET)
                    .path("/video/:/transcode/universal/decision");
                then.status(200)
                    .header("content-type", "text/json")
                    .body_from_file("tests/mocks/transcode/video_dash_h264_mp3.json");
            });

            let session = part
                .create_streaming_session(Protocol::Dash, VideoTranscodeOptions::default())
                .await
                .unwrap();
            m.assert();
            m.delete();

            assert!(!session.is_offline());
            assert_eq!(session.container(), ContainerFormat::Mp4);
            assert_eq!(session.protocol(), Protocol::Dash);
            assert_eq!(
                session.audio_transcode(),
                Some((Decision::Transcode, AudioCodec::Mp3))
            );
            assert_eq!(
                session.video_transcode(),
                Some((Decision::Transcode, VideoCodec::H264))
            );

            let mut m = mock_server.mock(|when, then| {
                when.method(GET)
                    .path("/video/:/transcode/universal/decision");
                then.status(200)
                    .header("content-type", "text/json")
                    .body_from_file("tests/mocks/transcode/video_dash_h265_aac.json");
            });

            let session = part
                .create_streaming_session(Protocol::Dash, VideoTranscodeOptions::default())
                .await
                .unwrap();
            m.assert();
            m.delete();

            assert!(!session.is_offline());
            assert_eq!(session.container(), ContainerFormat::Mp4);
            assert_eq!(session.protocol(), Protocol::Dash);
            assert_eq!(
                session.audio_transcode(),
                Some((Decision::Transcode, AudioCodec::Aac))
            );
            assert_eq!(
                session.video_transcode(),
                Some((Decision::Copy, VideoCodec::Hevc))
            );

            let mut m = mock_server.mock(|when, then| {
                when.method(GET)
                    .path("/video/:/transcode/universal/decision");
                then.status(200)
                    .header("content-type", "text/json")
                    .body_from_file("tests/mocks/transcode/video_hls_vp9_pcm.json");
            });

            let session = part
                .create_streaming_session(Protocol::Hls, VideoTranscodeOptions::default())
                .await
                .unwrap();
            m.assert();
            m.delete();

            assert!(!session.is_offline());
            assert_eq!(session.container(), ContainerFormat::MpegTs);
            assert_eq!(session.protocol(), Protocol::Hls);
            assert_eq!(
                session.audio_transcode(),
                Some((Decision::Copy, AudioCodec::Pcm))
            );
            assert_eq!(
                session.video_transcode(),
                Some((Decision::Transcode, VideoCodec::Vp9))
            );

            let mut m = mock_server.mock(|when, then| {
                when.method(GET)
                    .path("/video/:/transcode/universal/decision");
                then.status(200)
                    .header("content-type", "text/json")
                    .body_from_file("tests/mocks/transcode/video_hls_vp9_pcm.json");
            });

            let error = part
                .create_streaming_session(Protocol::Dash, VideoTranscodeOptions::default())
                .await
                .err()
                .unwrap();
            m.assert();
            m.delete();

            if let plex_api::Error::TranscodeError(message) = error {
                assert_eq!(message, "Server returned an invalid protocol.");
            } else {
                panic!("Unexpected error {error}");
            }

            let mut m = mock_server.mock(|when, then| {
                when.method(GET)
                    .path("/video/:/transcode/universal/decision");
                then.status(200)
                    .header("content-type", "text/json")
                    .body_from_file("tests/mocks/transcode/video_dash_h264_mp3.json");
            });

            let error = part
                .create_streaming_session(Protocol::Hls, VideoTranscodeOptions::default())
                .await
                .err()
                .unwrap();
            m.assert();
            m.delete();

            if let plex_api::Error::TranscodeError(message) = error {
                assert_eq!(message, "Server returned an invalid protocol.");
            } else {
                panic!("Unexpected error {error}");
            }

            let mut m = mock_server.mock(|when, then| {
                when.method(GET)
                    .path("/video/:/transcode/universal/decision");
                then.status(200)
                    .header("content-type", "text/json")
                    .body_from_file("tests/mocks/transcode/video_offline_h264_mp3.json");
            });

            let session = part
                .create_download_session(VideoTranscodeOptions::default())
                .await
                .unwrap();
            m.assert();
            m.delete();

            assert!(session.is_offline());
            assert_eq!(session.container(), ContainerFormat::Mp4);
            assert_eq!(session.protocol(), Protocol::Http);
            assert_eq!(
                session.audio_transcode(),
                Some((Decision::Transcode, AudioCodec::Mp3))
            );
            assert_eq!(
                session.video_transcode(),
                Some((Decision::Transcode, VideoCodec::H264))
            );

            let mut m = mock_server.mock(|when, then| {
                when.method(GET).path("/library/metadata/1036");
                then.status(200)
                    .header("content-type", "text/json")
                    .body_from_file("tests/mocks/transcode/metadata_1036.json");
            });

            let item: Movie = server.item_by_id("1036").await.unwrap().try_into().unwrap();
            m.assert();
            m.delete();

            let media = &item.media()[0];
            let part = &media.parts()[0];

            let mut m = mock_server.mock(|when, then| {
                when.method(GET)
                    .path("/video/:/transcode/universal/decision");
                then.status(200)
                    .header("content-type", "text/json")
                    .body_from_file("tests/mocks/transcode/video_offline_refused.json");
            });

            let error = part
                .create_download_session(VideoTranscodeOptions::default())
                .await
                .err()
                .unwrap();
            m.assert();
            m.delete();

            assert!(matches!(error, plex_api::Error::TranscodeRefused));

            let mut m = mock_server.mock(|when, then| {
                when.method(GET).path("/library/metadata/13194");
                then.status(200)
                    .header("content-type", "text/json")
                    .body_from_file("tests/mocks/transcode/metadata_13194.json");
            });

            let item: Movie = server
                .item_by_id("13194")
                .await
                .unwrap()
                .try_into()
                .unwrap();
            m.assert();
            m.delete();

            let media = &item.media()[0];
            let part = &media.parts()[0];

            let mut m = mock_server.mock(|when, then| {
                when.method(GET)
                    .path("/video/:/transcode/universal/decision");
                then.status(200)
                    .header("content-type", "text/json")
                    .body_from_file("tests/mocks/transcode/decision_13194.json");
            });

            let error = part
                .create_download_session(VideoTranscodeOptions::default())
                .await
                .err()
                .unwrap();
            m.assert();
            m.delete();

            assert!(matches!(error, plex_api::Error::TranscodeRefused));
        }
    }

    mod music {
        use super::*;
        use plex_api::{
            library::{MediaItem, MediaItemWithTranscoding, Track},
            transcode::{AudioSetting, Constraint, MusicTranscodeOptions},
        };

        #[plex_api_test_helper::offline_test]
        async fn transcode_profile_params(#[future] server_authenticated: Mocked<Server>) {
            let (server, mock_server) = server_authenticated.split();

            let mut m = mock_server.mock(|when, then| {
                when.method(GET).path("/library/metadata/157786");
                then.status(200)
                    .header("content-type", "text/json")
                    .body_from_file("tests/mocks/transcode/metadata_157786.json");
            });

            let item: Track = server
                .item_by_id("157786")
                .await
                .unwrap()
                .try_into()
                .unwrap();
            m.assert();
            m.delete();

            let media = &item.media()[0];
            let part = &media.parts()[0];

            let mut m = mock_server.mock(|when, then| {
                when.method(GET)
                    .path("/video/:/transcode/universal/decision")
                    .query_param_exists("session")
                    .query_param_exists("transcodeSessionId")
                    .query_param("transcodeType", "music")
                    .query_param("path", "/library/metadata/157786")
                    .query_param("mediaIndex", "0")
                    .query_param("partIndex", "0")
                    .query_param("directPlay", "0")
                    .query_param("directStream", "1")
                    .query_param("directStreamAudio", "1")
                    .query_param("context", "streaming")
                    .query_param("musicBitrate", "192")
                    .query_param("protocol", "dash")
                    .query_param_exists("X-Plex-Client-Profile-Extra")
                    .is_true(|req| {
                        let settings = expand_profile(req);

                        assert_setting_count(&settings, "add-transcode-target", 1);
                        assert_setting_count(&settings, "add-direct-play-profile", 0);
                        assert_setting_count(&settings, "append-transcode-target-codec", 0);
                        assert_setting_count(&settings, "add-transcode-target-audio-codec", 0);
                        assert_setting_count(&settings, "add-limitation", 1);

                        assert_setting(
                            &settings,
                            "add-transcode-target",
                            &[
                                ("type", "musicProfile"),
                                ("context", "streaming"),
                                ("protocol", "dash"),
                                ("container", "mp4"),
                                ("audioCodec", "mp3,vorbis"),
                                ("replace", "true"),
                            ],
                        );

                        assert_setting(
                            &settings,
                            "add-limitation",
                            &[
                                ("scope", "audioCodec"),
                                ("scopeName", "*"),
                                ("name", "audio.channels"),
                                ("type", "upperBound"),
                                ("value", "2"),
                            ],
                        );

                        true
                    });
                then.status(200)
                    .header("content-type", "text/json")
                    .body_from_file("tests/mocks/transcode/video_dash_h264_mp3.json");
            });

            part.create_streaming_session(
                Protocol::Dash,
                MusicTranscodeOptions {
                    bitrate: 192,
                    codecs: vec![AudioCodec::Mp3, AudioCodec::Vorbis],
                    limitations: vec![
                        (AudioSetting::Channels, Constraint::Max("2".to_string())).into()
                    ],
                    ..Default::default()
                },
            )
            .await
            .unwrap();
            m.assert();
            m.delete();
        }

        #[plex_api_test_helper::offline_test]
        async fn transcode_decision(#[future] server_authenticated: Mocked<Server>) {
            let (server, mock_server) = server_authenticated.split();

            let mut m = mock_server.mock(|when, then| {
                when.method(GET).path("/library/metadata/157786");
                then.status(200)
                    .header("content-type", "text/json")
                    .body_from_file("tests/mocks/transcode/metadata_157786.json");
            });

            let item: Track = server
                .item_by_id("157786")
                .await
                .unwrap()
                .try_into()
                .unwrap();
            m.assert();
            m.delete();

            let media = &item.media()[0];
            let part = &media.parts()[0];

            let mut m = mock_server.mock(|when, then| {
                when.method(GET)
                    .path("/video/:/transcode/universal/decision");
                then.status(200)
                    .header("content-type", "text/json")
                    .body_from_file("tests/mocks/transcode/music_mp3.json");
            });

            let session = part
                .create_streaming_session(Protocol::Dash, MusicTranscodeOptions::default())
                .await
                .unwrap();
            m.assert();
            m.delete();

            assert!(!session.is_offline());
            assert_eq!(session.container(), ContainerFormat::Mp4);
            assert_eq!(session.protocol(), Protocol::Dash);
            assert_eq!(
                session.audio_transcode(),
                Some((Decision::Transcode, AudioCodec::Mp3))
            );
            assert_eq!(session.video_transcode(), None);
        }
    }

    mod artwork {
        use super::*;
        use plex_api::{
            library::{MetadataItem, Movie},
            transcode::ArtTranscodeOptions,
        };

        #[plex_api_test_helper::offline_test]
        async fn transcode_art(#[future] server_authenticated: Mocked<Server>) {
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

            let mut m = mock_server.mock(|when, then| {
                when.method(GET)
                    .path("/photo/:/transcode")
                    .query_param("upscale", "1")
                    .query_param("minSize", "1")
                    .query_param("width", "1280")
                    .query_param("height", "1024")
                    .query_param("url", "/library/metadata/159637/thumb/1675330665");
                then.status(200)
                    .header("content-type", "image/jpeg")
                    // Doesn't make much difference what we return
                    .body("foo");
            });

            let mut buf = Vec::<u8>::new();
            server
                .transcode_artwork(
                    item.metadata().thumb.as_ref().unwrap(),
                    1280,
                    1024,
                    Default::default(),
                    &mut buf,
                )
                .await
                .unwrap();
            m.assert();
            m.delete();

            assert_eq!(std::str::from_utf8(&buf).unwrap(), "foo");

            let mut m = mock_server.mock(|when, then| {
                when.method(GET)
                    .path("/photo/:/transcode")
                    .query_param("upscale", "0")
                    .query_param("minSize", "0")
                    .query_param("width", "480")
                    .query_param("height", "320")
                    .query_param("url", "/library/metadata/159637/thumb/1675330665");
                then.status(200)
                    .header("content-type", "image/jpeg")
                    // Doesn't make much difference what we return
                    .body("foo");
            });

            let mut buf = Vec::<u8>::new();
            server
                .transcode_artwork(
                    item.metadata().thumb.as_ref().unwrap(),
                    480,
                    320,
                    ArtTranscodeOptions {
                        upscale: false,
                        min_size: false,
                    },
                    &mut buf,
                )
                .await
                .unwrap();
            m.assert();
            m.delete();

            assert_eq!(std::str::from_utf8(&buf).unwrap(), "foo");
        }
    }
}

mod online {
    use futures::Future;
    use plex_api::{
        media_container::server::library::{
            AudioCodec, ContainerFormat, Decision, Protocol, VideoCodec,
        },
        transcode::TranscodeSession,
        Server,
    };
    use std::time::Duration;
    use tokio::time::sleep;

    // Transcoding may crash the server in Linux container. It's unclear how or why,
    // but as a workaround, let's have an easy way of pausing a test while server's dead.
    macro_rules! ensure_server_alive {
        ($srv:ident) => {
            loop {
                let client = $srv.client().to_owned();
                if Server::new(client.api_url.clone(), client).await.is_ok() {
                    break;
                } else {
                    eprintln!("Server seems to be down, pausing for a while...");
                }
                sleep(Duration::from_secs(3)).await;
            }
        };
    }

    macro_rules! verify_no_sessions {
        ($srv:ident) => {
            #[cfg(not(feature = "tests_shared_server_access_token"))]
            {
                let sessions = $srv.transcode_sessions().await.unwrap();
                assert_eq!(sessions.len(), 0);
            }
        };
    }

    // Delays up to 5 seconds for the predicate to return true. Useful for
    // waiting on the server to complete some operation.
    #[cfg_attr(feature = "tests_shared_server_access_token", allow(dead_code))]
    async fn wait_for<C, F>(mut predicate: C)
    where
        C: FnMut() -> F,
        F: Future<Output = bool>,
    {
        for _ in 0..10 {
            if predicate().await {
                return;
            }

            sleep(Duration::from_millis(500)).await;
        }

        panic!("Timeout exceeded");
    }

    /// Checks the session was correct.
    async fn verify_session(
        session: &TranscodeSession,
        protocol: Protocol,
        container: ContainerFormat,
        audio: Option<(Decision, AudioCodec)>,
        video: Option<(Decision, VideoCodec)>,
        duration: Option<u64>,
    ) {
        assert_eq!(session.is_offline(), protocol == Protocol::Http);
        assert_eq!(session.protocol(), protocol);
        assert_eq!(session.container(), container);
        assert_eq!(session.audio_transcode(), audio);
        assert_eq!(session.video_transcode(), video);

        if let Some(duration) = duration {
            let stats = session.stats().await.unwrap();
            assert_eq!(stats.duration.unwrap(), duration);
        }
    }

    /// Checks the server lists a single session matching the one passed.
    #[cfg(not(feature = "tests_shared_server_access_token"))]
    async fn verify_remote_sessions(server: &Server, session: &TranscodeSession) {
        // It can take a few moments for the session to appear.
        wait_for(|| async {
            let sessions = server.transcode_sessions().await.unwrap();
            !sessions.is_empty()
        })
        .await;

        let sessions = server.transcode_sessions().await.unwrap();
        assert_eq!(sessions.len(), 1);

        let remote = &sessions[0];
        assert_eq!(remote.session_id(), session.session_id());
        assert_eq!(remote.is_offline(), session.is_offline());
        assert_eq!(remote.protocol(), session.protocol());
        assert_eq!(remote.container(), session.container());
        assert_eq!(remote.audio_transcode(), session.audio_transcode());
        assert_eq!(remote.video_transcode(), session.video_transcode());

        let remote = server
            .transcode_session(session.session_id())
            .await
            .unwrap();

        assert_eq!(remote.session_id(), session.session_id());
        assert_eq!(remote.is_offline(), session.is_offline());
        assert_eq!(remote.protocol(), session.protocol());
        assert_eq!(remote.container(), session.container());
        assert_eq!(remote.audio_transcode(), session.audio_transcode());
        assert_eq!(remote.video_transcode(), session.video_transcode());
    }

    /// Cancels the session and verifies it is gone from the server.
    #[cfg_attr(feature = "tests_shared_server_access_token", allow(unused_variables))]
    async fn cancel(server: &Server, session: TranscodeSession) {
        ensure_server_alive!(server);
        #[cfg(not(feature = "tests_shared_server_access_token"))]
        let existing = server
            .transcode_session(session.session_id())
            .await
            .unwrap();

        session.cancel().await.unwrap();

        // It can take a few moments for the session to disappear.
        #[cfg(not(feature = "tests_shared_server_access_token"))]
        {
            wait_for(|| async {
                ensure_server_alive!(server);
                let sessions = server.transcode_sessions().await.unwrap();
                sessions.is_empty()
            })
            .await;

            let err = existing.stats().await.unwrap_err();
            assert!(matches!(err, plex_api::Error::ItemNotFound));
        }

        ensure_server_alive!(server);
    }

    mod movie {
        use super::{super::fixtures::online::server::server, *};
        use hls_m3u8::{tags::VariantStream, MasterPlaylist, MediaPlaylist};
        use mp4::{AvcProfile, MediaType, Mp4Reader, TrackType};
        use plex_api::{
            library::{MediaItem, MediaItemWithTranscoding, MetadataItem, Movie},
            media_container::server::Feature,
            transcode::VideoTranscodeOptions,
        };
        use std::io::Cursor;

        #[plex_api_test_helper::online_test]
        async fn dash_transcode(
            #[future]
            #[with("Generic".to_owned())]
            server: Server,
        ) {
            let movie: Movie = server.item_by_id("55").await.unwrap().try_into().unwrap();
            assert_eq!(movie.title(), "Big Buck Bunny");

            let media = &movie.media()[0];
            let part = &media.parts()[0];

            let session = part
                .create_streaming_session(
                    Protocol::Dash,
                    // These settings will force transcoding as the original has
                    // higher bitrate and has a different audio codec.
                    VideoTranscodeOptions {
                        bitrate: 110,
                        video_codecs: vec![VideoCodec::H264],
                        audio_codecs: vec![AudioCodec::Mp3],
                        ..Default::default()
                    },
                )
                .await
                .unwrap();

            verify_session(
                &session,
                Protocol::Dash,
                ContainerFormat::Mp4,
                Some((Decision::Transcode, AudioCodec::Mp3)),
                Some((Decision::Transcode, VideoCodec::H264)),
                None,
            )
            .await;

            let mut buf: Vec<u8> = Vec::new();
            session.download(&mut buf).await.unwrap();
            let index = std::str::from_utf8(&buf).unwrap();
            assert!(dash_mpd::parse(index).is_ok());

            cancel(&server, session).await;

            verify_no_sessions!(server);
        }

        #[plex_api_test_helper::online_test]
        async fn dash_transcode_copy(
            #[future]
            #[with("Generic".to_owned())]
            server: Server,
        ) {
            let movie: Movie = server.item_by_id("57").await.unwrap().try_into().unwrap();
            assert_eq!(movie.title(), "Sintel");

            let media = &movie.media()[0];
            let part = &media.parts()[0];

            let session = part
                .create_streaming_session(
                    Protocol::Dash,
                    // These settings should allow for direct streaming of the video
                    // and audio.
                    VideoTranscodeOptions {
                        bitrate: 200000000,
                        width: 1280,
                        height: 720,
                        video_codecs: vec![VideoCodec::H264],
                        audio_codecs: vec![AudioCodec::Aac],
                        ..Default::default()
                    },
                )
                .await
                .unwrap();

            verify_session(
                &session,
                Protocol::Dash,
                ContainerFormat::Mp4,
                Some((Decision::Copy, AudioCodec::Aac)),
                Some((Decision::Copy, VideoCodec::H264)),
                None,
            )
            .await;

            let mut buf: Vec<u8> = Vec::new();
            session.download(&mut buf).await.unwrap();
            let index = std::str::from_utf8(&buf).unwrap();
            assert!(dash_mpd::parse(index).is_ok());

            cancel(&server, session).await;

            verify_no_sessions!(server);
        }

        #[plex_api_test_helper::online_test]
        async fn hls_transcode(
            #[future]
            #[with("Generic".to_owned())]
            server: Server,
        ) {
            let movie: Movie = server.item_by_id("55").await.unwrap().try_into().unwrap();
            assert_eq!(movie.title(), "Big Buck Bunny");

            let media = &movie.media()[0];
            assert_eq!(media.parts().len(), 2);
            let part = &media.parts()[0];

            // Tests transcoding a single part
            let session = part
                .create_streaming_session(
                    Protocol::Hls,
                    // These settings will force transcoding as the original has
                    // higher bitrate and has a different audio codec.
                    VideoTranscodeOptions {
                        bitrate: 110,
                        video_codecs: vec![VideoCodec::H264],
                        audio_codecs: vec![AudioCodec::Mp3],
                        ..Default::default()
                    },
                )
                .await
                .unwrap();

            verify_session(
                &session,
                Protocol::Hls,
                ContainerFormat::MpegTs,
                Some((Decision::Transcode, AudioCodec::Mp3)),
                Some((Decision::Transcode, VideoCodec::H264)),
                None,
            )
            .await;

            let mut buf = Vec::<u8>::new();
            session.download(&mut buf).await.unwrap();
            let index = std::str::from_utf8(&buf).unwrap();
            let playlist = MasterPlaylist::try_from(index).unwrap();
            if let VariantStream::ExtXStreamInf { uri, .. } = &playlist.variant_streams[0] {
                let path = format!("/video/:/transcode/universal/{uri}");
                let text = server
                    .client()
                    .get(path)
                    .send()
                    .await
                    .unwrap()
                    .text()
                    .await
                    .unwrap();

                let _media_playlist = MediaPlaylist::try_from(text.as_str()).unwrap();
            } else {
                panic!("Expected a media stream");
            }

            cancel(&server, session).await;

            verify_no_sessions!(server);

            // Tests transcoding a whole movie (multiple parts)
            let session = movie
                .create_streaming_session(
                    Protocol::Hls,
                    // These settings will force transcoding as the original has
                    // higher bitrate and has a different audio codec.
                    VideoTranscodeOptions {
                        bitrate: 110,
                        video_codecs: vec![VideoCodec::H264],
                        audio_codecs: vec![AudioCodec::Mp3],
                        ..Default::default()
                    },
                )
                .await
                .unwrap();

            verify_session(
                &session,
                Protocol::Hls,
                ContainerFormat::MpegTs,
                Some((Decision::Transcode, AudioCodec::Mp3)),
                Some((Decision::Transcode, VideoCodec::H264)),
                None,
            )
            .await;

            let mut buf = Vec::<u8>::new();
            session.download(&mut buf).await.unwrap();
            let index = std::str::from_utf8(&buf).unwrap();
            let playlist = MasterPlaylist::try_from(index).unwrap();
            if let VariantStream::ExtXStreamInf { uri, .. } = &playlist.variant_streams[0] {
                let path = format!("/video/:/transcode/universal/{uri}");
                let text = server
                    .client()
                    .get(path)
                    .send()
                    .await
                    .unwrap()
                    .text()
                    .await
                    .unwrap();

                let _media_playlist = MediaPlaylist::try_from(text.as_str()).unwrap();
            } else {
                panic!("Expected a media stream");
            }

            cancel(&server, session).await;

            verify_no_sessions!(server);
        }

        #[plex_api_test_helper::online_test]
        async fn hls_transcode_copy(
            #[future]
            #[with("Generic".to_owned())]
            server: Server,
        ) {
            let movie: Movie = server.item_by_id("55").await.unwrap().try_into().unwrap();
            assert_eq!(movie.title(), "Big Buck Bunny");

            let media = &movie.media()[0];
            assert_eq!(media.parts().len(), 2);
            let part = &media.parts()[0];

            // Tests transcoding a single part
            let session = part
                .create_streaming_session(
                    Protocol::Hls,
                    // These settings should allow for direct streaming of the video
                    // and audio.
                    VideoTranscodeOptions {
                        bitrate: 200000000,
                        width: 1280,
                        height: 720,
                        video_codecs: vec![VideoCodec::H264],
                        audio_codecs: vec![AudioCodec::Aac],
                        ..Default::default()
                    },
                )
                .await
                .unwrap();

            verify_session(
                &session,
                Protocol::Hls,
                ContainerFormat::MpegTs,
                Some((Decision::Copy, AudioCodec::Aac)),
                Some((Decision::Copy, VideoCodec::H264)),
                None,
            )
            .await;

            let mut buf = Vec::<u8>::new();
            session.download(&mut buf).await.unwrap();
            let index = std::str::from_utf8(&buf).unwrap();
            let playlist = MasterPlaylist::try_from(index).unwrap();
            if let VariantStream::ExtXStreamInf { uri, .. } = &playlist.variant_streams[0] {
                let path = format!("/video/:/transcode/universal/{uri}");
                let text = server
                    .client()
                    .get(path)
                    .send()
                    .await
                    .unwrap()
                    .text()
                    .await
                    .unwrap();

                let _media_playlist = MediaPlaylist::try_from(text.as_str()).unwrap();
            } else {
                panic!("Expected a media stream");
            }

            cancel(&server, session).await;

            verify_no_sessions!(server);

            // Tests transcoding a whole movie (multiple parts)
            let session = movie
                .create_streaming_session(
                    Protocol::Hls,
                    // These settings should allow for direct streaming of the video
                    // and audio.
                    VideoTranscodeOptions {
                        bitrate: 200000000,
                        width: 1280,
                        height: 720,
                        video_codecs: vec![VideoCodec::H264],
                        audio_codecs: vec![AudioCodec::Aac],
                        ..Default::default()
                    },
                )
                .await
                .unwrap();

            verify_session(
                &session,
                Protocol::Hls,
                ContainerFormat::MpegTs,
                // Despite being able to play the original parts directly the server will still
                // transcode in order to combine the parts into a single stream.
                Some((Decision::Transcode, AudioCodec::Aac)),
                Some((Decision::Transcode, VideoCodec::H264)),
                None,
            )
            .await;

            let mut buf = Vec::<u8>::new();
            session.download(&mut buf).await.unwrap();
            let index = std::str::from_utf8(&buf).unwrap();
            let playlist = MasterPlaylist::try_from(index).unwrap();
            if let VariantStream::ExtXStreamInf { uri, .. } = &playlist.variant_streams[0] {
                let path = format!("/video/:/transcode/universal/{uri}");
                let text = server
                    .client()
                    .get(path)
                    .send()
                    .await
                    .unwrap()
                    .text()
                    .await
                    .unwrap();

                let _media_playlist = MediaPlaylist::try_from(text.as_str()).unwrap();
            } else {
                panic!("Expected a media stream");
            }

            cancel(&server, session).await;

            verify_no_sessions!(server);
        }

        #[plex_api_test_helper::online_test_non_shared_server]
        async fn check_unknown_transcoding_session_response(#[future] server: Server) {
            let error = server
                .transcode_session("gfbrgbrbrfber")
                .await
                .err()
                .unwrap();

            assert!(matches!(error, plex_api::Error::ItemNotFound));
        }

        #[plex_api_test_helper::online_test_claimed_server]
        async fn offline_transcode(
            #[future]
            #[with("Generic".to_owned())]
            server: Server,
        ) {
            if !server
                .media_container
                .owner_features
                .contains(&Feature::SyncV3)
            {
                // Offline transcoding is only supported with a subscription.
                return;
            }

            let movie: Movie = server.item_by_id("57").await.unwrap().try_into().unwrap();
            assert_eq!(movie.title(), "Sintel");

            let media = &movie.media()[0];
            let part = &media.parts()[0];

            let session = part
                .create_download_session(
                    // These settings will force transcoding as the original has
                    // higher bitrate and has a different audio codec.
                    VideoTranscodeOptions {
                        bitrate: 110,
                        containers: vec![ContainerFormat::Mp4],
                        video_codecs: vec![VideoCodec::H264],
                        audio_codecs: vec![AudioCodec::Mp3],
                        ..Default::default()
                    },
                )
                .await
                .unwrap();

            verify_session(
                &session,
                Protocol::Http,
                ContainerFormat::Mp4,
                Some((Decision::Transcode, AudioCodec::Mp3)),
                Some((Decision::Transcode, VideoCodec::H264)),
                part.duration(),
            )
            .await;

            #[cfg(not(feature = "tests_shared_server_access_token"))]
            verify_remote_sessions(&server, &session).await;

            cancel(&server, session).await;

            verify_no_sessions!(server);

            let movie: Movie = server.item_by_id("55").await.unwrap().try_into().unwrap();
            assert_eq!(movie.title(), "Big Buck Bunny");

            let media = &movie.media()[0];
            assert_eq!(media.parts().len(), 2);

            let session = movie
                .create_download_session(
                    // These settings will force transcoding as the original has
                    // higher bitrate and has a different audio codec.
                    VideoTranscodeOptions {
                        bitrate: 110,
                        containers: vec![ContainerFormat::Mp4],
                        video_codecs: vec![VideoCodec::H264],
                        audio_codecs: vec![AudioCodec::Mp3],
                        ..Default::default()
                    },
                )
                .await
                .unwrap();

            verify_session(
                &session,
                Protocol::Http,
                ContainerFormat::Mp4,
                Some((Decision::Transcode, AudioCodec::Mp3)),
                Some((Decision::Transcode, VideoCodec::H264)),
                // There is only one media item so we know this will be the duration of the whole movie.
                media.duration(),
            )
            .await;

            #[cfg(not(feature = "tests_shared_server_access_token"))]
            verify_remote_sessions(&server, &session).await;

            cancel(&server, session).await;

            verify_no_sessions!(server);
        }

        #[plex_api_test_helper::online_test_claimed_server]
        async fn offline_transcode_copy(
            #[future]
            #[with("Generic".to_owned())]
            server: Server,
        ) {
            if !server
                .media_container
                .owner_features
                .contains(&Feature::SyncV3)
            {
                // Offline transcoding is only supported with a subscription.
                return;
            }

            let movie: Movie = server.item_by_id("57").await.unwrap().try_into().unwrap();
            assert_eq!(movie.title(), "Sintel");

            let media = &movie.media()[0];
            let part = &media.parts()[0];

            let session = part
                .create_download_session(
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
                )
                .await
                .unwrap();

            verify_session(
                &session,
                Protocol::Http,
                ContainerFormat::Mp4,
                Some((Decision::Copy, AudioCodec::Aac)),
                Some((Decision::Copy, VideoCodec::H264)),
                part.duration(),
            )
            .await;

            #[cfg(not(feature = "tests_shared_server_access_token"))]
            verify_remote_sessions(&server, &session).await;

            // As this transcode is just copying the existing streams into a new
            // container format it should complete quickly allowing us to download
            // the transcoded file.

            // To avoid download timeouts wait for the transcode to complete.
            loop {
                let stats = session.stats().await.unwrap();
                if stats.complete {
                    break;
                }
                sleep(Duration::from_millis(250)).await;
            }

            let mut buf = Vec::<u8>::new();
            session.download(&mut buf).await.unwrap();
            cancel(&server, session).await;

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

            verify_no_sessions!(server);

            let movie: Movie = server.item_by_id("55").await.unwrap().try_into().unwrap();
            assert_eq!(movie.title(), "Big Buck Bunny");

            let media = &movie.media()[0];
            assert_eq!(media.parts().len(), 2);

            let session = movie
                .create_download_session(
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
                )
                .await
                .unwrap();

            verify_session(
                &session,
                Protocol::Http,
                ContainerFormat::Mp4,
                // The server will need to transcode in order to combine the parts
                // into a single stream.
                Some((Decision::Transcode, AudioCodec::Aac)),
                Some((Decision::Transcode, VideoCodec::H264)),
                media.duration(),
            )
            .await;

            #[cfg(not(feature = "tests_shared_server_access_token"))]
            verify_remote_sessions(&server, &session).await;

            // As this transcode is just copying the existing streams into a new
            // container format it should complete quickly allowing us to download
            // the transcoded file.

            // To avoid download timeouts wait for the transcode to complete.
            loop {
                let stats = session.stats().await.unwrap();
                if stats.complete {
                    break;
                }
                sleep(Duration::from_millis(250)).await;
            }

            let mut buf = Vec::<u8>::new();
            session.download(&mut buf).await.unwrap();
            cancel(&server, session).await;

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

            verify_no_sessions!(server);
        }

        #[plex_api_test_helper::online_test_claimed_server]
        async fn offline_transcode_denied(
            #[future]
            #[with("Generic".to_owned())]
            server: Server,
        ) {
            if !server
                .media_container
                .owner_features
                .contains(&Feature::SyncV3)
            {
                // Offline transcoding is only supported with a subscription.
                return;
            }

            let movie: Movie = server.item_by_id("57").await.unwrap().try_into().unwrap();
            assert_eq!(movie.title(), "Sintel");

            let media = &movie.media()[0];
            let part = &media.parts()[0];

            let error = part
                .create_download_session(
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
                )
                .await
                .err()
                .unwrap();

            assert!(matches!(error, plex_api::Error::TranscodeRefused));
        }
    }

    mod music {
        use super::{super::fixtures::online::server::server, *};
        use hls_m3u8::{tags::VariantStream, MasterPlaylist, MediaPlaylist};
        use plex_api::{
            library::{MediaItem, MediaItemWithTranscoding, MetadataItem, Track},
            media_container::server::Feature,
            transcode::MusicTranscodeOptions,
        };

        #[plex_api_test_helper::online_test]
        async fn dash_transcode(
            #[future]
            #[with("Generic".to_owned())]
            server: Server,
        ) {
            let track: Track = server.item_by_id("158").await.unwrap().try_into().unwrap();
            assert_eq!(track.title(), "Try It Out (Neon mix)");

            let media = &track.media()[0];
            let part = &media.parts()[0];

            let session = part
                .create_streaming_session(
                    Protocol::Dash,
                    // These settings will force transcoding as the original is too
                    // high a bitrate and has a different audio codec.
                    MusicTranscodeOptions {
                        bitrate: 92,
                        codecs: vec![AudioCodec::Mp3],
                        ..Default::default()
                    },
                )
                .await
                .unwrap();

            verify_session(
                &session,
                Protocol::Dash,
                ContainerFormat::Mp4,
                Some((Decision::Transcode, AudioCodec::Mp3)),
                None,
                None,
            )
            .await;

            let mut buf: Vec<u8> = Vec::new();
            session.download(&mut buf).await.unwrap();
            let index = std::str::from_utf8(&buf).unwrap();
            assert!(dash_mpd::parse(index).is_ok());

            cancel(&server, session).await;

            verify_no_sessions!(server);
        }

        #[plex_api_test_helper::online_test]
        async fn dash_transcode_copy(
            #[future]
            #[with("Generic".to_owned())]
            server: Server,
        ) {
            let track: Track = server.item_by_id("158").await.unwrap().try_into().unwrap();
            assert_eq!(track.title(), "Try It Out (Neon mix)");

            let media = &track.media()[0];
            let part = &media.parts()[0];

            let session = part
                .create_streaming_session(
                    Protocol::Dash,
                    // These settings should allow for direct streaming of the music.
                    MusicTranscodeOptions {
                        bitrate: 256000,
                        codecs: vec![AudioCodec::Aac],
                        ..Default::default()
                    },
                )
                .await
                .unwrap();

            verify_session(
                &session,
                Protocol::Dash,
                ContainerFormat::Mp4,
                Some((Decision::Copy, AudioCodec::Aac)),
                None,
                None,
            )
            .await;

            let mut buf: Vec<u8> = Vec::new();
            session.download(&mut buf).await.unwrap();
            let index = std::str::from_utf8(&buf).unwrap();
            assert!(dash_mpd::parse(index).is_ok());

            cancel(&server, session).await;

            verify_no_sessions!(server);
        }

        #[plex_api_test_helper::online_test]
        async fn hls_transcode(
            #[future]
            #[with("Generic".to_owned())]
            server: Server,
        ) {
            let track: Track = server.item_by_id("158").await.unwrap().try_into().unwrap();
            assert_eq!(track.title(), "Try It Out (Neon mix)");

            let media = &track.media()[0];
            let part = &media.parts()[0];

            let session = part
                .create_streaming_session(
                    Protocol::Hls,
                    // These settings will force transcoding as the original is too
                    // high a bitrate and has a different audio codec.
                    MusicTranscodeOptions {
                        bitrate: 92,
                        codecs: vec![AudioCodec::Mp3],
                        ..Default::default()
                    },
                )
                .await
                .unwrap();

            verify_session(
                &session,
                Protocol::Hls,
                ContainerFormat::MpegTs,
                Some((Decision::Transcode, AudioCodec::Mp3)),
                None,
                None,
            )
            .await;

            let mut buf = Vec::<u8>::new();
            session.download(&mut buf).await.unwrap();
            let index = std::str::from_utf8(&buf).unwrap();
            let playlist = MasterPlaylist::try_from(index).unwrap();
            if let VariantStream::ExtXStreamInf { uri, .. } = &playlist.variant_streams[0] {
                let path = format!("/video/:/transcode/universal/{uri}");
                let text = server
                    .client()
                    .get(path)
                    .send()
                    .await
                    .unwrap()
                    .text()
                    .await
                    .unwrap();

                let _media_playlist = MediaPlaylist::try_from(text.as_str()).unwrap();
            } else {
                panic!("Expected a media stream");
            }

            cancel(&server, session).await;

            verify_no_sessions!(server);
        }

        #[plex_api_test_helper::online_test]
        async fn hls_transcode_copy(
            #[future]
            #[with("Generic".to_owned())]
            server: Server,
        ) {
            let track: Track = server.item_by_id("158").await.unwrap().try_into().unwrap();
            assert_eq!(track.title(), "Try It Out (Neon mix)");

            let media = &track.media()[0];
            let part = &media.parts()[0];

            verify_no_sessions!(server);

            let session = part
                .create_streaming_session(
                    Protocol::Hls,
                    // These settings should allow for direct streaming of the music.
                    MusicTranscodeOptions {
                        bitrate: 256000,
                        codecs: vec![AudioCodec::Aac],
                        ..Default::default()
                    },
                )
                .await
                .unwrap();

            verify_session(
                &session,
                Protocol::Hls,
                ContainerFormat::MpegTs,
                Some((Decision::Copy, AudioCodec::Aac)),
                None,
                None,
            )
            .await;

            let mut buf = Vec::<u8>::new();
            session.download(&mut buf).await.unwrap();
            let index = std::str::from_utf8(&buf).unwrap();
            let playlist = MasterPlaylist::try_from(index).unwrap();
            if let VariantStream::ExtXStreamInf { uri, .. } = &playlist.variant_streams[0] {
                let path = format!("/video/:/transcode/universal/{uri}");
                let text = server
                    .client()
                    .get(path)
                    .send()
                    .await
                    .unwrap()
                    .text()
                    .await
                    .unwrap();

                let _media_playlist = MediaPlaylist::try_from(text.as_str()).unwrap();
            } else {
                panic!("Expected a media stream");
            }

            cancel(&server, session).await;

            verify_no_sessions!(server);
        }

        #[plex_api_test_helper::online_test_claimed_server]
        async fn offline_transcode(
            #[future]
            #[with("Generic".to_owned())]
            server: Server,
        ) {
            if !server
                .media_container
                .owner_features
                .contains(&Feature::SyncV3)
            {
                // Offline transcoding is only supported with a subscription.
                return;
            }

            let track: Track = server.item_by_id("158").await.unwrap().try_into().unwrap();
            assert_eq!(track.title(), "Try It Out (Neon mix)");

            let media = &track.media()[0];
            let part = &media.parts()[0];

            verify_no_sessions!(server);

            let session = part
                .create_download_session(
                    // These settings will force transcoding as the original is too
                    // high a bitrate and has a different audio codec.
                    MusicTranscodeOptions {
                        bitrate: 92,
                        containers: vec![ContainerFormat::Mp3],
                        codecs: vec![AudioCodec::Mp3],
                        ..Default::default()
                    },
                )
                .await
                .unwrap();

            verify_session(
                &session,
                Protocol::Http,
                ContainerFormat::Mp3,
                Some((Decision::Transcode, AudioCodec::Mp3)),
                None,
                None,
            )
            .await;

            #[cfg(not(feature = "tests_shared_server_access_token"))]
            verify_remote_sessions(&server, &session).await;

            // Audio transcoding should be reasonably fast...

            // To avoid download timeouts wait for the transcode to complete.
            loop {
                let stats = session.stats().await.unwrap();
                if stats.complete {
                    break;
                }
                sleep(Duration::from_millis(250)).await;
            }

            let mut buf = Vec::<u8>::new();
            session.download(&mut buf).await.unwrap();

            // Check a few unlikely to change properties about the stream.
            let metadata = mp3_metadata::read_from_slice(&buf).unwrap();
            assert_eq!(metadata.duration.as_secs(), 5);
            let frame = metadata.frames.first().unwrap();
            assert_eq!(frame.layer, mp3_metadata::Layer::Layer3);
            assert_eq!(frame.chan_type, mp3_metadata::ChannelType::SingleChannel);

            cancel(&server, session).await;

            verify_no_sessions!(server);
        }

        #[plex_api_test_helper::online_test_claimed_server]
        async fn offline_transcode_denied(
            #[future]
            #[with("Generic".to_owned())]
            server: Server,
        ) {
            if !server
                .media_container
                .owner_features
                .contains(&Feature::SyncV3)
            {
                // Offline transcoding is only supported with a subscription.
                return;
            }

            let track: Track = server.item_by_id("158").await.unwrap().try_into().unwrap();
            assert_eq!(track.title(), "Try It Out (Neon mix)");

            let media = &track.media()[0];
            let part = &media.parts()[0];

            let error = part
                .create_download_session(
                    // Here we ask to transcode into a format the music is already
                    // in so the server denies the request.
                    MusicTranscodeOptions {
                        bitrate: 200000000,
                        containers: vec![ContainerFormat::Aac],
                        codecs: vec![AudioCodec::Aac],
                        ..Default::default()
                    },
                )
                .await
                .err()
                .unwrap();

            assert!(matches!(error, plex_api::Error::TranscodeRefused));
        }
    }

    mod artwork {
        use super::super::fixtures::online::server::server;
        use image::ImageReader;
        use plex_api::{
            library::MetadataItem, library::Movie, transcode::ArtTranscodeOptions, Server,
        };
        use std::io::Cursor;

        #[plex_api_test_helper::online_test]
        async fn transcode_art(
            #[future]
            #[with("Generic".to_owned())]
            server: Server,
        ) {
            let movie: Movie = server.item_by_id("55").await.unwrap().try_into().unwrap();
            assert_eq!(movie.title(), "Big Buck Bunny");

            let mut buf = Vec::<u8>::new();
            server
                .transcode_artwork(
                    movie.metadata().thumb.as_ref().unwrap(),
                    10000,
                    10000,
                    ArtTranscodeOptions {
                        upscale: false,
                        min_size: true,
                    },
                    &mut buf,
                )
                .await
                .unwrap();

            let img = ImageReader::new(Cursor::new(buf))
                .with_guessed_format()
                .unwrap()
                .decode()
                .unwrap();
            // Default size seems to be 1000x1500
            assert_eq!(img.width(), 1000);
            assert_eq!(img.height(), 1500);

            let mut buf = Vec::<u8>::new();
            server
                .transcode_artwork(
                    movie.metadata().thumb.as_ref().unwrap(),
                    900,
                    900,
                    ArtTranscodeOptions {
                        upscale: false,
                        min_size: true,
                    },
                    &mut buf,
                )
                .await
                .unwrap();

            let img = ImageReader::new(Cursor::new(buf))
                .with_guessed_format()
                .unwrap()
                .decode()
                .unwrap();
            // Image must be at least 900x900
            assert_eq!(img.width(), 900);
            assert_eq!(img.height(), 1350);

            let mut buf = Vec::<u8>::new();
            server
                .transcode_artwork(
                    movie.metadata().thumb.as_ref().unwrap(),
                    900,
                    900,
                    ArtTranscodeOptions {
                        upscale: false,
                        min_size: false,
                    },
                    &mut buf,
                )
                .await
                .unwrap();

            let img = ImageReader::new(Cursor::new(buf))
                .with_guessed_format()
                .unwrap()
                .decode()
                .unwrap();
            // Image must be at most 900x900
            assert_eq!(img.width(), 600);
            assert_eq!(img.height(), 900);

            let mut buf = Vec::<u8>::new();
            server
                .transcode_artwork(
                    movie.metadata().thumb.as_ref().unwrap(),
                    3000,
                    3000,
                    ArtTranscodeOptions {
                        upscale: true,
                        min_size: false,
                    },
                    &mut buf,
                )
                .await
                .unwrap();

            let img = ImageReader::new(Cursor::new(buf))
                .with_guessed_format()
                .unwrap()
                .decode()
                .unwrap();
            assert_eq!(img.width(), 2000);
            assert_eq!(img.height(), 3000);
        }
    }
}
