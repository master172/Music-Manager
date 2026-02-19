pub mod command;
pub mod executor;
pub mod parser;
pub mod tokenizer;

use std::{
    io::{self, Write},
    sync::{Arc, Mutex},
};

use crate::MusicManager;

pub fn start(app: Arc<Mutex<MusicManager>>) {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("failed to read input");
            continue;
        }
        let command = parser::parse(&input);

        let mut mgr = app.lock().unwrap();
        if !executor::execute(command, &mut *mgr) {
            break;
        }
    }
}
