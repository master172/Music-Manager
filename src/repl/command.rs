#[derive(Debug)]
pub enum PlaylistOptions {
    New,
    Delete,
    Enter,
}

#[derive(Debug)]
pub enum PlaybackOptions {
    Play,
    Pause,
    Stop,
    Resume,
    Seek(i32),
    Repeat(i32),
    PlaySelected(String),
}

#[derive(Debug)]
pub enum Command {
    Quit,
    Playlist {
        option: PlaylistOptions,
        name: String,
    },
    Playback {
        option: PlaybackOptions,
    },
    Search {
        query: String,
        limit: usize,
    },
    Add {
        link: String,
    },
    List,
    Delete(String),
    Help,
    Return,
    Unknown(String),
}
