use rsdp::Rsdp;
use acpi::AcpiError;
//pub mod gdt;
//pub mod apic;
//pub mod interrupts;

pub struct SystemHandles {
    pub acpi: Option<&'static Rsdp>,
    pub acpi2: Option<&'static Rsdp>,
}


#[derive(Debug)]
pub enum Errors {
    AcpiHandleNotFound,
    CouldNotFindApic,
    CouldNotFindSystemFont,
    AcpiError(AcpiError),
}
impl From<AcpiError> for Errors {
    fn from(orig: AcpiError) -> Self {
        Self::AcpiError(orig)
    }
}
