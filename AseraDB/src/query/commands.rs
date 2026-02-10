use crate::{
    core::{FieldObject, FieldTypesAllowed, QueryObject, TableMetadataObject, ValueTypes},
    parsing::get_table_schema,
};
use std::fs;

pub fn create_new_table(query: &mut QueryObject) -> Result<(), String> {
    let table_name = query.table.clone();

    let field_types = query.field_type.as_ref().ok_or("field_type is None")?;

    let indexed_fields = query
        .is_field_index
        .as_ref()
        .ok_or("indexed fields is None.")?;

    if query.fields.len() != field_types.len() {
        return Err("Fields and field types length mismatch".to_string());
    }

    let fields: Vec<FieldObject> = query
        .fields
        .iter()
        .zip(field_types.iter())
        .zip(indexed_fields.iter())
        .map(|((name, data_type), &is_indexed)| FieldObject {
            name: name.to_string().trim_matches('"').to_string(),
            data_type: data_type.clone(),
            is_indexed,
        })
        .collect();

    let metadata = TableMetadataObject {
        table_name: table_name.clone(),
        fields,
    };

    let dir_path = "database/catalogs";
    let file_path = format!("{}/{}.json", dir_path, table_name);

    fs::create_dir_all(dir_path)
        .map_err(|e| format!("Failed to create database directory: {}", e))?;

    let json = serde_json::to_string_pretty(&metadata)
        .map_err(|e| format!("Failed to serialize metadata: {}", e))?;

    fs::write(&file_path, json).map_err(|e| format!("Failed to write table file: {}", e))?;

    Ok(())
}

fn build_row_byte(schema: &TableMetadataObject, values: &[ValueTypes]) -> Result<bool, String> {
    let num_columns: usize = schema.fields.len();
    let row_header_size: usize = 1 + (num_columns * 32);

    let mut row_header: Vec<String> = Vec::new();

    let mut is_first: bool = true;

    let mut row_data: String = String::new();

    for (field, value) in schema.fields.iter().zip(values.iter()) {
        if !is_first {
            row_data.push_str(",");
        } else {
            is_first = false;
        }
        let raw: &str = value
            .as_str()
            .ok_or_else(|| format!("value for field '{}' is None", field.name))?;

        match field.data_type {
            FieldTypesAllowed::I8 => {
                let v = raw
                    .parse::<i8>()
                    .map_err(|_| format!("invalid I8 for field '{}'", field.name))?;
                row_data.push_str(&v.to_string());
            }
            FieldTypesAllowed::I32 => {
                let v = raw
                    .parse::<i32>()
                    .map_err(|_| format!("invalid I32 for field '{}'", field.name))?;
                row_data.push_str(&v.to_string());
            }
            FieldTypesAllowed::String => {
                let max_len = 255;
                if raw.len() > max_len {
                    return Err(format!(
                        "string length for field '{}' exceeds maximum of {}",
                        field.name, max_len
                    ));
                }
                row_data.push_str(raw);
            }
        }
    }
    Ok(true)
}

pub fn insert_new_data(query: &mut QueryObject) -> Result<(), String> {
    let schema = get_table_schema(&query.table)?;

    build_row_byte(&schema, &query.values)?;

    return Ok(());
}
