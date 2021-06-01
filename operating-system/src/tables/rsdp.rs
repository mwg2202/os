const RSDP_V1_LENGTH: usize = 20;
const RSDP_SIGNATURE: &'static [u8; 8] = b"RSD PTR ";
        
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RsdpError {
    NoValidRsdp,
    IncorrectSignature,
    InvalidOemId,
    InvalidChecksum
}

#[derive(Clone, Copy, Debug)]
#[repr(C, packed)]
pub struct Rsdp {
    signature: [u8; 8],
    checksum: u8,
    oem_id: [u8; 6],
    revision: u8,
    rsdt_address: u32,

    // Only valid if ACPI Version 2.0 or greater
    length: u32,
    xsdt_address: u64,
    ext_checksum: u8,
    reserved: [u8; 3],
} impl Rsdp {
    pub fn validate(&self) -> Result<(), RsdpError> {
        // Check the signature
        if &self.signature != RSDP_SIGNATURE {
            return Err(RsdpError::IncorrectSignature);
        }

        // Check the OEM id is valid UTF8 (allows use of unwrap)
        if str::from_utf8(&self.oem_id).is_err() { 
            return Err(RsdpError::InvalidOemId);
        }

         // Determine the length of the table
        let length = if self.revision > 0 { self.length as usize } 
        else { RSDP_V1_LENGTH };

        // Get the Rsdp table as bytes
        let bytes = unsafe { 
            slice::from_raw_parts(
                self as *const Rsdp as *const u8, length
            ) 
        };

        // Calculate the sum of the bytes
        let sum = bytes.iter().fold(
            0u8, |sum, &byte| sum.wrapping_add(byte)
        );

        if sum != 0 { return Err(RsdpError::InvalidChecksum); }

        Ok(())
    }
}
