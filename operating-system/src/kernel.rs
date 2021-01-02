#![no_std]
#![no_main]
#![feature(asm)]
use core::panic::PanicInfo;

#[no_mangle]
pub unsafe fn _start() {
    exit(0);
}
pub unsafe fn exit(code: i32) -> ! {
    let syscall_number: u64 = 60;
    asm!(
        "syscall", 
        in("rax") syscall_number,
        in("rdi") code,
        options(noreturn)
    ); 
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
