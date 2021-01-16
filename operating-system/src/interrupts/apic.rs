pub fn init_lapic {
    use x2apic::lapic::{LocalApic, LocalApicBuilder};

    let lapic = LocalApicBuilder::new()
        .timer_vector(timer_index)
        .error_vector(error_index)
        .spurious_vector(spurious_index)
        .build()
        .unwrap_or_else(|err| panic!("{}", err));

    unsafe { lapic.enable(); }
    
}

pub fn init_ioapic {
    use x2apic::ioapic::{IoApic, IrqFlags, IrqMode};
    
    /// !!! Map the IOAPIC's MMIO address `addr` here !!!
    
    unsafe {
        let ioapic = IoApic::new(addr);
        
        ioapic.init(irq_offset);
        
        ioapic.enable_irq(
            irq_number,
            dest, // CPU(s)
            IrqMode::Fixed,
            IrqFlags::LEVEL_TRIGGERED | IrqFlags::LOW_ACTIVE,
        );
    }
}
