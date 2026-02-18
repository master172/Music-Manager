use std::fs;
mod repl;
mod song_manager;

fn create_initial_playlist_dir() -> std::io::Result<()> {
    if !fs::exists("playlists")? {
        fs::create_dir("playlists")?;
    }
    Ok(())
}

fn main() {
    if let Err(e) = create_initial_playlist_dir() {
        println!("Error creating directory {}", e);
        return;
    }
    repl::start();
}
