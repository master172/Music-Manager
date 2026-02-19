use std::fs;
use std::path::Path;

use crate::app_interface::AppInterface;
use crate::song_manager::audio_commands::{AudioCommands, AudioEvent};
use crate::song_manager::track_manager;
mod app_interface;
mod playlist_manager;
mod repl;
mod song_manager;

#[derive(Debug)]
pub struct PlaylistState {
    pub name: String,
}

#[derive(Debug)]
enum State {
    Main,
    Playlist(PlaylistState),
}

pub struct MusicManager {
    audio_tx: std::sync::mpsc::Sender<AudioCommands>,
    event_rx: std::sync::mpsc::Receiver<AudioEvent>,
    state: State,
}

impl MusicManager {
    pub fn handle_event(&mut self, event: AudioEvent) -> bool {
        match event {
            AudioEvent::Quit => {
                return false;
            }
            AudioEvent::TrackFinished => {
                self.play();
            }
            AudioEvent::Error(e) => {
                println!("error occured {}", e);
            }
        }
        true
    }

    pub fn run(&mut self) {
        println!("Music Manager Working");
        while let Ok(event) = self.event_rx.recv() {
            if !self.handle_event(event) {
                break;
            }
        }
    }
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
                self.state = State::Playlist(PlaylistState { name });
            } else {
                println!("no souch playlist exsists")
            }
        }
    }

    fn play(&mut self) {
        match &mut self.state {
            State::Main => println!("no playlist selected"),
            State::Playlist(playlist) => {
                track_manager::play_playlist(&playlist.name, &self.audio_tx);
            }
        }
    }

    fn pause(&mut self) {
        self.audio_tx.send(AudioCommands::Pause).unwrap();
    }

    fn stop(&mut self) {
        self.audio_tx.send(AudioCommands::Stop).unwrap();
    }

    fn resume(&mut self) {
        self.audio_tx.send(AudioCommands::Resume).unwrap();
    }

    fn help(&mut self) {
        println!("To create a playlist type playlist new playlist_name");
        println!("To delete a playlist type playlist delete playlist_name");
        println!("To enter a playlist type playlist enter playlist_name");
        println!("To exit type exit or quit");
        println!(
            "When in a playlist type play to play, pause to pause current song,\n resume to resume the song, stop to stop the song"
        )
    }

    fn return_to_main(&mut self) {
        self.state = State::Main;
    }

    fn search(&mut self, query: String, limit: usize) {}

    fn add(&mut self, link: String) {}

    fn quit(&mut self) {
        self.audio_tx.send(AudioCommands::Quit).unwrap();
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
    let (event_tx, event_rx) = std::sync::mpsc::channel();
    let audio_tx = song_manager::audio_thread::start_audio_thread(event_tx);
    let mut manager: MusicManager = MusicManager {
        state: State::Main,
        audio_tx,
        event_rx,
    };

    if let Err(e) = create_initial_playlist_dir() {
        println!("Error creating directory {}", e);
        return;
    }

    repl::start(&mut manager);
}
