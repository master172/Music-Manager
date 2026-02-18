use std::fs;
mod song_manager;

fn create_initial_playlist_dir() -> std::io::Result<()> {
    fs::create_dir("playlists")?;
    Ok(())
}

fn main() {
    if let Err(e) = create_initial_playlist_dir() {
        println!("Error creating directory {}", e);
        return;
    }
    song_manager::audio_player::play_audio("data/testing/test.mp3")
}
