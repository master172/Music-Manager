use crate::app_interface::AppInterface;
use crate::song_manager::audio_commands::{AudioCommands, AudioEvent};
use crate::song_manager::track_manager;
use std::fs;
use std::path::Path;
use std::sync::{Arc, Mutex};
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
    //event_rx: std::sync::mpsc::Receiver<AudioEvent>,
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
                self.audio_tx.send(AudioCommands::Repeat(0)).unwrap();
            } else {
                println!("no such playlist exists")
            }
        }
    }

    fn list(&mut self) {
        match &mut self.state {
            State::Main => playlist_manager::operations::playlists(),
            State::Playlist(playlist) => {
                playlist_manager::operations::list(&playlist.name);
            }
        }
    }

    fn delete_song(&mut self, name: String) {
        match &mut self.state {
            State::Main => println!("no playlist selected"),
            State::Playlist(playlist) => {
                playlist_manager::operations::delete_file_from_playlist(&playlist.name, &name)
                    .unwrap();
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

    fn seek(&mut self, time: i32) {
        self.audio_tx.send(AudioCommands::Seek(time)).unwrap();
    }

    fn stop(&mut self) {
        self.audio_tx.send(AudioCommands::Stop).unwrap();
    }

    fn resume(&mut self) {
        self.audio_tx.send(AudioCommands::Resume).unwrap();
    }

    fn repeat(&mut self, count: i32) {
        self.audio_tx.send(AudioCommands::Repeat(count)).unwrap();
    }
    fn play_selected(&mut self, path: String) {
        match &mut self.state {
            State::Main => println!("no playlist selected"),
            State::Playlist(playlist) => {
                track_manager::play_selected(&playlist.name, &path, &self.audio_tx);
            }
        }
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

    fn search(&mut self, query: String, limit: usize) {
        playlist_manager::search::search_async(&query.replace("_", " "), limit);
    }

    fn add(&mut self, link: String) {
        match &mut self.state {
            State::Main => println!("no playlist selected"),
            State::Playlist(playlist) => {
                let path = format!("{}", &playlist.name);
                playlist_manager::downloader::download_audio(link, path);
            }
        }
    }

    fn quit(&mut self) {
        self.audio_tx.send(AudioCommands::Quit).unwrap();
        println! {"exiting"};
    }
}

fn create_initial_playlist_dir() -> std::io::Result<()> {
    if !fs::exists("playlists")? {
        fs::create_dir("playlists")?;
    }

    if !fs::exists("libs")? {
        fs::create_dir("libs")?;
    }
    Ok(())
}

fn main() {
    if let Err(e) = create_initial_playlist_dir() {
        println!("Error creating directory {}", e);
        return;
    }

    let (event_tx, event_rx) = std::sync::mpsc::channel();
    let audio_tx = song_manager::audio_thread::start_audio_thread(event_tx.clone());

    let manager = Arc::new(Mutex::new(MusicManager {
        state: State::Main,
        audio_tx: audio_tx.clone(),
        //event_rx,
    }));

    let manager_clone = Arc::clone(&manager);
    std::thread::spawn(move || {
        while let Ok(event) = event_rx.recv() {
            let mut mgr = manager_clone.lock().unwrap();
            if !mgr.handle_event(event) {
                break;
            }
        }
    });

    repl::start(Arc::clone(&manager));
}
