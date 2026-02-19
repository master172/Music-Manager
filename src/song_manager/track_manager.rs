use crate::song_manager::audio_commands::AudioCommands;
use rand::prelude::IndexedRandom;
use std::fs;

fn select_random_song(playlist_name: &str) -> Option<String> {
    let dir_path = format!("playlists/{}", playlist_name);

    let files: Vec<_> = fs::read_dir(&dir_path)
        .ok()?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| path.is_file())
        .filter(|path| {
            path.extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| ["mp3", "wav", "ogg", "flac"].contains(&ext))
                .unwrap_or(false)
        })
        .collect();

    let mut rng = rand::rng();
    files
        .choose(&mut rng)
        .and_then(|p| p.to_str().map(|s| s.to_string()))
}

pub fn play_playlist(playlist_name: &str, audio_tx: &std::sync::mpsc::Sender<AudioCommands>) {
    println!("playing playlist");
    let song = select_random_song(playlist_name);
    match song {
        Some(song) => audio_tx.send(AudioCommands::Play(song)).unwrap(),
        None => println!("No songs found in playlist"),
    }
}

pub fn play_selected(
    playlist_name: &str,
    song_name: &str,
    audio_tx: &std::sync::mpsc::Sender<AudioCommands>,
) {
    println!("playing selected");
    let song_path = format!("playlists/{}/{}", playlist_name, song_name);
    audio_tx.send(AudioCommands::Play(song_path)).unwrap();
}
