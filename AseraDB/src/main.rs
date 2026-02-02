mod consts;
mod enums;
mod handle_commands;
mod query;
mod structs;
mod utils;

use colored::*;
use query::handle_query;
use std::io::{self, Write};
use utils::handle_sql_inputs;

use crate::{enums::Command, structs::QueryObject};

fn main() {
    run();
}

fn run() {
    loop {
        print!("Enter command: ");
        if io::stdout().flush().is_err() {
            println!("{}", "Failed to flush stdout. Retrying".red());
            continue;
        }
        let mut cmd = String::new();
        match io::stdin().read_line(&mut cmd) {
            Ok(_) => {
                let cmd = cmd.trim().to_lowercase();
                let mut query: QueryObject = QueryObject::default();
                if !(handle_sql_inputs(&cmd.as_str(), &mut query)) {
                    break;
                }
                if query.command != Some(Command::EXIT) || query.command != None {
                    let _ = handle_query(&mut query);
                }
            }
            Err(_) => {
                println!("{}", "Failed to read input, please try again.".red());
                continue;
            }
        }
    }
}
