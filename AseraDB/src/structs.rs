use crate::enums::Command;
use crate::enums::FieldTypesAllowed;
use crate::enums::Operand;
use crate::enums::ValueTypes;
use serde::{Deserialize, Serialize};
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
    pub(crate) field_type: Option<Vec<FieldTypesAllowed>>,
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
        let field_types = match &self.field_type {
            Some(ft) => ft
                .iter()
                .map(|t| format!("{}", t))
                .collect::<Vec<_>>()
                .join(", "),
            None => "None".to_string(),
        };

        write!(
            f,
            "Command: {}, Table: {}, Fields: [{}], Values: [{}], Conditions: {:?}, Index: {}, Length: {}, Field_Types[{}]",
            cmd,
            self.table,
            fields.join(", "),
            values.join(", "),
            self.conditions,
            self.index,
            self.length,
            field_types,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Field {
    pub name: String,
    pub data_type: FieldTypesAllowed,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TableMetadata {
    pub table_name: String,
    pub fields: Vec<Field>,
}
