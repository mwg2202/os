extern crate alloc;
use acpi::{AcpiTables, sdt::Signature, platform::Apic, 
    PhysicalMapping, AmlTable};
use aml::{AmlContext, DebugVerbosity};
use super::{Error, SystemHandles};
use rsdp::Rsdp;
use alloc::boxed::Box;
use x86_64::instructions::port::Port;

use super::fadt::Fadt;

static mut AML_CONTEXT: Option<AmlContext> = None;
static mut TABLES: Option<AcpiTables<Handler>> = None;

pub fn init_acpi(h: &SystemHandles) -> Result<(), Error> {
    // Initialize the TABLES static variable
    let t = {
        let rsdp = h.acpi2.or(h.acpi).ok_or(Error::AcpiHandleNotFound)?;
        //let handler = AcpiHandlerTraitImpl;
        unsafe { AcpiTables::from_rsdp(Handler, (rsdp as *const Rsdp) as usize) }
    }?;
    unsafe {TABLES = Some(t)};

    // Initialize the AML_CONTEXT static variable
    unsafe {AML_CONTEXT = Some(
        AmlContext::new(Box::new(Handler), false, DebugVerbosity::All)
    )};

    Ok(())
}
    
pub fn find_apic() -> Result<Apic, Error> {
    let t = unsafe {TABLES.as_ref()}.ok_or(Error::NoAcpiTables)?;
    match t.platform_info()?.interrupt_model {
        acpi::InterruptModel::Apic(apic) => Ok(apic),
        _ => Err(Error::CouldNotFindApic),
    }
}

/// More information in chapter 7 of the acpi specification
pub fn shutdown() -> Result<(), Error> {
    let context = unsafe{AML_CONTEXT.as_ref()}.ok_or(Error::NoAmlContext)?;
    let tables = unsafe {TABLES.as_ref()}.ok_or(Error::NoAcpiTables)?;
    let fadt = unsafe {tables.get_sdt::<Fadt>(Signature::FADT)?.unwrap()};
    let mut port = Port::new(fadt.pm1a_control_block as u16);
    unsafe { port.write(0b0011010000000000 as u16) };
    
    // Returns a PhysicalMapping<H, T>
    //let fadt = t.get_sdt::<AmlTable>(Signature::FADT)?.unwrap();
    //c.parse_table(fadt.virtual_start.as_ref());
    //c.invoke_method(AmlName::from_str("\\_TTS"), 5)?;
    
    // Notify Device Drivers
    //c.invoke_method(AmlName::from_str("\\_PTS"), 5)?;
    //let s5 = c.invoke_method(AmlName::from_str("\\_S5"))?;
    // Writes the sleer
    Ok(())
}

#[derive(Clone, Copy)]
struct Handler;
impl acpi::AcpiHandler for Handler {
    unsafe fn map_physical_region<T>(
        &self,
        physical_start: usize,
        size: usize,
    ) -> PhysicalMapping<Self, T> {
        use core::ptr::NonNull;

        PhysicalMapping {
            physical_start,
            virtual_start: NonNull::new_unchecked(physical_start as *mut T),
            region_length: core::mem::size_of::<T>(),
            mapped_length: core::mem::size_of::<T>(),
            handler: *self,
        }
    }

    fn unmap_physical_region<T>(&self, region: &PhysicalMapping<Self, T>) {}
}
impl aml::Handler for Handler {
    fn read_u8(&self, _address: usize) -> u8 {
        unimplemented!()
    }
    fn read_u16(&self, _address: usize) -> u16 {
        unimplemented!()
    }
    fn read_u32(&self, _address: usize) -> u32 {
        unimplemented!()
    }
    fn read_u64(&self, _address: usize) -> u64 {
        unimplemented!()
    }

    fn write_u8(&mut self, _address: usize, _value: u8) {
        unimplemented!()
    }
    fn write_u16(&mut self, _address: usize, _value: u16) {
        unimplemented!()
    }
    fn write_u32(&mut self, _address: usize, _value: u32) {
        unimplemented!()
    }
    fn write_u64(&mut self, _address: usize, _value: u64) {
        unimplemented!()
    }

    fn read_io_u8(&self, _port: u16) -> u8 {
        unimplemented!()
    }
    fn read_io_u16(&self, _port: u16) -> u16 {
        unimplemented!()
    }
    fn read_io_u32(&self, _port: u16) -> u32 {
        unimplemented!()
    }

    fn write_io_u8(&self, _port: u16, _value: u8) {
        unimplemented!()
    }
    fn write_io_u16(&self, _port: u16, _value: u16) {
        unimplemented!()
    }
    fn write_io_u32(&self, _port: u16, _value: u32) {
        unimplemented!()
    }

    fn read_pci_u8(&self, _segment: u16, _bus: u8, device: u8, _function: u8, _offset: u16) -> u8 {
        unimplemented!()
    }
    fn read_pci_u16(&self, _segment: u16, _bus: u8, device: u8, _function: u8, _offset: u16) -> u16 {
        unimplemented!()
    }
    fn read_pci_u32(&self, _segment: u16, _bus: u8, device: u8, _function: u8, _offset: u16) -> u32 {
        unimplemented!()
    }
    fn write_pci_u8(&self, _segment: u16, _bus: u8, device: u8, _function: u8, _offset: u16, _value: u8) {
        unimplemented!()
    }
    fn write_pci_u16(&self, _segment: u16, _bus: u8, device: u8, _function: u8, _offset: u16, _value: u16) {
        unimplemented!()
    }
    fn write_pci_u32(&self, _segment: u16, _bus: u8, device: u8, _function: u8, _offset: u16, _value: u32) {
        unimplemented!()
    }
}
