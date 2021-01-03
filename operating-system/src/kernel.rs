#![no_std]
#![no_main]
#![feature(asm)]
use core::panic::PanicInfo;
mod vga;

#[no_mangle]
pub unsafe fn _start(st: &SystemTable) {
    (st.boot_services().stdout(), "Hello World");
    // let mut vga_handler = vga::HANDLER.lock();
    // vga_handler.write_str("Hello World!");
    loop{}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

//Since the bootloader could load the kernel at any point in memory we should choose a virtual
//address that will be set as an entry point at the compilation stage
