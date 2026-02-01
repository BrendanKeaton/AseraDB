use crate::structs::QueryObject;

pub fn handle_query(query: &mut QueryObject) -> Result<(), String> {
    println!("{}", query);
    return Ok(());
}
