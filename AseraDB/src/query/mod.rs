pub mod commands;

use crate::core::Command;
use crate::core::QueryObject;

pub fn handle_query(query: &mut QueryObject) -> Result<bool, String> {
    println!("{}", query);
    match query.command {
        Some(Command::SELECT) => {
            println!("Not implemented");
        }
        Some(Command::CREATE) => {
            let _ = commands::create_new_table(query);
        }
        Some(Command::INSERT) => {
            let _ = commands::insert_new_data(query);
        }
        Some(Command::EXIT) => {
            return Ok(false);
        }

        None => todo!(),
    }
    return Ok(true);
}
