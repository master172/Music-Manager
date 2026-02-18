pub mod command;
pub mod executor;
pub mod parser;

use std::io::{self, Write};

use crate::app_interface::AppInterface;

pub fn start(app: &mut dyn AppInterface) {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("failed to read input");
            continue;
        }
        let command = parser::parse(&input);
        let result = executor::execute(command, app);
        if !result {
            break;
        }
    }
}
