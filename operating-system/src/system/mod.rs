// pub mod gdt;
// pub mod apic;
// pub mod interrupts;
mod acpi_methods;

pub use acpi_methods::*;
use rsdp::Rsdp;
use crate::ST;
use uefi::table::cfg::{ACPI_GUID, ACPI2_GUID};

pub struct SystemHandles {
    pub acpi: Option<&'static Rsdp>,
    pub acpi2: Option<&'static Rsdp>,
} impl SystemHandles {

    /// Get system handles using the system table global variable
    pub fn get() -> SystemHandles {
        if let Some(st) = unsafe { ST.as_ref() } {
            
            // Create a struct to hold the system info
            let mut sys_handles = SystemHandles {
                acpi: None,
                acpi2: None,
            };

            // Gather system information from the config table
            for e in st.config_table() {
                match e.guid {
                    ACPI_GUID => sys_handles.acpi = {
                        Some(unsafe { &*(e.address as *const Rsdp) })
                    },
                    ACPI2_GUID => sys_handles.acpi2 = {
                        Some(unsafe { &*(e.address as *const Rsdp) })
                    },
                    _ => ()
                }
            }

            // Return the system info
            sys_handles

        } else { panic!("Could not get system handles") }
    }
}


use acpi::AcpiError;
use aml::AmlError;
#[derive(Debug)]
pub enum Error {
    RsdpNotFound,
    // CouldNotFindApic,
    // CouldNotFindSystemFont,
    // NoAcpiTables,
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
