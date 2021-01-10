#![feature(abi_x86_interrupt)]

#[path = ".."]
mod graphics;

use x86_64::structures::idt::InterruptDescriptorTable;
use graphics::GraphicsSystem;
use lazy_static::lazy_static;

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(
    stack_frame: &mut InterruptStackFrame) {
    fill_screen()
}
lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt
    };
}
