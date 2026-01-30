use crate::enums::Command;
use crate::enums::ValueTypes;

#[derive(Default)]
pub struct QueryObject {
    command: Option<Command>,
    table: String,
    columns: Vec<String>,
    values: Vec<ValueTypes>,
    index: i8, // max length of query is 255
    length: i8,
}
