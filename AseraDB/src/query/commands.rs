use crate::core::{FieldObject, QueryObject, TableMetadataObject, ValueTypes};
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
            name: name.to_string(),
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

fn build_row(
    schema: &TableMetadataObject,
    values: &[ValueTypes],
) -> Result<serde_json::Value, String> {
    let mut obj = serde_json::Map::new();
    for (field, value) in schema.fields.iter().zip(values.iter()) {
        obj.insert(field.name.clone(), value.to_json());
    }
    println!("here");
    Ok(serde_json::Value::Object(obj))
}

pub fn insert_new_data(query: &mut QueryObject) -> Result<(), String> {
    let schema_path = format!("database/catalogs/{}.json", query.table);
    let json = fs::read_to_string(&schema_path).unwrap();
    let schema: TableMetadataObject = serde_json::from_str(&json).unwrap();

    let row: serde_json::Value = build_row(&schema, &query.values)?;

    println!("{}", row);
    return Ok(());
}
