pub const EUSN_MAX_NAME_SIZE: usize = 300;
pub struct USN {
    pub data: u8,
    pub size: u32,
    pub drive_letter: char,
}
impl USN {
    pub fn new() -> Self {
        USN {
            data: 0,
            size: 0,
            drive_letter: 0 as char,
        }
    }
}
