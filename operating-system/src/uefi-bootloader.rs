#![no_std]
#![no_main]
#![feature(asm)]
#![feature(abi_efiapi)]
#![allow(dead_code)]
#![allow(unused_variables)]

extern crate alloc;
use uefi::prelude::*;
mod graphics;
mod system;

#[entry]
fn efi_main(_image: uefi::Handle, st: SystemTable<Boot>) -> Status {
    // Initialize utilities (logging, memory allocation...)
    uefi_services::init(&st).expect_success("Failed to initialize utilities");

    // Reset the console
    st.stdout()
        .reset(false)
        .expect_success("Failed to reset output buffer");

    let bs = st.boot_services();
    let rs = st.runtime_services();
    let sys_info = system::get_system_info(&st);
    system::check_compatability(&st, &sys_info);
    system::shutdown_on_keypress(&st);
}

/// Exits uefi boot services
fn exit_boot_services(st: SystemTable<Boot>, image: uefi::Handle) {
    use uefi::table::boot::MemoryDescriptor;
    use core::mem;
    use alloc::vec;
    let max_mmap_size =
        st.boot_services().memory_map_size()+8*mem::size_of::<MemoryDescriptor>();
    let mut mmap_storage = vec![0; max_mmap_size].into_boxed_slice();
    let (st, _iter) = st
        .exit_boot_services(image, &mut mmap_storage[..])
        .expect_success("Failed to exit boot services");
}
