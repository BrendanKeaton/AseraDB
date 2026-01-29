mod consts;
mod structs;
mod utils;

use crate::consts::COMMAND_OPTIONS;
use colored::*;
use std::io::{self, Write};
use utils::handle_sql_inputs;

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
                let cmd = cmd.trim().to_uppercase();

                if cmd == "EXIT" {
                    break;
                } else if COMMAND_OPTIONS.contains(&cmd.as_str()) {
                    handle_sql_inputs(&cmd.as_str());
                    break;
                } else {
                    println!("");
                    println!("{}", "Not a valid input command, please try again.".red());
                    println!("OPTIONS ARE: EXIT, {}", COMMAND_OPTIONS.join(", "));
                }
            }
            Err(_) => {
                println!("{}", "Failed to read input, please try again.".red());
                continue;
            }
        }
    }
}
