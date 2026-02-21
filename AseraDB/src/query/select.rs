use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};

use sqlparser::ast::Table;

use crate::{
    core::{
        FieldTypesAllowed, PAGE_HEADER_SIZE_ON_CREATE, PAGE_HEADER_SLOT_SIZE_FOR_ROW, PAGE_SIZE,
        Page, QueryObject, TableMetadataObject, ValueTypes,
    },
    parsing::get_table_schema,
};

pub fn read_data(query: &mut QueryObject) -> Result<(), String> {
    let schema: TableMetadataObject = get_table_schema(&query.table)?;
    let schema_path = format!("database/tables/{}.asera", &query.table);
    let file: File = File::open(&schema_path).map_err(|e| e.to_string())?;
    let file_length = file.metadata().map_err(|e| e.to_string())?.len();
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

    for curr_page_id in 1..num_pages {
        file.seek(SeekFrom::Start(curr_page_id * PAGE_SIZE as u64))
            .map_err(|e| e.to_string())?;

        file.read_exact(&mut page_data).map_err(|e| e.to_string())?;
    }

    Ok(())
}
