#![no_main]
use multiboot2::{load, BootInformation};

mod kernel;
use kernel::system::SystemHandles;

#[no_mangle]
fn _start(image: uefi::Handle, st: SystemTable<Boot>) -> Status {
    let multiboot_info_ptr: u32;
    unsafe { asm!("mov $2, %ebx" : "=r"(multiboot_info_ptr)) };
    let boot_info = unsafe { load(multiboot_info_ptr) };

    // Gets a list of handles to system tables
    let sys_handles = get_handles(&boot_info);

    loop {}
}

fn get_handles(bi: BootInformation) -> SystemHandles {
    // Create a struct to hold the system info
    SystemHandles {
        acpi: bi.rsdp_v2_tag().map(|r| unsafe { &*(&r as *const Rsdp) }),
        acpi2: bi.rsdp_v1_tag().map(|r| unsafe { &*(&r as *const Rsdp) }),
    };
}
