use uefi::prelude::*;
use rsdp::Rsdp;
use acpi::{PhysicalMapping, AcpiTables, AcpiError};
use acpi::platform::{PlatformInfo, Apic};
use uefi::table::cfg::{ACPI2_GUID, ACPI_GUID};
use log::{info};

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
            sys_handles.acpi = Some(unsafe { core::mem::transmute(entry.address) });
        }
        if entry.guid == ACPI2_GUID { 
            sys_handles.acpi2 = Some(unsafe { core::mem::transmute(entry.address) }); 
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


/// Checks the system to make sure that it is compatible with the os
pub fn get_platform_info(sys_handles: &SystemHandles) 
                                    -> Result<PlatformInfo, Errors> {
    if sys_handles.acpi.is_none() && sys_handles.acpi2.is_none() {
        return Err(Errors::AcpiHandleNotFound);
    }

    let rsdp;
    if sys_handles.acpi2.is_none() {
        info!("Using ACPI v1");
        rsdp = sys_handles.acpi.unwrap();
    } else {
        info!("Using ACPI v2");
        rsdp = sys_handles.acpi2.unwrap();
    }

    let handler: AcpiHandlerStruct = AcpiHandlerStruct {number: 5};
    let acpi_tables: AcpiTables<AcpiHandlerStruct>;
    match unsafe { AcpiTables::from_rsdp(handler, (rsdp as *const Rsdp) as usize) } {
        Err(e) => return Err(Errors::AcpiError(e)),
        Ok(t) => acpi_tables = t,
    };

    match acpi_tables.platform_info() {
        Err(e) => Err(Errors::AcpiError(e)),
        Ok(platform_info) => Ok(platform_info),
    }
}

pub fn find_apic(platform_info: &PlatformInfo) -> Result<&Apic, Errors> {
    match &platform_info.interrupt_model {
        acpi::InterruptModel::Apic(apic) => {
            info!("Apic Found: {:?}", apic);
            Ok(apic)
        },
        _ => Err(Errors::CouldNotFindApic),
    }
}

#[derive(Clone, Copy)]
struct AcpiHandlerStruct {
    number: u8,
}
impl acpi::AcpiHandler for AcpiHandlerStruct {
    unsafe fn map_physical_region<T>(&self, physical_address: usize, 
        size: usize ) -> PhysicalMapping<Self, T> {
        PhysicalMapping {
            physical_start: physical_address,
            virtual_start: 
                core::ptr::NonNull::<T>::new_unchecked(physical_address as *mut T),
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

    st.runtime_services().reset(ResetType::Shutdown, Status::SUCCESS, None);
}

/// Shutsdown the device
pub fn shutdown(st: &SystemTable<Boot>) {
    use uefi::table::runtime::ResetType;
    st.runtime_services().reset(ResetType::Shutdown, Status::SUCCESS, None);    
}

use log::error;
pub fn crash(st: &SystemTable<Boot>, e: Errors) -> ! {
    error!("{:?}", e);
    shutdown_on_keypress(st);
}
