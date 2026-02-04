# Rust SQL Database Project

This is a side project built to increase my understanding of SQL databases while learning Rust through hands-on implementation of core database concepts.

Currently there is a basic SQL Parser 80% implemented with support of commands like:

COMMAND: CREATE [table_name_string_only] [col_name]:[type]:[is_index]
COMMAND: INSERT [table_name] [value_for_column_1]:[value_for_column_2]:[etc]
COMMAND: SELECT [row_names_or_*] from [table_name]

types supports: i8, i32, string

Example flow:
CREATE profile id:i32:true email:string:false username:string:false
INSERT profile 1:user@gmail.com:user
SELECT \* from profile

Only create has _actual_ functionality currently. Currently it creates a catalog in /database/catalog/[table_name].json to be used later for items like:
Selecting rows that are valid,
Inserting with types that are valid,
future metadata

### Roadmap

Full WHERE clause support for SELECT
Page-based byte storage format (row + page layout)
Indexing on Create with B+Tree
Write-Ahead Log (WAL) implementation (Crash Recovery with Tests)
Buffer pool management
More "rules", IE table with no index listed defaults to first column
Delete Support
Benchmarking / Tests vs SQL Lite

# TO RUN:

cargo run
