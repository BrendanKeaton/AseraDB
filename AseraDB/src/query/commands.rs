use crate::structs::{Field, QueryObject, TableMetadata};
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

    let fields: Vec<Field> = query
        .fields
        .iter()
        .zip(field_types.iter())
        .zip(indexed_fields.iter())
        .map(|((name, data_type), &is_indexed)| Field {
            name: name.to_string(),
            data_type: data_type.clone(),
            is_indexed,
        })
        .collect();

    let metadata = TableMetadata {
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
