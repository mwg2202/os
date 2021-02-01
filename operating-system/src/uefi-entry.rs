#![no_main]
#![feature(abi_efiapi)]
#![no_std]
#![feature(asm)]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]

use uefi::prelude::*;
use uefi::table::cfg::{ACPI2_GUID, ACPI_GUID};
use rsdp::Rsdp;

mod kernel;
use kernel::system::SystemHandles;

#[entry]
fn efi_main(image: uefi::Handle, st: SystemTable<Boot>) -> Status {
    
    // Reset the console
    st.stdout()
        .reset(false)
       .expect_success("Failed to reset output buffer");
    
    // Gets a list of handles to system tables
    let sys_handles = get_handles(&st);
    kernel::start(sys_handles);
    loop {}
}

fn get_handles(st: &SystemTable<Boot>) -> SystemHandles {
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
