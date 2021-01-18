use log::info;
use crate::system::gdt;

use x86_64::structures::idt::{
    InterruptDescriptorTable, 
    InterruptStackFrame,
    PageFaultErrorCode,
};
use lazy_static::lazy_static;

/// Enables interrupts and sets up the IDT
pub fn enable() {
    IDT.load();
}

extern "x86-interrupt" fn divide_error_handler(
    stack_frame: &mut InterruptStackFrame) {
    info!("divide_error");
}

extern "x86-interrupt" fn debug_handler(
    stack_frame: &mut InterruptStackFrame) {
    info!("debug");
}

extern "x86-interrupt" fn non_maskable_interrupt_handler(
    stack_frame: &mut InterruptStackFrame) {
    info!("non_maskable_interrupt");
}

extern "x86-interrupt" fn breakpoint_handler(
    stack_frame: &mut InterruptStackFrame) {
    info!("breakpoint");
}

extern "x86-interrupt" fn overflow_handler(
    stack_frame: &mut InterruptStackFrame) {
    info!("overflow");
}

extern "x86-interrupt" fn bound_range_exceeded_handler(
    stack_frame: &mut InterruptStackFrame) {
    info!("bound_range_exceeded");
}

extern "x86-interrupt" fn invalid_opcode_handler(
    stack_frame: &mut InterruptStackFrame) {
    info!("invalid_opcode");
}

extern "x86-interrupt" fn device_not_available_handler(
    stack_frame: &mut InterruptStackFrame) {
    info!("device_not_available");
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: &mut InterruptStackFrame, error_code: u64) -> ! {
    info!("double_fault");
    loop{}
}

extern "x86-interrupt" fn invalid_tss_handler(
    stack_frame: &mut InterruptStackFrame, error_code: u64) {
    info!("invalid_tss");
}

extern "x86-interrupt" fn segment_not_present_handler(
    stack_frame: &mut InterruptStackFrame, error_code: u64) {
    info!("segment_not_present: err_code={}", error_code);
    info!("InterruptStackFrame: {:?}", stack_frame);
}

extern "x86-interrupt" fn stack_segment_fault_handler(
    stack_frame: &mut InterruptStackFrame, error_code: u64) {
    info!("stack_segment_fault");
}

extern "x86-interrupt" fn general_protection_fault_handler(
    stack_frame: &mut InterruptStackFrame, error_code: u64) {
    info!("general_protection_fault: err_code={}", error_code);
}

extern "x86-interrupt" fn page_fault_handler(
    stack_frame: &mut InterruptStackFrame, error_code: PageFaultErrorCode) {
    info!("page_fault");
}

extern "x86-interrupt" fn x87_floating_point_handler(
    stack_frame: &mut InterruptStackFrame) {
    info!("x87_floating_point");
}

extern "x86-interrupt" fn alignment_check_handler(
    stack_frame: &mut InterruptStackFrame, error_code: u64) {
    info!("alignment_check");
}

extern "x86-interrupt" fn machine_check_handler(
    stack_frame: &mut InterruptStackFrame) -> ! {
    info!("machine_check");
    loop {}
}

extern "x86-interrupt" fn simd_floating_point_handler(
    stack_frame: &mut InterruptStackFrame) {
    info!("simd_floating_point");
}

extern "x86-interrupt" fn virtualization_handler(
    stack_frame: &mut InterruptStackFrame) {
    info!("virtualization");
}

extern "x86-interrupt" fn security_exception_handler(
    stack_frame: &mut InterruptStackFrame, error_code: u64) {
    info!("security_exception");
}

const SYSCALL: usize = 0x80;
const TIMER: usize = 0x81;
const APIC_ERROR: usize = 0x82;
const SPURIOUS_VECTOR: usize = 0xff;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.divide_error.set_handler_fn(divide_error_handler);
        idt.debug.set_handler_fn(debug_handler);
        idt.non_maskable_interrupt.set_handler_fn(non_maskable_interrupt_handler);
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.overflow.set_handler_fn(overflow_handler);
        idt.bound_range_exceeded.set_handler_fn(bound_range_exceeded_handler);
        idt.invalid_opcode.set_handler_fn(invalid_opcode_handler);
        idt.device_not_available.set_handler_fn(device_not_available_handler);
        unsafe {
            idt.double_fault.set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }
        idt.invalid_tss.set_handler_fn(invalid_tss_handler);
        idt.segment_not_present.set_handler_fn(segment_not_present_handler);
        idt.stack_segment_fault.set_handler_fn(stack_segment_fault_handler);
        idt.general_protection_fault.set_handler_fn(general_protection_fault_handler);
        idt.page_fault.set_handler_fn(page_fault_handler);
        idt.x87_floating_point.set_handler_fn(x87_floating_point_handler);
        idt.alignment_check.set_handler_fn(alignment_check_handler);
        idt.machine_check.set_handler_fn(machine_check_handler);
        idt.simd_floating_point.set_handler_fn(simd_floating_point_handler);
        idt.virtualization.set_handler_fn(virtualization_handler);
        idt.security_exception.set_handler_fn(security_exception_handler);
        idt[SYSCALL].set_handler_fn(syscall_handler);
        idt[TIMER].set_handler_fn(timer_handler);
        idt[APIC_ERROR].set_handler_fn(apic_error_handler);
        idt[SPURIOUS_VECTOR].set_handler_fn(spurious_vector_handler);

        idt
    };
}

extern "x86-interrupt" fn syscall_handler(stack_frame: &mut InterruptStackFrame) {
    info!("syscall");
}
extern "x86-interrupt" fn timer_handler(stack_frame: &mut InterruptStackFrame) {
    info!("timer");
}
extern "x86-interrupt" fn apic_error_handler(stack_frame: &mut InterruptStackFrame) {
    info!("apic_error");
}
extern "x86-interrupt" fn spurious_vector_handler(stack_frame: &mut InterruptStackFrame) {
    info!("spurious_vector");
}
