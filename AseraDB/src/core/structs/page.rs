pub const PAGE_SIZE: usize = 4096;

pub struct Page {
    pub id: u64,
    pub data: [u8; PAGE_SIZE],
    pub dirty: bool,
    pub pin_count: usize,
}
