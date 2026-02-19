pub mod command;
pub mod executor;
pub mod parser;
pub mod tokenizer;

use std::sync::{Arc, Mutex};

use crate::MusicManager;
use rustyline::DefaultEditor;

pub fn start(app: Arc<Mutex<MusicManager>>) {
    let mut rl = DefaultEditor::new().unwrap();

    loop {
        match rl.readline("> ") {
            Ok(input) => {
                let command = parser::parse(&input);

                let mut mgr = app.lock().unwrap();
                if !executor::execute(command, &mut *mgr) {
                    break;
                }
            }
            Err(rustyline::error::ReadlineError::Interrupted) => {
                // Ctrl-C → just show prompt again
                continue;
            }
            Err(rustyline::error::ReadlineError::Eof) => {
                // Ctrl-D / Ctrl-Z → exit cleanly
                break;
            }
            Err(err) => {
                eprintln!("read error: {err}");
                break;
            }
        }
    }
}
