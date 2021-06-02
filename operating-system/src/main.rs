#![no_main]
#![feature(abi_efiapi)]
#![no_std]
#![feature(asm)]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]
#![feature(panic_info_message)]
#![feature(min_type_alias_impl_trait)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_must_use)]
extern crate alloc;

mod uefi;
mod mb2;

mod kernel;

use core::fmt::Debug;
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
