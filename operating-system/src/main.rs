#![no_main]
#![feature(abi_efiapi)]
#![no_std]
#![feature(asm)]
#![feature(abi_x86_interrupt)]
#![feature(allocator_api)]
#![feature(alloc_error_handler)]
#![feature(panic_info_message)]
#![feature(min_type_alias_impl_trait)]
#![allow(unreachable_code)]
extern crate alloc;

mod kernel;
mod logging;
mod memory;
mod system;
// mod graphics;
// mod filesystem;
// mod system;

use core::alloc::Layout;
use core::panic::PanicInfo;
use log::{info, debug, error};
use uefi::prelude::*;
use logging::UefiLogger;
use system::SystemHandles;
use memory::frame_allocator::FRAME_ALLOCATOR;
use memory::allocator::Allocator;

pub static mut ST: Option<SystemTable<Boot>> = None;

#[panic_handler]
fn panic(i: &PanicInfo) -> ! {
    error!("{:?}", i.message());
    loop{}
}


#[alloc_error_handler]
fn alloc_error_handler(layout: Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}

#[entry]
fn efi_main(_image: uefi::Handle, st: SystemTable<Boot>) -> Status {
    
    // Save the system table as a global variable
    unsafe { ST = Some(st) };

    // Initialize UEFI text services and logging
    UefiLogger::init();
    
    info!("Loading OS...");
    
    info!("Initializing the Global Frame Allocator");
    unsafe { FRAME_ALLOCATOR.init(); }

    // info!("Initializing the Global Allocator");
    // Allocator::init();
    
    debug!("Getting Handles to System Tables");
    let sys_handles = SystemHandles::get();

    // info!("Exiting UEFI");
    // let mmap = exit_uefi(image).unwrap();
    // read_memory_map(&mmap);

    // Start the kernel
    debug!("Starting the kernel");
    kernel::start(sys_handles);
}

// /// Ends UEFI Boot Services and returns a final memory map
// pub fn exit_uefi(image: uefi::Handle) -> Result<MemoryMap, ()> {
//     let mut mmap_storage = {
//         if let Some(st) = unsafe { ST.as_ref() } {
//             let max_mmap_size =
//                 st.boot_services().memory_map_size() 
//                 + 8 * size_of::<MemoryDescriptor>();

//             info!("{:?}", &max_mmap_size); // REMOVE LATER
//             info!("{:?}", &size_of::<MemoryDescriptor>());

//             vec![0; max_mmap_size].into_boxed_slice()
//         } else { return Err(()) }
//     };

//     if let Some(st) = unsafe { ST.take() } {
//         let (_st, iter) = st
//             .exit_boot_services(image, &mut mmap_storage[..])
//             .expect_success("Failed to exit boot services");
        
//         Ok(iter.copied().collect())
//     } else { Err(()) }
// }
