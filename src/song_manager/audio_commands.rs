pub enum AudioCommands {
    Play(String),
    Pause,
    Resume,
    Stop,
    Quit,
}

pub enum AudioEvent {
    TrackFinished,
    Quit,
    Error(String),
}
