// pub mod gdt;
// pub mod apic;
// pub mod interrupts;
mod acpi_methods;
mod fadt;
pub use acpi_methods::*;
use rsdp::Rsdp;

pub struct SystemHandles {
    pub acpi: Option<&'static Rsdp>,
    pub acpi2: Option<&'static Rsdp>,
}

use acpi::AcpiError;
use aml::AmlError;
#[derive(Debug)]
pub enum Error {
    AcpiHandleNotFound,
    CouldNotFindApic,
    CouldNotFindSystemFont,
    NoAcpiTables,
    NoAmlContext,
    AcpiError(AcpiError),
    AmlError(AmlError),
}
impl From<AcpiError> for Error {
    fn from(orig: AcpiError) -> Self { Self::AcpiError(orig) }
}
impl From<AmlError> for Error {
    fn from(orig: AmlError) -> Self { Self::AmlError(orig) }
}
