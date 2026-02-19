use crate::song_manager::audio_commands::AudioCommands;
use rand::seq::IteratorRandom;
use std::fs;

fn select_random_song(playlist_name: &str) -> String {
    let mut rng = rand::rng();
    let files = fs::read_dir(format!("playlists/{}", playlist_name)).unwrap();
    let file = files.choose(&mut rng).unwrap().unwrap();
    let file_path = file.path().to_str().unwrap().to_string();
    return file_path;
}

pub fn play_playlist(playlist_name: &str, audio_tx: &std::sync::mpsc::Sender<AudioCommands>) {
    println!("playing playlist");
    let song = select_random_song(playlist_name);
    audio_tx.send(AudioCommands::Play(song)).unwrap();
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
