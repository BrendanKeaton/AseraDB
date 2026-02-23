use crate::{
    core::{FieldTypesAllowed, PAGE_SIZE, QueryObject, TableMetadataObject, ValueTypes},
    parsing::get_table_schema,
};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

pub fn read_data(query: &mut QueryObject) -> Result<(), String> {
    let schema: TableMetadataObject = get_table_schema(&query.table)?;
    let schema_path = format!("database/tables/{}.asera", &query.table);
    let file: File = File::open(&schema_path).map_err(|e| e.to_string())?;
    let file_length = file.metadata().map_err(|e| e.to_string())?.len();
    let _ = read_sequential(query, file, file_length, schema);
    Ok(())
}

pub fn read_sequential(
    query: &QueryObject,
    mut file: File,
    file_len: u64,
    schema: TableMetadataObject,
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

            let decoded_row = decode_row(row_bytes, &schema, &selected_column_ids)?;
            println!("{:?}", decoded_row);
        }
    }

    Ok(())
}

fn decode_row(
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

fn get_selected_column_ids(
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
