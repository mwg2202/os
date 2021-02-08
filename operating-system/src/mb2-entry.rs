#![no_main]
#![no_std]
#![feature(asm)]
#![feature(alloc_error_handler)]

use multiboot2::{load, BootInformation, RsdpV1Tag, RsdpV2Tag};
use rsdp::Rsdp;

mod kernel;
use kernel::{SystemHandles, Error};

#[no_mangle]
fn _start() {
    let mb2_info_ptr: u32;
    unsafe { asm!("mov {0}, %ebx", out(reg) mb2_info_ptr) };
    let boot_info = unsafe { load(mb2_info_ptr as usize) };

    // Gets a list of handles to system tables
    let sys_handles = get_handles(&boot_info);

    loop {}
}

fn get_handles(bi: &BootInformation) -> SystemHandles {
    // Create a struct to hold the system info
    SystemHandles {
        acpi: bi.rsdp_v1_tag().map(|r| unsafe { &*(r as *const RsdpV1Tag as *const Rsdp) }),
        acpi2: bi.rsdp_v2_tag().map(|r| unsafe { &*(r as *const RsdpV2Tag as *const Rsdp) }),
    }
}

pub fn crash(_err: Error) -> ! {
	loop{}
}

pub fn get_mmap() {

}
