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
(byte 9-10): Total Freed space: DIFFERENT than space remaining. This tracks deletions. If it gets to a certain size, readjust page.
Rest of bytes: Slots / Data

Slots are handled with 2 bytes of length, and then 2 bytes of offset from BACK of the page (ie, first row offset of 0).
if a slot is open, offset is set to 0xFFFF, this is so that length can be found still to find open spots for new rows.
in the event this happens, we insert with O(n) by scanning the page from the previous slot to find the new opening in order.
this will be optimized in the future.
}
