use std::fs;
use std::path::PathBuf;
use std::thread;

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
