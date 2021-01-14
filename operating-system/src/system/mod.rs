use uefi::prelude::*;
use rsdp::Rsdp;
use uefi::table::cfg::{ACPI2_GUID, ACPI_GUID};
use log::error;
use log::info;

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

pub fn check_compatability(st: &SystemTable<Boot>, sys_info: &SystemInfo) {
    if sys_info.acpi.is_none() && sys_info.acpi2.is_none() {
        error!("The ACPI GUID was not found");
        shutdown_on_keypress(&st);
    }

    let mut rsdp;
    use rsdp::RsdpError::*;
    if sys_info.acpi.is_none() {
        info!("Using ACPI v1");
        rsdp = sys_info.acpi.unwrap();
    } else {
        info!("Using ACPI v2");
        rsdp = sys_info.acpi2.unwrap();
    }

    match rsdp.validate() {
        Err(NoValidRsdp) => {
            error!("No Valid RSDP");
            shutdown_on_keypress(&st);
        },
        Err(IncorrectSignature) => {
            error!("Incorrect Signature found in RSDP");
            shutdown_on_keypress(&st);
        },
        Err(InvalidOemId) => {
            error!("Invalid OEM ID found in RSDP");
            shutdown_on_keypress(&st);
        },
        Err(InvalidChecksum) => {
            error!("Invalid checksum found in RSDP");
            shutdown_on_keypress(&st);
        },
        Ok(_) => (),
    }
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
