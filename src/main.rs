use std::fs;
use std::path::Path;

use crate::app_interface::AppInterface;
use crate::song_manager::track_manager;
mod app_interface;
mod playlist_manager;
mod repl;
mod song_manager;

#[derive(Debug)]
pub enum PlaybackState {
    playing,
    paused,
    stopped,
}

#[derive(Debug)]
pub struct PlaylistState {
    pub name: String,
    pub playback_state: PlaybackState,
}

#[derive(Debug)]
enum State {
    Main,
    Playlist(PlaylistState),
}

pub struct MusicManager {
    state: State,
}

impl MusicManager {
    fn audio_playback_finished(&mut self) {}
}

impl AppInterface for MusicManager {
    fn create_playlist(&mut self, name: String) {
        if let Err(e) = playlist_manager::playlist_manager::create_playlist(&name) {
            println!("error creating playlist {} with error {}", name, e);
        }
    }

    fn delete_playlist(&mut self, name: String) {
        if let Err(e) = playlist_manager::playlist_manager::delete_playlist(&name) {
            println!("error deleting playlist {} with error {}", name, e);
        }
    }

    fn enter_playlist(&mut self, name: String) {
        if matches!(&self.state, State::Main) {
            let path = format!("playlists/{}", name);
            if Path::new(&path).exists() {
                self.state = State::Playlist(PlaylistState {
                    name,
                    playback_state: PlaybackState::stopped,
                });
            } else {
                println!("no souch playlist exsists")
            }
        }
    }

    fn play(&mut self) {
        match &mut self.state {
            State::Main => println!("no playlist selected"),
            State::Playlist(playlist) => {
                playlist.playback_state = PlaybackState::playing;
                track_manager::play_playlist(&playlist.name);
            }
        }
    }

    fn pause(&mut self) {}

    fn stop(&mut self) {}

    fn help(&mut self) {}

    fn return_to_main(&mut self) {
        self.state = State::Main;
    }

    fn search(&mut self, query: String, limit: usize) {}

    fn add(&mut self, link: String) {}

    fn quit(&mut self) {
        println! {"exiting"};
    }
}

fn create_initial_playlist_dir() -> std::io::Result<()> {
    if !fs::exists("playlists")? {
        fs::create_dir("playlists")?;
    }
    Ok(())
}

fn main() {
    let mut manager: MusicManager = MusicManager { state: State::Main };

    if let Err(e) = create_initial_playlist_dir() {
        println!("Error creating directory {}", e);
        return;
    }

    repl::start(&mut manager);
}
