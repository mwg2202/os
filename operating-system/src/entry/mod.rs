pub mod logger;
pub mod memory_map;

use memory_map::*;
use logger::*;

use uefi::prelude::*;
use uefi::table::cfg::{ACPI_GUID, ACPI2_GUID};
use uefi::table::boot::MemoryDescriptor;
use core::fmt::Debug;
use core::fmt::Write;
use rsdp::Rsdp;
use super::kernel;
use super::kernel::{SystemHandles, Error};
use super::ALLOCATOR;
use alloc::vec;
use alloc::vec::Vec;
use alloc::format;
use alloc::string::ToString;
use log::{Record, Level, Metadata, LevelFilter, info, debug, trace};
use core::mem::size_of;

#[entry]
fn efi_main(image: uefi::Handle, st: SystemTable<Boot>) -> Status {

    // Set up the allocator
    unsafe { ALLOCATOR.lock().init(100000, 100000); }

    // Initialize UEFI text services and logging
    init_logging(st);

    info!("Loading OS...");
    
    debug!("Getting Handles to System Tables");
    let sys_handles = get_handles();

    debug!("Getting UEFI Memory Map");
    let (mmap, size) = get_memory_map().unwrap();
    trace!("Map Size: {:?}", &size);
    trace!("Descriptor Size: {:?}", &size_of::<MemoryDescriptor>());
    trace!("{}", &count_descriptors(&mmap));

    // Getting the UEFI Memory Map
    // info!("Exiting UEFI");
    // let mmap = exit_uefi(image).unwrap();
    // read_memory_map(&mmap);

    // Start the kernel
    debug!("Starting the kernel");
    kernel::start(sys_handles, mmap);
}

fn init_logging(st: SystemTable<Boot>) {

    // Reset the console
    st.stdout()
        .reset(false)
        .expect_success("Failed to reset output buffer");
    
    // Save the system table as a global variable
    unsafe { ST = Some(st) };

    // Set the default logger
    log::set_logger(&UEFI_LOGGER)
        .map(|()| log::set_max_level(LevelFilter::Trace));

}

fn get_handles() -> SystemHandles {
    if let Some(st) = unsafe { ST.as_ref() } {
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

    } else { panic!("Could not get system handles") }
}

pub fn _crash(string: &dyn Debug) -> ! {
    if let Some(st) = unsafe { ST.as_ref() } {
        writeln!(st.stdout(), "FATAL ERROR: {:?}", string).unwrap();
    }
    loop {}
}

pub static mut ST: Option<SystemTable<Boot>> = None;
