#![no_std]
#![no_main]
#![feature(asm)]
#![feature(abi_efiapi)]

extern crate alloc;
use uefi::prelude::*;
use core::mem;
use alloc::vec;
use uefi::table::boot::MemoryDescriptor;

#[entry]
fn efi_main(image: uefi::Handle, st: SystemTable<Boot>) -> Status {
    
    // Initialize utilities (logging, memory allocation...)
    uefi_services::init(&st).expect_success("Failed to initialize utilities");

    // Reset the console
    st.stdout()
        .reset(false)
        .expect_success("Failed to reset output buffer"); 

    // Exit boot services
    exit_boot_services(st);
    
    loop {}
}

/// Exits boot services
fn exit_boot_services(st: &SystemTable<Boot>) -> Result {
    let max_mmap_size = st.boot_services().memory_map_size() + 
        8 * mem::size_of::<MemoryDescriptor>();
    let mut mmap_storage = vec![0; max_mmap_size].into_boxed_slice();
    let (st, _iter) = st.exit_boot_services(image, &mut mmap_storage[..])
        .expect_success("Failed to exit boot services");
}
