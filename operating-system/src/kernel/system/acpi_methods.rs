extern crate alloc;
use alloc::boxed::Box;

use acpi::{platform::Apic, sdt::Signature, AcpiTables, AmlTable, PhysicalMapping};
use aml::{AmlContext, DebugVerbosity};
use rsdp::Rsdp;
use x86_64::instructions::port::Port;
use x86_64::structures::port::{PortRead, PortWrite};

use super::fadt::Fadt;
use super::{Error, SystemHandles};
use crate::info;

static mut AML_CONTEXT: Option<AmlContext> = None;
static mut TABLES: Option<AcpiTables<Handler>> = None;

pub fn init_acpi(h: &SystemHandles) -> Result<(), Error> {
    info("Setting up tables");
    // Initialize the TABLES static variable
    let t = {
        // Get the ACPI handle
        let rsdp = h.acpi2.or(h.acpi)
            .ok_or(Error::AcpiHandleNotFound)?;
    
        info("unsafe");
        unsafe { 
            AcpiTables::from_rsdp(
                Handler, (rsdp as *const Rsdp) as usize
            ) 
        }
    }?;

    info("unsafe 2");
    unsafe { TABLES = Some(t) };
    info("Set up tables");

    info("Setting up aml_context");
    // Initialize the AML_CONTEXT static variable
    unsafe {
        AML_CONTEXT = Some(AmlContext::new(
            Box::new(Handler),
            false,
            DebugVerbosity::All,
        ))
    };
    info("Set up aml_context");
    Ok(())
}

pub fn find_apic() -> Result<Apic, Error> {
    let t = unsafe { TABLES.as_ref() }.ok_or(Error::NoAcpiTables)?;
    match t.platform_info()?.interrupt_model {
        acpi::InterruptModel::Apic(apic) => Ok(apic),
        _ => Err(Error::CouldNotFindApic),
    }
}

/// More information in chapter 7 of the acpi specification
pub fn shutdown() -> Result<(), Error> {
    let context = unsafe { AML_CONTEXT.as_ref() }.ok_or(Error::NoAmlContext)?;
    let tables = unsafe { TABLES.as_ref() }.ok_or(Error::NoAcpiTables)?;

    // Returns a PhysicalMapping<H, T>
    let fadt = unsafe { tables.get_sdt::<Fadt>(Signature::FADT)?.unwrap() };
    // c.parse_table(fadt.virtual_start.as_ref());
    // let s5 = c.invoke_method(AmlName::from_str("\\_S5"))?;

    let mut port = Port::new(fadt.pm1a_control_block as u16);
    unsafe { port.write(0b0011110000000000 as u16) };

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
        info("Creating a physical mapping");
        PhysicalMapping {
            physical_start,
            virtual_start: NonNull::new_unchecked(physical_start as *mut T),
            region_length: size,
            mapped_length: size,
            handler: *self,
        }
    }

    fn unmap_physical_region<T>(&self, region: &PhysicalMapping<Self, T>) {}
}
impl aml::Handler for Handler {
    fn read_u8(&self, address: usize) -> u8 { unsafe { *(address as *const u8) } }

    fn read_u16(&self, address: usize) -> u16 { unsafe { *(address as *const u16) } }

    fn read_u32(&self, address: usize) -> u32 { unsafe { *(address as *const u32) } }

    fn read_u64(&self, address: usize) -> u64 { unsafe { *(address as *const u64) } }

    fn write_u8(&mut self, address: usize, value: u8) {
        unsafe { *(address as *mut u8) = value }
    }

    fn write_u16(&mut self, address: usize, value: u16) {
        unsafe { *(address as *mut u16) = value }
    }

    fn write_u32(&mut self, address: usize, value: u32) {
        unsafe { *(address as *mut u32) = value }
    }

    fn write_u64(&mut self, address: usize, value: u64) {
        unsafe { *(address as *mut u64) = value }
    }

    fn read_io_u8(&self, port: u16) -> u8 { unsafe { u8::read_from_port(port) } }

    fn read_io_u16(&self, port: u16) -> u16 { unsafe { u16::read_from_port(port) } }

    fn read_io_u32(&self, port: u16) -> u32 { unsafe { u32::read_from_port(port) } }

    fn write_io_u8(&self, port: u16, value: u8) {
        unsafe { u8::write_to_port(port, value) }
    }

    fn write_io_u16(&self, port: u16, value: u16) {
        unsafe { u16::write_to_port(port, value) }
    }

    fn write_io_u32(&self, port: u16, value: u32) {
        unsafe { u32::write_to_port(port, value) }
    }

    fn read_pci_u8(
        &self,
        _segment: u16,
        _bus: u8,
        device: u8,
        _function: u8,
        _offset: u16,
    ) -> u8 {
        info("read_pci called");
        unimplemented!()
    }

    fn read_pci_u16(
        &self,
        _segment: u16,
        _bus: u8,
        device: u8,
        _function: u8,
        _offset: u16,
    ) -> u16 {
        info("read_pci called");
        unimplemented!()
    }

    fn read_pci_u32(
        &self,
        _segment: u16,
        _bus: u8,
        device: u8,
        _function: u8,
        _offset: u16,
    ) -> u32 {
        info("read_pci called");
        unimplemented!()
    }

    fn write_pci_u8(
        &self,
        _segment: u16,
        _bus: u8,
        device: u8,
        _function: u8,
        _offset: u16,
        _value: u8,
    ) {
        info("read_pci called");
        unimplemented!()
    }

    fn write_pci_u16(
        &self,
        _segment: u16,
        _bus: u8,
        device: u8,
        _function: u8,
        _offset: u16,
        _value: u16,
    ) {
        info("read_pci called");
        unimplemented!()
    }

    fn write_pci_u32(
        &self,
        _segment: u16,
        _bus: u8,
        device: u8,
        _function: u8,
        _offset: u16,
        _value: u32,
    ) {
        info("read_pci called");
        unimplemented!()
    }
}
