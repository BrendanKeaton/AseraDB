mod consts;
mod enums;
mod handle_commands;
mod structs;
mod utils;

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
                let cmd = cmd.trim().to_lowercase();
                if !(handle_sql_inputs(&cmd.as_str())) {
                    break;
                }
            }
            Err(_) => {
                println!("{}", "Failed to read input, please try again.".red());
                continue;
            }
        }
    }
}
