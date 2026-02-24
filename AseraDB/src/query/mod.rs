pub mod create;
pub mod delete;
pub mod insert;
pub mod select;
pub mod utils;

use crate::core::Command;
use crate::core::QueryObject;

pub fn handle_query(query: &mut QueryObject) -> Result<bool, String> {
    match query.command {
        Some(Command::SELECT) => {
            let _ = select::read_data(query);
        }
        Some(Command::CREATE) => {
            let _ = create::create_new_table(query);
        }
        Some(Command::INSERT) => {
            let _ = insert::insert_new_data(query);
        }
        Some(Command::DELETE) => {
            println!("Not yet handled")
        }
        Some(Command::EXIT) => {
            return Ok(false);
        }

        None => todo!(),
    }
    return Ok(true);
}
