byte page information:
extension: .asera, example "profile.asera". Contains many pages.
page size: 4096 bytes
page header:
{
(byte 0) page id: defines the offset of the page in the file
(byte 1) row count: defines the number of rows in the current page. Inc and Dec on insert / delete
(bytes 2-3) size of current rows: helps define the insert point for any new rows. IE page_size - this_value is where open space begins
(bytes 4-5) space remaining: total amount of space remaining on this page. Update on insert/delete
(byte 6) Last Sequence Number (LSN): Indicates last WAL record applied to the page
(bytes 7-8) Size of page header: describes the size of the page header, which will change with each added and removed row
(byte 9): Total Free Space: DIFFERENT than space remaining. This tracks deletions. If it gets to a certain size, readjust page.
Rest of bytes: Slots / Data
}
