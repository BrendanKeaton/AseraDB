use std::process::id;

use crate::{
    core::{FieldTypesAllowed, Page, QueryObject, TableMetadataObject, ValueTypes},
    parsing::get_table_schema,
};

pub fn insert_new_data(query: &mut QueryObject) -> Result<(), String> {
    let schema = get_table_schema(&query.table)?;

    let row_bytes = build_row_byte(&schema, &query.values)?;

    println!("Inserting row bytes: {:?}", row_bytes); // Test command is : insert profile billy:24:172912
    let page: Page = build_new_page(&row_bytes);

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

fn build_new_page(prelim_row_data: &Vec<u8>) -> Page {
    let PAGE_HEADER_SIZE: u16 = 8 as u16; // this should go into a "consts" file of some sort
    let mut page: Page = Page::default();

    // This curr_page_id value should be replaced w/ Metadata.len() and page size divison to get the "real" value.
    // Link here: https://doc.rust-lang.org/stable/std/fs/struct.Metadata.html#method.len
    // For now/testing, setting to "1" and deleting each time is fine
    // TODO
    let curr_page_id = 1;

    page.id = curr_page_id;

    page.data[0] = 1 as u8; // id - 1b
    page.data[1] = 1 as u8; // row_count - 1b

    let len: u16 = prelim_row_data.len() as u16;
    let bytes = len.to_le_bytes();
    // len of first row of data, max ~65k (over a page size, but u8 is too small). This is saved to the "free page offset"
    // To calculate the next page of offset, you would just take this value, and minus the new
    // rows length
    page.data[2..4].copy_from_slice(&bytes);

    let data_used: u16 = PAGE_HEADER_SIZE + len;
    let space_remaining_bytes = data_used.to_le_bytes();
    page.data[5..6].copy_from_slice(&space_remaining_bytes); // This is the amount of space remaining.. update on insert / delete

    page.data[7] = 0 as u8; // This is Last Sequence Number (for WAL recovery)... this needs to be updated as the "actual" value once WAL is created in this repo TODO

    return page;
}
