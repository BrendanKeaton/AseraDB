use crate::consts::COMMAND_OPTIONS;

pub fn handle_sql_inputs(input: &str) -> Result<(), &'static str> {
    match input {
        "SELECT" => {
            run_select()?;
            Ok(())
        }
        "INSERT" => {
            run_insert()?;
            Ok(())
        }
        "UPDATE" => {
            run_update()?;
            Ok(())
        }
        "DELETE" => {
            run_delete()?;
            Ok(())
        }
        "EXIT" => Ok(()),
        _ => Err("invalid input"),
    }
}
