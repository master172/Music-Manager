use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::thread;

pub fn download_audio(url: String, output_path: String) {
    thread::spawn(move || {
        let output_dir = playlist_output_path(&output_path);
        let output_template = output_dir.join("%(title)s.%(ext)s");
        let output_template_str = output_template.to_str().unwrap();

        let yt_dlp_path = Path::new("libs/yt-dlp.exe");
        let ffmpeg_path = Path::new("libs/ffmpeg.exe");

        let status = Command::new(yt_dlp_path)
            .args(["-f", "bestaudio", "-o", output_template_str, &url])
            .status()
            .expect("Failed to run yt-dlp.exe");

        if !status.success() {
            eprintln!("yt-dlp failed for URL: {}", url);
            return;
        }

        let downloaded_file = fs::read_dir(&output_dir)
            .unwrap()
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .find(|p| {
                p.is_file()
                    && matches!(
                        p.extension().and_then(|ext| ext.to_str()).unwrap_or(""),
                        "webm" | "m4a" | "opus"
                    )
            });

        let downloaded_file = match downloaded_file {
            Some(f) => f,
            None => {
                eprintln!("No audio file found for URL: {}", url);
                return;
            }
        };

        let mut mp3_file = downloaded_file.clone();
        mp3_file.set_extension("mp3");

        let status = Command::new(ffmpeg_path)
            .args([
                "-i",
                downloaded_file.to_str().unwrap(),
                "-vn",
                "-ab",
                "192k",
                "-ar",
                "44100",
                "-y",
                mp3_file.to_str().unwrap(),
            ])
            .status()
            .expect("Failed to run ffmpeg");

        if !status.success() {
            eprintln!(
                "FFmpeg failed to convert {} to mp3",
                downloaded_file.display()
            );
        } else {
            println!("Downloaded and converted: {}", mp3_file.display());
        }

        let _ = fs::remove_file(downloaded_file);
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
