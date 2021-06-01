#![no_main]
#![feature(abi_efiapi)]
#![no_std]
#![feature(asm)]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]
#![feature(panic_info_message)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_must_use)]

mod uefi;
mod mb2;

mod kernel;

use core::fmt::Debug;

#[allow(non_upper_case_globals)]
const log: fn(text: &str) = _return;

#[allow(non_upper_case_globals)]
const info: fn(text: &str) = _return;

#[allow(non_upper_case_globals)]
const error: fn(text: &str) = _return;

#[allow(non_upper_case_globals)]
const crash: fn(text: &dyn Debug) -> ! = _loop;

use core::panic::PanicInfo;
#[panic_handler]
fn panic(i: &PanicInfo) -> ! {
    crash(&i.message());
}

fn _return(_: &str) { return; }
fn _loop(_: &dyn Debug) -> ! { loop {} }
