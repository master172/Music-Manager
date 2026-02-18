use std::fs;

pub fn create_playlist(playlist_name: &str) -> std::io::Result<()> {
    fs::create_dir(format!("playlists/{}", playlist_name))?;
    Ok(())
}

pub fn delete_playlist(playlist_name: &str) -> std::io::Result<()> {
    fs::remove_dir_all(format!("playlists/{}", playlist_name))?;
    Ok(())
}
