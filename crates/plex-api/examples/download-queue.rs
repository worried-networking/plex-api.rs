use plex_api::{
    device::Device,
    library::{Item, MetadataItem, Transcodable},
    media_container::server::library::{AudioCodec, ContainerFormat, VideoCodec},
    transcode::{QueueItemStatus, VideoTranscodeOptions},
    HttpClientBuilder, MyPlexBuilder,
};
use rpassword::prompt_password;
use std::{
    io::{stdin, stdout, BufRead, Write},
    path::PathBuf,
};
use tokio::{
    fs::OpenOptions,
    io::BufWriter,
    time::{sleep, Duration},
};
use tokio_util::compat::TokioAsyncReadCompatExt;

async fn download<M>(media: M)
where
    M: Transcodable<Options = VideoTranscodeOptions> + MetadataItem,
{
    let mut entry = media
        .queue_download(
            VideoTranscodeOptions {
                bitrate: 4000,
                width: 1280,
                height: 720,
                containers: vec![ContainerFormat::Mp4],
                video_codecs: vec![VideoCodec::H264],
                audio_codecs: vec![AudioCodec::Ac3],
                ..Default::default()
            },
            None,
        )
        .await
        .unwrap();

    loop {
        match entry.status() {
            QueueItemStatus::Deciding | QueueItemStatus::Waiting => {
                sleep(Duration::from_millis(200)).await;
                entry.update().await.unwrap();
            }
            QueueItemStatus::Processing => {
                sleep(Duration::from_millis(1000)).await;
                entry.update().await.unwrap();
            }
            QueueItemStatus::Available => {
                println!("\nDownload available!:\n{:#?}\n", entry);
                break;
            }
            QueueItemStatus::Error => {
                println!("\nTranscode errored\n");
                break;
            }
            QueueItemStatus::Expired => {
                println!("\nTranscode expired\n");
                break;
            }
        }
    }

    let target = PathBuf::from(format!(
        "{}.{}",
        media.metadata().title,
        entry.container().await.unwrap()
    ));

    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(&target)
        .await
        .unwrap();

    let mut writer = BufWriter::new(file).compat();

    entry.download(&mut writer, ..).await.unwrap();

    eprintln!("Item {}: Downloaded", media.metadata().key);

    entry.delete().await.unwrap();
}

#[tokio::main]
async fn main() {
    let mut pargs = pico_args::Arguments::from_env();

    let token = std::env::var("PLEX_API_AUTH_TOKEN")
        .unwrap_or_else(|_| prompt_password("Token: ").unwrap());
    stdout().flush().unwrap();

    let client_identifier = pargs
        .opt_value_from_str("--client-id")
        .unwrap()
        .unwrap_or_else(|| "dqueueexample".to_string());

    let myplex = MyPlexBuilder::default()
        .set_client(
            HttpClientBuilder::generic()
                .set_x_plex_client_identifier(&client_identifier)
                .build()
                .unwrap(),
        )
        .set_token(token)
        .build()
        .await
        .unwrap();

    let device_manager = myplex.device_manager().unwrap();
    let devices = device_manager.devices().await.unwrap();

    if devices.is_empty() {
        eprintln!("You have no devices");
        return;
    }

    let devices = devices
        .into_iter()
        .filter(|device| device.is_server())
        .collect::<Vec<Device>>();

    let device = {
        if devices.len() == 1 {
            devices.first().unwrap()
        } else {
            println!("Please selected the server you want to connect to:");
            for (idx, d) in devices.iter().enumerate() {
                println!("{}. {}", idx + 1, d.name());
            }
            let idx = stdin().lock().lines().next().unwrap().unwrap();
            let idx: usize = idx.parse().unwrap();
            if idx > devices.len() + 1 || idx < 1 {
                eprintln!("Don't be like that");
                return;
            }
            devices.get(idx - 1).unwrap()
        }
    };

    let server = match device.connect().await.unwrap() {
        plex_api::device::DeviceConnection::Server(srv) => srv,
        _ => panic!("HOW?"),
    };

    let queue = server.download_queue().await.unwrap();

    if pargs.contains("--clear") {
        for item in queue.items().await.unwrap() {
            item.delete().await.unwrap();
        }
    }

    eprintln!("Current queue: {:#?}", queue.items().await.unwrap());

    print!("Item ID: ");
    stdout().flush().unwrap();

    let identifier = stdin().lock().lines().next().unwrap().unwrap();

    let item = server.item_by_id(&identifier).await.unwrap();

    match item {
        Item::Movie(m) => {
            download(m).await;
        }
        Item::Episode(e) => {
            download(e).await;
        }
        _ => eprintln!("Unsupported item type"),
    }
}
