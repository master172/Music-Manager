use rodio::Decoder;
use std::fs::File;

pub fn play_audio(file_path: &str) -> bool {
    let stream_handle =
        rodio::OutputStreamBuilder::open_default_stream().expect("open default audio stream");

    let sink = rodio::Sink::connect_new(&stream_handle.mixer());

    let file = File::open(file_path).unwrap();

    let source = Decoder::try_from(file).unwrap();
    sink.append(source);

    sink.sleep_until_end();
    true
}
