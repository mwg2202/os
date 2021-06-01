// All standard UEFI tables begin with a common header
#[derive(Debug, Copy, Clone)]
pub struct Header {
    pub signature: u64,
    pub revision: u32,
    pub length: u32,
    pub crc: u32,
    _reserved: u32, // Must be zero
}
