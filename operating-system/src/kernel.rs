#![no_std]
#![no_main]
#![feature(asm)]
use core::panic::PanicInfo;

#[no_mangle]
pub unsafe fn _start() {
    loop {}
}

/*#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
*/
