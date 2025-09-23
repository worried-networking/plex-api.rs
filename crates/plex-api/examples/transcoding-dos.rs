//! This script is used to investigate issue #541.
//! It creates and cancels a lot of transcoding sessions, to crash the Plex server.

use plex_api::{
    device::DeviceConnection,
    library::{MediaItem, Movie, Transcodable},
    media_container::server::library::{AudioCodec, VideoCodec},
    transcode::{TranscodeSession, VideoTranscodeOptions},
    HttpClientBuilder, MyPlex, MyPlexBuilder, Server,
};
use std::{error::Error, time::Duration};
use tokio::time::sleep;
use tracing::{debug, error, info, trace};

struct AppArgs {
    pub server: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub token: Option<String>,
    pub client_id: Option<String>,
    pub item_id: Option<String>,
    pub delay_after_transcoding_start: u8,
    pub delay_after_transcoding_cancel: u8,
}

const HELP: &str = "\
Transcoding DoS

USAGE:
  cargo run --example transcoding-dos -- [OPTIONS]

FLAGS:
  -h, --help            Prints help information

OPTIONS:
  --server URL          Sets the server's url
  --token STRING        Sets the authentication token (preferred over username/password)
  --username STRING     Sets the username for MyPlex
  --password STRING     Sets the password for MyPlex
  --client-id STRING    Sets the client id to use
  --item-id STRING      Sets the item id from the server to use
  --delay-after-transcoding-start   Sets the delay after starting the transcoding, seconds (default 0)
  --delay-after-transcoding-cancel  Sets the delay after cancelling the transcoding, seconds (default 0)
";

fn parse_args() -> Result<AppArgs, pico_args::Error> {
    let mut pargs = pico_args::Arguments::from_env();

    // Help has a higher priority and should be handled separately.
    if pargs.contains(["-h", "--help"]) {
        print!("{}", HELP);
        std::process::exit(0);
    }

    let args = AppArgs {
        server: pargs.opt_value_from_str("--server")?,
        username: pargs.opt_value_from_str("--username")?,
        password: pargs.opt_value_from_str("--password")?,
        token: pargs.opt_value_from_str("--token")?,
        client_id: pargs.opt_value_from_str("--client-id")?,
        item_id: pargs.opt_value_from_str("--item-id")?,
        delay_after_transcoding_start: pargs
            .opt_value_from_str("--delay-after-transcoding-start")?
            .unwrap_or(0),
        delay_after_transcoding_cancel: pargs
            .opt_value_from_str("--delay-after-transcoding-cancel")?
            .unwrap_or(0),
    };

    // It's up to the caller what to do with the remaining arguments.
    let remaining = pargs.finish();
    if !remaining.is_empty() {
        panic!("Warning: unused arguments left: {:?}.", remaining);
    }

    Ok(args)
}

async fn find_server(myplex: &MyPlex) -> Result<Option<Server>, Box<dyn Error>> {
    for device in myplex.device_manager()?.devices().await? {
        if !device.is_server() {
            continue;
        }
        info!("Connecting to {}...", device.name());
        match device.connect().await {
            Ok(DeviceConnection::Server(server)) => {
                info!("Connected via {}", server.client().api_url);
                return Ok(Some(*server));
            }
            Err(e) => {
                error!("Error connecting: {e}");
            }
            _ => {}
        }
    }
    Ok(None)
}

async fn cancel_transcodes(server: &Server) -> Result<(), Box<dyn Error>> {
    for session in server.transcode_sessions().await? {
        info!("Cancelling {}", session.session_id());
        if let Err(e) = session.cancel().await {
            error!("{e}");
        }
    }
    Ok(())
}

async fn do_transcode(id: &str, server: Server) -> Option<TranscodeSession> {
    trace!("Starting transcode {id}");
    let item: Movie = server.item_by_id(id).await.unwrap().try_into().unwrap();
    let media = &item.media()[0];
    let part = &media.parts()[0];
    trace!("Got item {id}");
    let session = part
        .create_download_session(VideoTranscodeOptions {
            bitrate: 110,
            video_codecs: vec![VideoCodec::H264],
            audio_codecs: vec![AudioCodec::Mp3],
            ..Default::default()
        })
        .await
        .unwrap();
    debug!("Got session {id} {}", session.session_id());
    // Wait until the transcode session has started.
    let mut count = 0;
    loop {
        match session.stats().await {
            Ok(_) => {
                info!("Session started {id}");
                break;
            }
            Err(plex_api::Error::ItemNotFound) => {
                count += 1;
                if count > 10 {
                    error!("Transcode failed to start");
                    return None;
                }
            }
            Err(e) => panic!("Transcode failed: {e}"),
        }
        sleep(Duration::from_millis(200)).await;
    }
    Some(session)
}

async fn punish(server: &Server, args: AppArgs) -> Result<(), Box<dyn Error>> {
    let item_id = args.item_id.as_ref().expect("item-id must be provided");
    loop {
        for id in item_id.split(',') {
            if let Some(session) = do_transcode(id, server.clone()).await {
                sleep(Duration::from_secs(
                    args.delay_after_transcoding_start as u64,
                ))
                .await;
                info!("Cancelling {}", session.session_id());
                session.cancel().await.unwrap();
            }
            sleep(Duration::from_secs(
                args.delay_after_transcoding_cancel as u64,
            ))
            .await;
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();

    let args = parse_args()?;
    let mut client_builder = HttpClientBuilder::generic();
    if let Some(token) = &args.token {
        client_builder = client_builder.set_x_plex_token(token.to_owned());
    }
    if let Some(client_id) = &args.client_id {
        client_builder = client_builder.set_x_plex_client_identifier(client_id);
    }

    let client = client_builder.build()?;
    let server = if let Some(server) = &args.server {
        Server::new(server, client).await?
    } else if let (Some(username), Some(password)) = (&args.username, &args.password) {
        let myplex = MyPlexBuilder::default()
            .set_client(client)
            .set_username_and_password(username, password.to_owned())
            .build()
            .await?;
        debug!("Token: {}", myplex.client().x_plex_token());
        find_server(&myplex)
            .await?
            .expect("unable to find a server")
    } else if client.is_authenticated() {
        let myplex = MyPlexBuilder::default().set_client(client).build().await?;
        find_server(&myplex)
            .await?
            .expect("unable to find a server")
    } else {
        panic!("Must provide a server URL or plex username and password.");
    };
    cancel_transcodes(&server).await?;
    punish(&server, args).await?;
    Ok(())
}
