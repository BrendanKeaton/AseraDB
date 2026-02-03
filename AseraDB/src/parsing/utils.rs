use crate::core::{FieldTypesAllowed, TableMetadataObject, ValueTypes};

pub fn parse_field(s: &str) -> Result<(ValueTypes, FieldTypesAllowed, &str), String> {
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

pub fn parse_values_from_token(
    token: &str,
    schema: &TableMetadataObject,
) -> Result<Vec<ValueTypes>, String> {
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
