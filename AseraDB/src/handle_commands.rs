use crate::enums::TokenType;
use crate::utils::classify_token;
use crate::{enums::Command, enums::FieldTypesAllowed, enums::ValueTypes, structs::QueryObject};

pub fn handle_select(tokens: &[&str], query: &mut QueryObject) -> Result<(), String> {
    query.command = Some(Command::SELECT);
    query.index += 1;
    while let TokenType::VALUE(val) = classify_token(tokens[query.index]) {
        query.fields.push(val);
        query.index += 1;

        if query.index == query.length {
            return Err("Please complete command. SELECT cannot be the final token.".to_string());
        }
    }
    Ok(())
}

fn parse_field(s: &str) -> Result<(ValueTypes, FieldTypesAllowed), String> {
    let count_colon: usize = s.chars().filter(|c| *c == ':').count();

    if count_colon == 1 {
        let (name, ty) = s
            .split_once(':')
            .ok_or("Field must be formatted as name:type")?;

        let field_name = ValueTypes::from_str(name).ok_or("Protected name for field")?;

        let field_type = FieldTypesAllowed::from_str(ty).ok_or("Unknown field type")?;

        Ok((field_name, field_type))
    } else if count_colon == 2 {
        let (name, ty, is_index: &str) = s.splitn(2, ':');

        let field_name = ValueTypes::from_str(name).ok_or("Protected name for field")?;

        let field_type = FieldTypesAllowed::from_str(ty).ok_or("Unknown field type")?;

        Ok((field_name, field_type))
    } else {
        Err("Incorrect formatting".to_string())
    }
}

pub fn handle_create(tokens: &[&str], query: &mut QueryObject) -> Result<(), String> {
    query.command = Some(Command::CREATE);
    query.field_type = Some(Vec::new());
    query.index += 1;

    query.table = tokens[query.index].to_owned();
    query.index += 1;

    let field_types = query.field_type.as_mut().unwrap();

    while let TokenType::VALUE(ValueTypes::String(s)) = classify_token(tokens[query.index]) {
        let (field_name, field_type) = parse_field(&s)?;

        query.fields.push(field_name);
        field_types.push(field_type);

        query.index += 1;

        if query.index == query.length {
            return Ok(());
        }
    }

    Ok(())
}
