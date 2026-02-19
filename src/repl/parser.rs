use crate::repl::command::{Command, PlaybackOptions, PlaylistOptions};
use crate::repl::tokenizer::tokenize;

pub fn parse(input: &str) -> Command {
    //tokenize the input string currently just splitting whitespace and inverted commas later can substitute for a proper tokenizer
    let tokens: Vec<String> = tokenize(input);

    //parsing currently also only supports whitespace and inverted comma separated ordered fixed tokens
    match tokens
        .iter()
        .map(|s| s.as_str())
        .collect::<Vec<_>>()
        .as_slice()
    {
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

        ["repeat", count] => {
            let repeat_count: i32 = count.parse().expect("failed to parse repeat count");
            return Command::Playback {
                option: PlaybackOptions::Repeat(repeat_count),
            };
        }

        ["play", path] => Command::Playback {
            option: PlaybackOptions::PlaySelected(path.to_string()),
        },

        ["search", query, limit] => Command::Search {
            query: query.to_string(),
            limit: limit.parse().unwrap(),
        },

        ["search", query] => Command::Search {
            query: query.to_string(),
            limit: 10,
        },

        ["list"] => Command::List,
        ["delete", name] => Command::Delete(name.to_string()),

        ["add", url] => Command::Add {
            link: url.to_string(),
        },

        ["return"] => Command::Return,

        _ => Command::Unknown(input.to_string()),
    }
}
