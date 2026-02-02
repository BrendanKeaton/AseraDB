pub mod commands;

use crate::enums::Command;
use crate::structs::QueryObject;

pub fn handle_query(query: &mut QueryObject) -> Result<(), String> {
    println!("{}", query);
    match query.command {
        Some(Command::SELECT) => {
            println!("Not implemented");
        }
        Some(Command::CREATE) => {
            let _ = commands::create_new_table(query);
        }
        Some(Command::INSERT) => {
            todo!();
        }
        Some(Command::DELETE) => {
            todo!();
        }
        Some(Command::EXIT) => {
            return Ok(());
        }

        None => todo!(),
    }
    return Ok(());
}
