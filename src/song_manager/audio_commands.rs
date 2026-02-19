pub enum AudioCommands {
    Play(String),
    Pause,
    Resume,
    Stop,
    Quit,
    Seek(i32),
    Repeat(i32),
}

pub enum AudioEvent {
    TrackFinished,
    Quit,
    Error(String),
}
