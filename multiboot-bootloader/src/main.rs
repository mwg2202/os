#![no_std]

use multiboot2::load;
let multiboot_info_ptr: u32;
unsafe { asm!("mov $2, %ebx" : "+r"(multiboot_info_ptr)) };

fn main() {
    
}
