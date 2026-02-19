use std::fs::File;
use std::thread;
use std::{io::Sink, sync::mpsc};

use crate::song_manager::audio_commands::{AudioCommands, AudioEvent};
use rodio::Decoder;
use std::time::Duration;

pub fn start_audio_thread(event_tx: mpsc::Sender<AudioEvent>) -> mpsc::Sender<AudioCommands> {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let stream_handle = rodio::OutputStreamBuilder::open_default_stream()
            .expect("failed to get default audio stream");
        let mut sink = rodio::Sink::connect_new(&stream_handle.mixer());

        let tick = Duration::from_millis(100);

        loop {
            match rx.recv_timeout(tick) {
                Ok(command) => match command {
                    AudioCommands::Play(path) => {
                        sink.stop();
                        sink = rodio::Sink::connect_new(&stream_handle.mixer());

                        let file = File::open(path).unwrap();
                        let source = Decoder::try_from(file).unwrap();
                        sink.append(source);
                        sink.play();
                    }
                    AudioCommands::Pause => sink.pause(),
                    AudioCommands::Resume => sink.play(),
                    AudioCommands::Stop => sink.stop(),
                    AudioCommands::Quit => {
                        event_tx.send(AudioEvent::Quit);
                        break;
                    }
                },
                Err(mpsc::RecvTimeoutError::Timeout) => {}
                Err(_) => break,
            }
            if sink.empty() {
                let _ = event_tx.send(AudioEvent::TrackFinished);
            }
        }
    });

    tx
}
