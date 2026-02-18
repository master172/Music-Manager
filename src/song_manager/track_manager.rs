use crate::song_manager::audio_player;
use rand::seq::IteratorRandom;
use std::fs;

fn select_random_song(playlist_name: &str) -> String {
    let mut rng = rand::rng();
    let files = fs::read_dir(format!("playlists/{}", playlist_name)).unwrap();
    let file = files.choose(&mut rng).unwrap().unwrap();
    let file_path = file.path().to_str().unwrap().to_string();
    return file_path;
}

pub fn play_playlist(playlist_name: &str) {
    let song = select_random_song(playlist_name);
    let success = audio_player::play_audio(&song);
    if !success {
        println!("failed to play song {}", song);
    } else {
        println!("switching song now");
        play_playlist(playlist_name);
    }
}
