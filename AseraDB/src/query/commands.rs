use crate::{
    core::{FieldObject, FieldTypesAllowed, QueryObject, TableMetadataObject, ValueTypes},
    parsing::get_table_schema,
};
use core::num;
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

fn build_row_byte(schema: &TableMetadataObject, values: &[ValueTypes]) -> Result<Vec<u8>, String> {
    let num_columns: usize = schema.fields.len();

    let mut row_header: Vec<u8> = Vec::new();
    let mut row_data: Vec<u8> = Vec::new();

    for (field, value) in schema.fields.iter().zip(values.iter()) {
        let raw: &str = value
            .as_str()
            .ok_or_else(|| format!("value for field '{}' is None", field.name))?;

        match field.data_type {
            FieldTypesAllowed::I8 => {
                let v = raw
                    .parse::<i8>()
                    .map_err(|_| format!("invalid I8 for field '{}'", field.name))?;
                row_data.push(v as u8);
                row_header.push(1);
            }
            FieldTypesAllowed::I32 => {
                let v = raw
                    .parse::<i32>()
                    .map_err(|_| format!("invalid I32 for field '{}'", field.name))?;
                row_data.extend_from_slice(&v.to_le_bytes());
                row_header.push(4);
            }
            FieldTypesAllowed::String => {
                let max_len = 255;
                let bytes = raw.as_bytes();
                let byte_len = bytes.len();

                if byte_len > max_len {
                    return Err(format!(
                        "string byte length for field '{}' exceeds maximum of {}",
                        field.name, max_len
                    ));
                }
                row_data.extend_from_slice(bytes);
                row_header.push(byte_len as u8);
            }
        }
    }

    let mut result: Vec<u8> = Vec::new();
    let row_header_size = 1 + num_columns;

    result.push(row_header_size as u8);
    result.push(num_columns as u8);
    result.extend(row_header);
    result.extend(row_data);

    Ok(result)
}
pub fn insert_new_data(query: &mut QueryObject) -> Result<(), String> {
    let schema = get_table_schema(&query.table)?;

    let row_bytes = build_row_byte(&schema, &query.values)?;

    println!("Inserting row bytes: {:?}", row_bytes); // Test command is : insert profile brendan:24:172912

    return Ok(());
}
