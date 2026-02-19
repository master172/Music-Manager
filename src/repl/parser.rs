use crate::repl::command::{Command, PlaybackOptions, PlaylistOptions};

pub fn parse(input: &str) -> Command {
    //tokenize the input string currently just splitting whitespace later can substitute for a proper tokenizer
    let tokens: Vec<&str> = input.split_whitespace().collect();

    //parsing currently also only supports whitespace separated ordered fixed tokens
    match tokens.as_slice() {
        ["quit"] | ["exit"] => Command::Quit,

        ["help"] => Command::Help,

        ["playlist", "new", name] => Command::Playlist {
            option: PlaylistOptions::New,
            name: name.to_string(),
        },

        ["playlist", "delete", name] => Command::Playlist {
            option: PlaylistOptions::Delete,
            name: name.to_string(),
        },

        ["playlist", "enter", name] => Command::Playlist {
            option: PlaylistOptions::Enter,
            name: name.to_string(),
        },

        ["play"] => Command::Playback {
            option: PlaybackOptions::Play,
        },

        ["stop"] => Command::Playback {
            option: PlaybackOptions::Stop,
        },

        ["resume"] => Command::Playback {
            option: PlaybackOptions::Resume,
        },

        ["pause"] => Command::Playback {
            option: PlaybackOptions::Pause,
        },

        ["seek", time] => {
            let seek_time: i32 = time.parse().expect("failed to parse seek time");
            return Command::Playback {
                option: PlaybackOptions::Seek(seek_time),
            };
        }

        ["search", query, limit] => Command::Search {
            query: query.to_string(),
            limit: limit.parse().unwrap_or(10),
        },

        ["add", url] => Command::Add {
            link: url.to_string(),
        },

        ["return"] => Command::Return,

        _ => Command::Unknown(input.to_string()),
    }
}
