use crate::enums::Command;
use crate::enums::Operand;
use crate::enums::ValueTypes;
use std::fmt;

#[derive(Default, Debug)]
pub struct QueryObject {
    pub(crate) command: Option<Command>,
    pub(crate) table: String,
    pub(crate) fields: Vec<ValueTypes>,
    pub(crate) values: Vec<ValueTypes>,
    pub(crate) conditions: Vec<Conditions>,
    pub(crate) index: usize,
    pub(crate) length: usize, // max length of query is 255 for now.
}

impl fmt::Display for QueryObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let cmd: String = match &self.command {
            Some(c) => format!("{}", c),
            None => "None".to_string(),
        };
        let fields: Vec<String> = self
            .fields
            .iter()
            .map(|v: &ValueTypes| format!("{}", v))
            .collect();
        let values: Vec<String> = self
            .values
            .iter()
            .map(|v: &ValueTypes| format!("{}", v))
            .collect();

        write!(
            f,
            "Command: {}, Table: {}, Fields: [{}], Values: [{}], Conditions: {:?}, Index: {}, Length: {}",
            cmd,
            self.table,
            fields.join(", "),
            values.join(", "),
            self.conditions,
            self.index,
            self.length
        )
    }
}

#[derive(Default, Debug)]
pub struct Conditions {
    objectOne: String,
    objectTwo: String,
    objectOneIsField: bool, // is object one a literal or field
    objecttwoIsField: bool, // is object two a literal or field
    operand: Operand,
}
