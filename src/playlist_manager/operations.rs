use std::fs;
use std::path::PathBuf;
use std::thread;

use yt_dlp::utils::Platform;

pub fn list_playlist_files_async(
    playlist_name: String,
    callback: impl Fn(Vec<PathBuf>) + Send + 'static,
) {
    thread::spawn(move || {
        let folder_path = std::env::current_dir()
            .unwrap()
            .join("playlists")
            .join(&playlist_name);

        let mut files = vec![];

        if folder_path.exists() {
            files = fs::read_dir(&folder_path)
                .unwrap()
                .filter_map(|entry| entry.ok())
                .map(|entry| entry.path())
                .filter(|p| p.is_file())
                .collect();
        }

        callback(files);
    });
}

pub fn list(playlist_name: &str) {
    list_playlist_files_async(playlist_name.into(), |files| {
        println!("Files in playlist:");
        for f in files {
            println!("{}", f.display());
        }
    });
}

pub fn delete_file_from_playlist(playlist_name: &str, file_name: &str) -> std::io::Result<()> {
    let folder_path = std::env::current_dir()?
        .join("playlists")
        .join(playlist_name);
    let file_path = folder_path.join(file_name);

    if file_path.exists() && file_path.is_file() {
        fs::remove_file(&file_path)?;
        println!("Deleted file: {}", file_path.display());
        Ok(())
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("File {} not found in playlist {}", file_name, playlist_name),
        ))
    }
}

pub fn list_playlists() -> Vec<String> {
    let playlists_dir = std::env::current_dir()
        .expect("Failed to get current working directory")
        .join("playlists");

    if !playlists_dir.exists() {
        return vec![]; // no playlists folder yet
    }

    fs::read_dir(&playlists_dir)
        .unwrap()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().is_dir()) // only folders
        .filter_map(|entry| entry.file_name().into_string().ok()) // convert OsString to String
        .collect()
}

pub fn playlists() {
    let playlists = list_playlists();
    println!("Playlists:");
    for playlist in playlists {
        println!("{}", playlist);
    }
}
