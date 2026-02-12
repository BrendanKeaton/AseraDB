pub const PAGE_SIZE: usize = 4096;

#[derive(Debug, Copy, Clone)]
pub struct Page {
    pub id: u64,
    pub data: [u8; PAGE_SIZE],
    pub dirty: bool,
    pub pin_count: usize,
}

impl Default for Page {
    fn default() -> Self {
        Self {
            id: 0,
            data: [0; PAGE_SIZE],
            dirty: true,
            pin_count: 0,
        }
    }
}
