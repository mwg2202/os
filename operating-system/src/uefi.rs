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
use log::{Record, Level, Metadata, LevelFilter, info};
use core::mem::size_of;

#[entry]
fn efi_main(image: uefi::Handle, st: SystemTable<Boot>) -> Status {
    // Set up the allocator
    unsafe { ALLOCATOR.lock().init(100000, 100000); }

    // Reset the console
    st.stdout()
        .reset(false)
        .expect_success("Failed to reset output buffer");
    
    // Gets a list of handles to system tables
    let sys_handles = get_handles(&st);
    
    // Save the system table as a global variable
    unsafe { ST = Some(st) };

    // Set the default logger
    log::set_logger(&UEFI_LOGGER)
        .map(|()| log::set_max_level(LevelFilter::Trace));
   
    info!("Loading OS...");

    info!("Getting UEFI Memory Map");
    let mmap = get_memory_map();
    info!("{:?}", mmap);

    // Getting the UEFI Memory Map
    info!("Exiting UEFI");
    let mmap = exit_uefi(image);
    info!("{:?}", &mmap);

    // Start the kernel
    info!("Starting the kernel");
    kernel::start(sys_handles);
}

fn exit_uefi(image: Handle) -> Result<Vec<MemoryDescriptor>, ()> {
    let mut mmap_storage = {
        if let Some(st) = unsafe { ST.as_ref() } {
            let max_mmap_size =
                st.boot_services().memory_map_size() 
                + 8 * size_of::<MemoryDescriptor>();

            info!("{:?}", &max_mmap_size); // REMOVE LATER
            info!("{:?}", &size_of::<MemoryDescriptor>());

            vec![0; max_mmap_size].into_boxed_slice()
        } else { return Err(()) }
    };

    if let Some(st) = unsafe { ST.take() } {
        let (_st, iter) = st
            .exit_boot_services(image, &mut mmap_storage[..])
            .expect_success("Failed to exit boot services");
        
        Ok(iter.copied().collect())
    } else { Err(()) }
}

fn get_memory_map() -> Result<Vec<MemoryDescriptor>, ()> {
    if let Some(st) = unsafe { ST.as_ref() } {
        let max_mmap_size =
            st.boot_services().memory_map_size() 
            + 8 * size_of::<MemoryDescriptor>();

        info!("{:?}", &max_mmap_size); // REMOVE LATER
        info!("{:?}", &size_of::<MemoryDescriptor>());

        let mut mmap_storage 
            = vec![0; max_mmap_size].into_boxed_slice();

        let (_key, iter) = st
            .boot_services()
            .memory_map(&mut mmap_storage[..])
            .expect_success("Failed to get memory map");
            
        Ok(iter.copied().collect())
    } else { return Err(()) }
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

pub fn _crash(string: &dyn Debug) -> ! {
    if let Some(st) = unsafe { ST.as_ref() } {
        writeln!(st.stdout(), "FATAL ERROR: {:?}", string).unwrap();
    }
    loop {}
}

static mut ST: Option<SystemTable<Boot>> = None;


static UEFI_LOGGER: UefiLogger = UefiLogger;

struct UefiLogger;
impl log::Log for UefiLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if let Some(st) = unsafe { ST.as_ref() } {
            writeln!(
                st.stdout(),
                "{}: {}",
                record.level(),
                record.args()
            );
        }

        if record.level() == Level::Error {
            loop {}
        }
    }
    
    fn flush(&self) {}
}
