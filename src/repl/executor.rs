use crate::repl::command::{Command, PlaybackOptions, PlaylistOptions};

pub fn execute(command: Command) -> bool {
    match command {
        Command::Quit => {
            println!("Bye");
            return false;
        }
        Command::Help => {
            println!("Help menu line 1");
            println!("Help menu line 2");
        }
        Command::Playlist { option, name } => match option {
            PlaylistOptions::New => println!("create new playlist with name {}", name),
            PlaylistOptions::Delete => println!("Delete playlist with name {}", name),
            PlaylistOptions::Enter => println!("Enter playlist with name {}", name),
        },
        Command::Playback { option } => match option {
            PlaybackOptions::Play => println!("Play playlist"),
            PlaybackOptions::Pause => println!("Pause playlist"),
            PlaybackOptions::Stop => println!("Stop playlist"),
        },
        Command::Search { query, limit } => println!(
            "search for music with name {} and with limit {}",
            query, limit
        ),
        Command::Add { link } => println!("Add music with link {}", link),
        Command::Unknown(cmd) => println!("Unknown command: {}", cmd),
    };

    true
}
