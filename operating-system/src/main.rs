#![no_main]
#![feature(abi_efiapi)]
#![no_std]
#![feature(asm)]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]
#![feature(panic_info_message)]
// #![allow(dead_code)]
extern crate alloc;

mod entry;
mod kernel;

use core::alloc::Layout;
use linked_list_allocator::LockedHeap;
use core::panic::PanicInfo;
use log::error;

#[panic_handler]
fn panic(i: &PanicInfo) -> ! {
    error!("{:?}", i.message());
    loop{}
}

#[global_allocator]
static mut ALLOCATOR: LockedHeap = LockedHeap::empty();


#[alloc_error_handler]
fn alloc_error_handler(layout: Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}
