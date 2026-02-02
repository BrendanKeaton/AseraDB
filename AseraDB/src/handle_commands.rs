use std::fs;

use crate::enums::TokenType;
use crate::structs::TableMetadata;
use crate::utils::classify_token;
use crate::{enums::Command, enums::FieldTypesAllowed, enums::ValueTypes, structs::QueryObject};

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

fn parse_field(s: &str) -> Result<(ValueTypes, FieldTypesAllowed, &str), String> {
    let count_colon: usize = s.chars().filter(|c| *c == ':').count();

    if count_colon == 1 {
        let (name, ty) = s
            .split_once(':')
            .ok_or("Field must be formatted as name:type")?;

        let field_name = ValueTypes::from_str(name).ok_or("Protected name for field")?;

        let field_type = FieldTypesAllowed::from_str(ty).ok_or("Unknown field type")?;

        Ok((field_name, field_type, "false"))
    } else if count_colon == 2 {
        let parts: Vec<&str> = s.splitn(3, ':').collect();
        if parts.len() != 3 {
            return Err("Field must be formatted as name:type:is_index".to_string());
        }
        let (name, ty, is_index) = (parts[0], parts[1], parts[2]);

        let field_name = ValueTypes::from_str(name).ok_or("Protected name for field")?;

        let field_type = FieldTypesAllowed::from_str(ty).ok_or("Unknown field type")?;

        Ok((field_name, field_type, is_index))
    } else {
        Err("Incorrect formatting".to_string())
    }
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
        let (field_name, field_type, is_index_str) = parse_field(&s)?;

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

fn parse_values_from_token(token: &str, schema: &TableMetadata) -> Result<Vec<ValueTypes>, String> {
    let parts: Vec<&str> = token.split(':').collect();

    if parts.len() != schema.fields.len() {
        return Err(format!(
            "Column count mismatch: expected {}, got {}",
            schema.fields.len(),
            parts.len()
        ));
    }

    let mut values = Vec::new();
    for (part, field) in parts.iter().zip(schema.fields.iter()) {
        values.push(ValueTypes::String(part.to_string()));
    }

    Ok(values)
}

pub fn handle_insert(tokens: &[&str], query: &mut QueryObject) -> Result<bool, String> {
    query.command = Some(Command::INSERT);
    query.index += 1;

    // table name
    query.table = tokens[query.index].to_owned();
    query.index += 1;

    // load schema to know how many values per token
    let schema_path = format!("database/catalogs/{}.json", query.table);
    let json = fs::read_to_string(&schema_path).map_err(|e| e.to_string())?;
    let schema: TableMetadata = serde_json::from_str(&json).map_err(|e| e.to_string())?;

    // parse values from each token
    while query.index < tokens.len() {
        if let TokenType::VALUE(ValueTypes::String(s)) = classify_token(tokens[query.index]) {
            let parsed_values = parse_values_from_token(&s, &schema)?;
            query.values.extend(parsed_values); // append to QueryObject
        } else {
            return Err(format!("Unexpected token: {}", tokens[query.index]));
        }
        query.index += 1;
    }

    return Ok(true);
}
