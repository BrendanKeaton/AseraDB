use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::Path;

use crate::{
    core::{FieldTypesAllowed, Page, QueryObject, TableMetadataObject, ValueTypes},
    parsing::get_table_schema,
};

pub const PAGE_SIZE: usize = 4096; // This should become a gloabl const in ../core : TODO

pub fn insert_new_data(query: &mut QueryObject) -> Result<(), String> {
    let schema = get_table_schema(&query.table)?;

    let row_bytes = build_row_byte(&schema, &query.values)?;

    println!("Inserting row bytes: {:?}", row_bytes); // Test command is : insert profile billy:24:172912
    let _ = find_page(&query.table, row_bytes);

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

fn build_new_page(prelim_row_data: &Vec<u8>, table_name: &str) -> Result<Page, String> {
    let page_header_size: u16 = 8 as u16; // this should go into a "consts" file of some sort
    let mut page: Page = Page::default();

    let schema_path = format!("database/tables/{}.asera", table_name);
    let path = Path::new(&schema_path);

    let curr_page_id: u64;
    if path.exists() {
        let file: File = File::open(&schema_path).map_err(|e| e.to_string())?;

        let metadata = file.metadata().map_err(|e| e.to_string())?;
        curr_page_id = metadata.len() / PAGE_SIZE as u64;
    } else {
        curr_page_id = 0;
        File::create(path).expect("Failed to create table file");
    }

    page.id = curr_page_id;

    page.data[0] = 1 as u8; // id - 1b
    page.data[1] = 1 as u8; // row_count - 1b

    let len: u16 = prelim_row_data.len() as u16;
    let bytes = len.to_le_bytes();
    // len of current rows of data, max ~65k (over a page size, but u8 is too small). This is saved to the "free page offset"
    // To calculate the next page of offset, you would just take this value, and minus the new
    // rows length
    page.data[2..4].copy_from_slice(&bytes);

    let data_used: u16 = page_header_size + len;
    let space_remaining: u16 = PAGE_SIZE as u16 - data_used;
    page.data[4..6].copy_from_slice(&space_remaining.to_le_bytes()); // This is the amount of space remaining.. update on insert / delete

    // This is Last Sequence Number (for WAL recovery)... this needs to be updated as the "actual" value once WAL is created in this repo TODO
    page.data[6] = 0 as u8;
    let new_row_start = PAGE_SIZE - len as usize;
    page.data[new_row_start..PAGE_SIZE].copy_from_slice(prelim_row_data);

    let _ = insert_page(table_name, &page);

    return Ok(page);
}

fn find_page(table_name: &str, row_bytes: Vec<u8>) -> Result<Page, String> {
    let mut page: Page = Page::default();
    let schema_path = format!("database/tables/{}.asera", table_name);

    if !Path::new(&schema_path).exists() {
        let page = build_new_page(&row_bytes, table_name)?;
        return Ok(page);
    }
    let mut file: File = File::open(&schema_path).map_err(|e| e.to_string())?;

    let metadata = file.metadata().map_err(|e| e.to_string())?;
    let file_len = metadata.len();
    let row_len: u64 = row_bytes.len() as u64;

    for curr_page_id in 0..(file_len / PAGE_SIZE as u64) {
        file.seek(SeekFrom::Start(curr_page_id * PAGE_SIZE as u64))
            .map_err(|e| e.to_string())?;

        file.read_exact(&mut page.data).map_err(|e| e.to_string())?;

        let space_remaining = u16::from_le_bytes(
            page.data[4..6]
                .try_into()
                .map_err(|_| "Corrupt page header")?,
        );

        if space_remaining as u64 > row_len {
            page.pin_count += 1;
            page.dirty = true;
            page.id = curr_page_id;
            page.data[1] += 1;
            let current_offset = u16::from_le_bytes(
                page.data[2..4]
                    .try_into()
                    .map_err(|_| "Corrupt page header")?,
            );
            let start_new_data: usize = PAGE_SIZE - current_offset as usize - row_len as usize;
            let end_new_data = start_new_data + row_len as usize;
            page.data[start_new_data..end_new_data].copy_from_slice(&row_bytes);
            let new_offset = current_offset + row_len as u16;
            let current_space_remaining = u16::from_le_bytes(
                page.data[4..6]
                    .try_into()
                    .map_err(|_| "Corrupt page header")?,
            );
            let new_space_remaining = current_space_remaining - row_len as u16;
            // TODO update LSN once WAL is implemented
            page.data[2..4].copy_from_slice(&new_offset.to_le_bytes());
            page.data[4..6].copy_from_slice(&new_space_remaining.to_le_bytes());

            let _ = insert_page(table_name, &page);
            return Ok(page);
        }
        println!("{:?}", file);
    }
    page = build_new_page(&row_bytes, &table_name).map_err(|e| e.to_string())?;
    return Ok(page);
}

fn update_page_bytes(page: &[u8]) -> Page {
    return Page::default(); // placeholder to get an error to stop yelling at me
}

fn insert_page(table_name: &str, page: &Page) -> Result<(), String> {
    let schema_path = format!("database/tables/{}.asera", table_name);

    println!("new page: {:?}", page);

    let mut file = OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .open(&schema_path)
        .map_err(|e| e.to_string())?;

    let offset = page.id * PAGE_SIZE as u64;

    file.seek(SeekFrom::Start(offset))
        .map_err(|e| e.to_string())?;

    file.write_all(&page.data).map_err(|e| e.to_string())?;

    file.flush().map_err(|e| e.to_string())?;

    Ok(())
}
