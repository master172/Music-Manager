use std::fs::File;
use std::sync::mpsc;
use std::thread;

use crate::song_manager::audio_commands::{AudioCommands, AudioEvent};
use rodio::Decoder;
use std::time::Duration;

pub fn start_audio_thread(event_tx: mpsc::Sender<AudioEvent>) -> mpsc::Sender<AudioCommands> {
    let mut music_playing: bool = false;
    let (tx, rx) = mpsc::channel();

    let mut repeat_count: i32 = 0;
    let mut current_song: Option<String> = None;

    thread::spawn(move || {
        let stream_handle = rodio::OutputStreamBuilder::open_default_stream()
            .expect("failed to get default audio stream");
        let mut sink = rodio::Sink::connect_new(&stream_handle.mixer());

        let tick = Duration::from_millis(100);

        loop {
            match rx.recv_timeout(tick) {
                Ok(command) => match command {
                    AudioCommands::Play(path) => {
                        music_playing = true;
                        let mut path_to_play = path.clone();

                        if repeat_count != 0
                            && let Some(song) = &current_song
                        {
                            path_to_play = song.clone();
                            current_song = Some(song.clone());
                            repeat_count -= 1;
                        } else {
                            current_song = Some(path.clone());
                        }

                        sink.stop();
                        sink = rodio::Sink::connect_new(&stream_handle.mixer());

                        let file = File::open(&path_to_play).unwrap();
                        let source = Decoder::try_from(file).unwrap();
                        sink.append(source);
                        sink.play();
                    }
                    AudioCommands::Pause => sink.pause(),
                    AudioCommands::Resume => sink.play(),
                    AudioCommands::Stop => {
                        sink.stop();
                        music_playing = false
                    }
                    AudioCommands::Repeat(count) => {
                        repeat_count = count;
                    }
                    AudioCommands::Seek(time) => {
                        sink.try_seek(Duration::from_secs(time as u64)).unwrap()
                    }
                    AudioCommands::Quit => {
                        let _ = event_tx.send(AudioEvent::Quit).unwrap();
                        break;
                    }
                },
                Err(mpsc::RecvTimeoutError::Timeout) => {}
                Err(_) => break,
            }
            if sink.empty() && music_playing {
                let _ = event_tx.send(AudioEvent::TrackFinished).unwrap();
            }
        }
    });

    tx
}
