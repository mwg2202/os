use acpi::platform::{Apic, PlatformInfo};
use acpi::{AcpiError, AcpiTables, PhysicalMapping};
use log::info;
use rsdp::Rsdp;
use uefi::prelude::*;
use uefi::table::cfg::{ACPI2_GUID, ACPI_GUID};

pub struct SystemHandles {
    acpi: Option<&'static Rsdp>,
    acpi2: Option<&'static Rsdp>,
}

/// For some reason, the locate_handle function of the UEFI crate was not finding
/// the RSDP for ACPI. So this function searches through the UEFI config table to
/// find it and other handles.
pub fn get_handles(st: &SystemTable<Boot>) -> SystemHandles {
    // Create a struct to hold the system info
    let mut sys_handles = SystemHandles {
        acpi: None,
        acpi2: None,
    };

    // Get the UEFI config table
    let cfg_table = st.config_table();

    // Gather system information from the config table
    for entry in cfg_table {
        if entry.guid == ACPI_GUID {
            sys_handles.acpi = Some(unsafe { &*(entry.address as *const Rsdp) });
        }
        if entry.guid == ACPI2_GUID {
            sys_handles.acpi2 = Some(unsafe { &*(entry.address as *const Rsdp) });
        }
    }

    // Return the system info
    sys_handles
}

#[derive(Debug)]
pub enum Errors {
    AcpiHandleNotFound,
    CouldNotFindApic,
    AcpiError(AcpiError),
}
impl From<AcpiError> for Errors {
    fn from(orig: AcpiError) -> Self {
        Self::AcpiError(orig)
    }
}

/// Checks the system to make sure that it is compatible with the os
pub fn get_platform_info(sys_handles: &SystemHandles) -> Result<PlatformInfo, Errors> {
    let rsdp = sys_handles
        .acpi2
        .or(sys_handles.acpi)
        .ok_or(Errors::AcpiHandleNotFound)?;

    let handler = AcpiHandlerStruct { number: 0 };
    let acpi_tables = unsafe { AcpiTables::from_rsdp(handler, (rsdp as *const Rsdp) as usize) }?;

    Ok(acpi_tables.platform_info()?)
}

pub fn find_apic(platform_info: &PlatformInfo) -> Result<&Apic, Errors> {
    match &platform_info.interrupt_model {
        acpi::InterruptModel::Apic(apic) => {
            info!("Apic Found: {:?}", apic);
            Ok(apic)
        }
        _ => Err(Errors::CouldNotFindApic),
    }
}

#[derive(Clone, Copy)]
struct AcpiHandlerStruct {
    number: u8,
}
impl acpi::AcpiHandler for AcpiHandlerStruct {
    unsafe fn map_physical_region<T>(
        &self,
        physical_start: usize,
        size: usize,
    ) -> PhysicalMapping<Self, T> {
        use core::ptr::NonNull;

        PhysicalMapping {
            physical_start,
            virtual_start: NonNull::new_unchecked(physical_start as *mut T),
            region_length: core::mem::size_of::<T>(),
            mapped_length: core::mem::size_of::<T>(),
            handler: *self,
        }
    }

    fn unmap_physical_region<T>(&self, region: &PhysicalMapping<Self, T>) {}
}

/// Shutdowns the device after the next key press
pub fn shutdown_on_keypress(st: &SystemTable<Boot>) -> ! {
    use uefi::table::runtime::ResetType;

    st.boot_services()
        .wait_for_event(&mut [st.stdin().wait_for_key_event()])
        .unwrap_success();

    st.runtime_services()
        .reset(ResetType::Shutdown, Status::SUCCESS, None);
}

/// Shutsdown the device
pub fn shutdown(st: &SystemTable<Boot>) {
    use uefi::table::runtime::ResetType;
    st.runtime_services()
        .reset(ResetType::Shutdown, Status::SUCCESS, None);
}

use log::error;
pub fn crash(st: &SystemTable<Boot>, e: Errors) -> ! {
    error!("{:?}", e);
    shutdown_on_keypress(st);
}
