use uefi::prelude::*;
use rsdp::Rsdp;
use acpi::{PhysicalMapping, AcpiTables};
use uefi::table::cfg::{ACPI2_GUID, ACPI_GUID};
use log::{error, info};

pub struct SystemInfo {
    acpi: Option<&'static Rsdp>,
    acpi2: Option<&'static Rsdp>,
}

// For some reason, the locate_handle function of the UEFI crate was not finding
// the RSDP for ACPI. So this function searches through the UEFI config table to
// find it
pub fn get_system_info(st: &SystemTable<Boot>) -> SystemInfo {
    
    // Create a struct to hold the system info
    let mut sys_info = SystemInfo {
        acpi: None,
        acpi2: None,
    };

    // Get the UEFI config table
    let cfg_table = st.config_table();
    
    // Gather system information from the config table
    for entry in cfg_table {
        if entry.guid == ACPI_GUID { 
            sys_info.acpi = Some(unsafe { core::mem::transmute(entry.address) });
        }
        if entry.guid == ACPI2_GUID { 
            sys_info.acpi2 = Some(unsafe { core::mem::transmute(entry.address) }); 
        }
    }

    // Return the system info
    sys_info
}

/// Checks the system to make sure that it is compatible with the os
pub fn check_compatability(st: &SystemTable<Boot>, sys_info: &SystemInfo) {
    if sys_info.acpi.is_none() && sys_info.acpi2.is_none() {
        error!("The ACPI GUID was not found");
        shutdown_on_keypress(&st);
    }

    let rsdp;
    if sys_info.acpi2.is_none() {
        info!("Using ACPI v1");
        rsdp = sys_info.acpi.unwrap();
    } else {
        info!("Using ACPI v2");
        rsdp = sys_info.acpi2.unwrap();
    }

    let acpi_table: AcpiTables<AcpiHandlerStruct>;
    let handler: AcpiHandlerStruct = AcpiHandlerStruct {number: 5};
    match unsafe { AcpiTables::from_rsdp(handler, (rsdp as *const Rsdp) as usize) } {
        Err(error) => {
            error!("{:?}", error);
            shutdown_on_keypress(&st);
        },
        Ok(table) => acpi_table = table,
    }

    match acpi_table.platform_info() {
        Err(error) => error!("Error getting platform info: {:?}", error),
        Ok(platform_info) => {
            info!("Power Platform: {:?}", platform_info.power_profile);
            match platform_info.interrupt_model {
                acpi::InterruptModel::Apic(apic) => {
                    info!("Apic Found: {:?}", apic);
                    if apic.also_has_legacy_pics {
                        // Remap and mask all the lines of the legacy PIC.
                    }
                }
                Unknown => {
                    error!("Could not find APIC details on system. Most likely only the legacy i8259 PIC is present. The legacy i8259 PIC is currently not supported.");
                    shutdown_on_keypress(&st);
                },
            }
        }
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
