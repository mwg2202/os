use uefi::prelude::*;
use uefi::table::cfg::{ACPI_GUID, ACPI2_GUID};
use core::fmt::Debug;
use core::fmt::Write;
use rsdp::Rsdp;
use super::kernel;
use super::kernel::{SystemHandles, Error};

#[entry]
fn efi_main(image: uefi::Handle, st: SystemTable<Boot>) -> Status {
    // Reset the console
    st.stdout()
        .reset(false)
        .expect_success("Failed to reset output buffer");

    writeln!(st.stdout(), "Loading OS...");

    // Gets a list of handles to system tables
    let sys_handles = get_handles(&st);
    info("Obtained system table handles");

    // Save the system table as a global variable
    unsafe { ST = Some(st) };
    unsafe { IMAGE = Some(image) };

    info("Starting the kernel");

    // Start the kernel
    kernel::start(sys_handles);
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
        match entry.guid {
            ACPI_GUID => sys_handles.acpi = {
                Some(unsafe { &*(entry.address as *const Rsdp) })
            },
            ACPI2_GUID => sys_handles.acpi2 = {
                Some(unsafe { &*(entry.address as *const Rsdp) })
            },
            _ => ()
        }
    }

    // Return the system info
    sys_handles
}

pub fn crash(err: Error) -> ! {
    if let Some(st) = unsafe { ST.as_ref() } {
        writeln!(st.stdout(), "FATAL ERROR: {:?}", err).unwrap();
    }
    loop {}
}

pub fn info(string: &(impl Debug + ?Sized)) {
    if let Some(st) = unsafe { ST.as_ref() } {
        writeln!(st.stdout(), "INFO: {:?}", string).unwrap();
    }
}

static mut ST: Option<SystemTable<Boot>> = None;
static mut IMAGE: Option<uefi::Handle> = None;
