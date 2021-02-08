#![no_main]
#![feature(abi_efiapi)]
#![no_std]
#![feature(asm)]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]

#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

extern crate alloc;
use alloc::vec;
use uefi::prelude::*;
use uefi::table::cfg::{ACPI2_GUID, ACPI_GUID};
use uefi::table::boot::MemoryDescriptor;
use rsdp::Rsdp;
use spin::Mutex;

mod kernel;
use kernel::{SystemHandles, Error};
use core::fmt::Write;
use core::mem::size_of;

#[entry]
fn efi_main(image: uefi::Handle, st: SystemTable<Boot>) -> Status {
    // Reset the console
    st.stdout()
        .reset(false)
        .expect_success("Failed to reset output buffer");
    
    write!(st.stdout(), "Hello World!");

    // Gets a list of handles to system tables
    let sys_handles = get_handles(&st);
    
    
    // Save the system table as a global variable
    unsafe {ST = Some(st)};
    unsafe {IMAGE = Some(image)};
    
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
pub fn crash(err: Error) -> ! {
    if let Some(st) = unsafe {ST.as_ref()} {
        write!(st.stdout(), "FATAL ERROR: {:?}", err).unwrap();
    }
    loop {}
}

static mut ST: Option<SystemTable<Boot>> = None;
static mut IMAGE: Option<uefi::Handle> = None;
