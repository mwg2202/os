pub const RSDP_SIGNATURE: &'static [u8; 8] = b"RSD PTR ";
pub const RSDT_SIGNATURE: &'static [u8; 4] = b"RSDT";
pub const XSDT_SIGNATURE: &'static [u8; 4] = b"XSDT";
pub const FADT_SIGNATURE: &'static [u8; 4] = b"FACP";
pub const HPET_SIGNATURE: &'static [u8; 4] = b"HPET";
pub const MADT_SIGNATURE: &'static [u8; 4] = b"APIC";
pub const MCFG_SIGNATURE: &'static [u8; 4] = b"MCFG";
pub const SSDT_SIGNATURE: &'static [u8; 4] = b"SSDT";

#[derive(Clone, Copy, Debug)]
#[repr(C, packed)]
pub struct SdtHeader {
    pub signature: Signature,
    pub length: u32,
    pub revision: u8,
    pub checksum: u8,
    pub oem_id: [u8; 6],
    pub oem_table_id: [u8; 8],
    pub oem_revision: u32,
    pub creator_id: u32,
    pub creator_revision: u32,
}
impl SdtHeader {
    pub fn validate(
        &self, 
        signature: Signature
    ) -> Result<(), AcpiError> {
        // Check the signature
        if self.signature != signature {
            return Err(AcpiError::SdtInvalidSignature(signature));
        }

        // Check the OEM id
        if str::from_utf8(&self.oem_id).is_err() {
            return Err(AcpiError::SdtInvalidOemId(signature));
        }

        // Check the OEM table id
        if str::from_utf8(&self.oem_table_id).is_err() {
            return Err(AcpiError::SdtInvalidTableId(signature));
        }

        // Validate the checksum
        let self_ptr = self as *const SdtHeader as *const u8;
        let mut sum: u8 = 0;
        for i in 0..self.length {
            sum = sum.wrapping_add(
                unsafe { *(self_ptr.offset(i as isize)) } as u8
            );
        }

        if sum > 0 {
            return Err(AcpiError::SdtInvalidChecksum(signature));
        }

        Ok(())
    }



}
