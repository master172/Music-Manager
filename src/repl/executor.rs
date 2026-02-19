use crate::app_interface::AppInterface;

use crate::repl::command::{Command, PlaybackOptions, PlaylistOptions};

pub fn execute(command: Command, app: &mut dyn AppInterface) -> bool {
    match command {
        Command::Quit => {
            app.quit();
            return false;
        }
        Command::Help => {
            app.help();
        }
        Command::Playlist { option, name } => match option {
            PlaylistOptions::New => app.create_playlist(name),
            PlaylistOptions::Delete => app.delete_playlist(name),
            PlaylistOptions::Enter => app.enter_playlist(name),
        },
        Command::Playback { option } => match option {
            PlaybackOptions::Play => app.play(),
            PlaybackOptions::Pause => app.pause(),
            PlaybackOptions::Stop => app.stop(),
            PlaybackOptions::Resume => app.resume(),
            PlaybackOptions::Seek(time) => app.seek(time),
            PlaybackOptions::Repeat(count) => app.repeat(count),
            PlaybackOptions::PlaySelected(path) => app.play_selected(path),
        },
        Command::Search { query, limit } => app.search(query, limit),
        Command::Add { link } => app.add(link),
        Command::Return => app.return_to_main(),

        Command::Unknown(cmd) => println!("Unknown command: {}", cmd),
    };

    true
}
