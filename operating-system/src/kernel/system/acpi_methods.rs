extern crate alloc;
use alloc::boxed::Box;

use acpi::{
    platform::Apic, 
    sdt::Signature, 
    AcpiTables,
    AmlTable, 
    PhysicalMapping
};
use aml::{AmlContext, DebugVerbosity};
use rsdp::Rsdp;
use x86_64::instructions::port::Port;
use x86_64::structures::port::{PortRead, PortWrite};

use super::fadt::Fadt;
use super::{Error, SystemHandles};
use log::{info, debug, trace};

static mut AML_CONTEXT: Option<AmlContext> = None;
static mut TABLES: Option<AcpiTables<Handler>> = None;

pub fn init_acpi(h: &SystemHandles) -> Result<(), Error> {
    
    debug!("Setting up ACPI tables");
    // Get the rsdp from the system handles
    let rsdp = h.acpi2.or(h.acpi)
        .ok_or(Error::RsdpNotFound)?;
        
    // Get the ACPI tables from the rsdp pointer
    let tables = unsafe { 
        AcpiTables::from_rsdp(
            Handler, (rsdp as *const Rsdp) as usize
        ) 
    }?;

    debug!("Creating a new AML context");
    let mut aml_ctx = AmlContext::new(
        Box::new(Handler),
        DebugVerbosity::None,
    );

    debug!("Running DSDT through AML context");
    if let Some(dsdt) = tables.dsdt {
        let dsdt = unsafe {
            alloc::slice::from_raw_parts(
                dsdt.address as *const u8,
                dsdt.length as usize,
            )
        };
        aml_ctx.parse_table(dsdt)?;
    } else { debug!("DSDT not found"); }

    debug!("Running SSDTs through AML context");
    for ssdt in tables.ssdts {
        let ssdt = unsafe {
            alloc::slice::from_raw_parts(
                ssdt.address as *const u8,
                ssdt.length as usize,
            )
        };
        aml_ctx.parse_table(ssdt)?;
    }


    
    // Save the AML context object and the ACPI tables object
    unsafe { AML_CONTEXT = Some(aml_ctx) };
    // unsafe { TABLES = Some(tables) };

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
        debug!("Creating a physical mapping");
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
        debug!("read_pci called");
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
        debug!("read_pci called");
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
        debug!("read_pci called");
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
        debug!("read_pci called");
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
        debug!("read_pci called");
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
        debug!("read_pci called");
        unimplemented!()
    }
}
