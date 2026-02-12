use std::process::id;

use crate::{
    core::{FieldTypesAllowed, Page, QueryObject, TableMetadataObject, ValueTypes},
    parsing::get_table_schema,
};

pub fn insert_new_data(query: &mut QueryObject) -> Result<(), String> {
    let schema = get_table_schema(&query.table)?;

    let row_bytes = build_row_byte(&schema, &query.values)?;

    println!("Inserting row bytes: {:?}", row_bytes); // Test command is : insert profile billy:24:172912
    let page: Page = build_new_page(row_bytes);

    println!("new page: {:?}", page);

    return Ok(());
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

fn build_new_page(prelim_row_data: Vec<u8>) -> Page {
    let mut page: Page = Page::default();

    // This curr_page_id value should be replaced w/ Metadata.len() and page size divison to get the "real" value.
    // Link here: https://doc.rust-lang.org/stable/std/fs/struct.Metadata.html#method.len
    // For now/testing, setting to "1" and deleting each time is fine
    // TODO
    let curr_page_id = 1;

    page.id = curr_page_id;

    // TODO: Create page header, will have-
    // id (u8, max 255 pages for toy db, can be set with above curr_page_id)
    // row_count (curr number of rows stored, start 1)
    // free page offset, number of bytes from end to start of "true" data (non header)
    // free space remaining, amount of space avail to be stored (will have to be re-done when deletes are supported, but thats fine for now)
    // Last Sequence Number (for WAL recovery)
    // Checksum.... maybe not useful for this? unsure.

    return page;
}
