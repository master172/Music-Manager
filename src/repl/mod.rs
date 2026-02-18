pub mod command;
pub mod executor;
pub mod parser;

use std::io::{self, Write};

pub fn start() {
    loop {
        println!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("failed to read input");
            continue;
        }
        let command = parser::parse(&input);
        let result = executor::execute(command);
        if !result {
            break;
        }
    }
}
