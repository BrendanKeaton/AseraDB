pub mod utils;
use crate::core::{Command, QueryObject, TableMetadataObject, TokenType, ValueTypes};
use crate::utils::classify_token;
use std::fs;

pub fn handle_select(tokens: &[&str], query: &mut QueryObject) -> Result<bool, String> {
    query.command = Some(Command::SELECT);
    query.index += 1;
    while let TokenType::VALUE(val) = classify_token(tokens[query.index]) {
        query.fields.push(val);
        query.index += 1;

        if query.index == query.length {
            return Err("Please complete command. SELECT cannot be the final token.".to_string());
        }
    }
    return Ok(true);
}

pub fn handle_create(tokens: &[&str], query: &mut QueryObject) -> Result<bool, String> {
    query.command = Some(Command::CREATE);
    query.field_type = Some(Vec::new());
    query.is_field_index = Some(Vec::new());
    query.index += 1;

    query.table = tokens[query.index].to_owned();
    query.index += 1;

    let field_types = query.field_type.as_mut().unwrap();
    let index_fields = query.is_field_index.as_mut().unwrap();

    while let TokenType::VALUE(ValueTypes::String(s)) = classify_token(tokens[query.index]) {
        let (field_name, field_type, is_index_str) = utils::parse_field(&s)?;

        let is_index = match is_index_str {
            "false" => false,
            "true" => true,
            _ => return Err("Invalid index flag (expected true or false)".into()),
        };

        query.fields.push(field_name);
        field_types.push(field_type);
        index_fields.push(is_index);

        query.index += 1;

        if query.index == query.length {
            return Ok(true);
        }
    }

    return Ok(true);
}

pub fn handle_insert(tokens: &[&str], query: &mut QueryObject) -> Result<bool, String> {
    query.command = Some(Command::INSERT);
    query.index += 1;

    query.table = tokens[query.index].to_owned();
    query.index += 1;

    let schema_path = format!("database/catalogs/{}.json", query.table);
    let json = fs::read_to_string(&schema_path).map_err(|e| e.to_string())?;
    let schema: TableMetadataObject = serde_json::from_str(&json).map_err(|e| e.to_string())?;

    while query.index < tokens.len() {
        if let TokenType::VALUE(ValueTypes::String(s)) = classify_token(tokens[query.index]) {
            let parsed_values = utils::parse_values_from_token(&s, &schema)?;
            query.values.extend(parsed_values);
        } else {
            return Err(format!("Unexpected token: {}", tokens[query.index]));
        }
        query.index += 1;
    }

    return Ok(true);
}
