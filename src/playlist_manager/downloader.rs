use std::fs;
use std::path::PathBuf;
use tokio::runtime::Runtime;
use yt_dlp::Downloader;
use yt_dlp::client::deps::Libraries;

pub fn download_audio(url: String, output_path: String, name: String) {
    std::thread::spawn(move || {
        let rt = Runtime::new().unwrap();
        rt.block_on(async move {
            let libraries_dir = PathBuf::from("libs");

            let youtube = libraries_dir.join("yt-dlp");
            let ffmpeg = libraries_dir.join("ffmpeg");

            let libraries = Libraries::new(youtube, ffmpeg);
            let output_dir = playlist_output_path(&output_path);

            let file_name = format!("{}.mp3", name);
            //let path: String = String::from(url);

            match Downloader::new(libraries, output_dir).await {
                Ok(downloader) => {
                    if let Err(e) = downloader
                        .download_audio_stream_from_url(url, file_name)
                        .await
                    {
                        eprintln!("Error downloading audio: {}", e);
                    } else {
                        println!("Audio downloaded successfully");
                    }
                }
                Err(err) => {
                    eprintln!("Error initializing downloader: {}", err);
                }
            }
        });
    });
}

fn playlist_output_path(playlist_name: &str) -> PathBuf {
    let cwd = std::env::current_dir().expect("Failed to get current working directory");

    let folder_path = cwd.join("playlists").join(playlist_name);

    println!("Folder path: {}", folder_path.display());
    if !folder_path.exists() {
        fs::create_dir_all(&folder_path).expect("failed to create destination folder");
    }

    folder_path
}
