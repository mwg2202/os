#![no_std]
#![no_main]
#![feature(asm)]
#![feature(abi_efiapi)]
#![allow(dead_code)]
#![allow(unused_variables)]

#![feature(abi_x86_interrupt)]
#![feature(convert_float_to_int)]

extern crate alloc;
use uefi::prelude::*;
mod graphics;
mod system;
use system::interrupts;
use graphics::{fonts, Color, BufferTrait, Size, Location, WindowManager, PixelFormat};
use system::Errors;

#[entry]
fn efi_main(image: uefi::Handle, st: SystemTable<Boot>) -> Status {
    // Initialize utilities (logging, memory allocation...)
    uefi_services::init(&st).expect_success("Failed to initialize utilities");

    // Reset the console
    st.stdout()
        .reset(false)
        .expect_success("Failed to reset output buffer");
    
    let sys_handles = system::get_handles(&st);
    
    let platform_info = match system::get_platform_info(&sys_handles) {
        Ok(i) => i,
        Err(e) => system::crash(&st, e),
    };

    let apic = match system::find_apic(&platform_info) {
        Ok(a) => a,
        Err(e) => system::crash(&st, e),
    };
    
    let system_font = match fonts::init() {
        Some(f) => f,
        None => system::crash(&st, Errors::CouldNotFindSystemFont),
    };

    let mut gb = graphics::Screen::init(&st.boot_services());
    let mut wm = WindowManager::new();

    //system::gdt::init();
    //interrupts::enable();
   
    gb.fill(Color::new(0, 255, 0));
    gb.write_text("System Successfully Loaded!", Location {x:50, y:50},
                  &system_font, 50.0, Color::new(255, 255, 255));
    
    wm.create_window(0, Size {width:100, height:100}, Location {x:100, y:100}, gb.fmt());
    wm.draw(&mut gb);
    
    // exit_boot_services(st, image);
    loop {}
}

/// Exits uefi boot services
fn exit_boot_services(st: SystemTable<Boot>, image: uefi::Handle) {
    use alloc::vec;
    use core::mem;
    use uefi::table::boot::MemoryDescriptor;
    let max_mmap_size =
        st.boot_services().memory_map_size() + 8 * mem::size_of::<MemoryDescriptor>();
    let mut mmap_storage = vec![0; max_mmap_size];
    let (st, _iter) = st
        .exit_boot_services(image, &mut mmap_storage)
        .expect_success("Failed to exit boot services");
}
