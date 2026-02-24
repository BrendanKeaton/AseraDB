use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

use crate::core::{FieldTypesAllowed, PAGE_SIZE, QueryObject, TableMetadataObject, ValueTypes};

pub fn get_selected_column_ids(
    query: &QueryObject,
    schema: &TableMetadataObject,
) -> Result<Vec<u8>, String> {
    let mut return_vec: Vec<u8> = Vec::new();

    if query.fields.iter().any(|f| matches!(f, ValueTypes::STAR)) {
        return Ok((0..schema.fields.len()).map(|i| i as u8).collect());
    }

    for (index, field) in schema.fields.iter().enumerate() {
        if query
            .fields
            .iter()
            .any(|qf| qf.as_str().map_or(false, |name| name == field.name))
        {
            return_vec.push(index as u8);
        }
    }
    Ok(return_vec)
}

pub fn parse_sequential(
    query: &QueryObject,
    mut file: File,
    file_len: u64,
    schema: TableMetadataObject,
    action: &str,
) -> Result<(), String> {
    let mut page_data: [u8; PAGE_SIZE] = [0; PAGE_SIZE];
    let num_pages: u64 = file_len / PAGE_SIZE as u64;

    let selected_column_ids = get_selected_column_ids(query, &schema)?;

    for curr_page_id in 0..num_pages {
        file.seek(SeekFrom::Start(curr_page_id * PAGE_SIZE as u64))
            .map_err(|e| e.to_string())?;

        file.read_exact(&mut page_data).map_err(|e| e.to_string())?;

        if curr_page_id as u8 != page_data[0] {
            return Err("Page ID mismatch".to_string());
        }

        let row_count = page_data[1];

        for row in 0..row_count {
            let curr_slot_offset = row * 4 + 11;
            let row_offset = u16::from_le_bytes(
                page_data[curr_slot_offset as usize..(curr_slot_offset as usize + 2)]
                    .try_into()
                    .unwrap(),
            );
            let row_length = u16::from_le_bytes(
                page_data[curr_slot_offset as usize + 2..(curr_slot_offset as usize + 4)]
                    .try_into()
                    .unwrap(),
            );

            let row_start = PAGE_SIZE - row_length as usize - row_offset as usize;
            let row_end = row_start + row_length as usize;
            let row_bytes = &page_data[row_start..row_end];
            let mut decoded_row: Vec<String> = Vec::new();
            if action == "delete" {
                // do a where check on the query passed in before, also this should not be "decoded row"
                let _ = delete_row(curr_slot_offset, &file)?;
            } else {
                decoded_row = decode_row(row_bytes, &schema, &selected_column_ids)?;
            }

            println!("{:?}", decoded_row);
        }
    }

    Ok(())
}

pub fn decode_row(
    row_bytes: &[u8],
    schema: &TableMetadataObject,
    selected_column_ids: &Vec<u8>,
) -> Result<Vec<String>, String> {
    let header_size = row_bytes[0] as usize;
    let num_columns = row_bytes[1] as usize;
    let col_lengths = &row_bytes[2..2 + num_columns];
    let mut data_offset = header_size;

    let mut values: Vec<String> = Vec::new();

    for (i, field) in schema.fields.iter().enumerate() {
        let len = col_lengths[i] as usize;
        let field_bytes = &row_bytes[data_offset..data_offset + len];

        if selected_column_ids.contains(&(i as u8)) {
            let value = match field.data_type {
                FieldTypesAllowed::I8 => (field_bytes[0] as i8).to_string(),

                FieldTypesAllowed::I32 => {
                    let arr: [u8; 4] = field_bytes.try_into().map_err(|_| "corrupt I32")?;
                    i32::from_le_bytes(arr).to_string()
                }

                FieldTypesAllowed::String => String::from_utf8(field_bytes.to_vec())
                    .map_err(|_| format!("invalid UTF-8 in field '{}'", field.name))?,
            };

            values.push(value);
        }

        data_offset += len;
    }

    Ok(values)
}

pub fn build_row_byte(
    schema: &TableMetadataObject,
    values: &[ValueTypes],
) -> Result<Vec<u8>, String> {
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
    let row_header_size = 2 + num_columns;

    result.push(row_header_size as u8);
    result.push(num_columns as u8);
    result.extend(row_header);
    result.extend(row_data);

    Ok(result)
}

pub fn delete_row(row_slot_offset: u8, file: &File) -> Result<(), String> {
    return Ok(());
}
