extern crate alloc;
use alloc::boxed::Box;

use acpi::{ AcpiTables, PhysicalMapping };
use acpi::mcfg::{PciConfigRegions};
use aml::{AmlContext, DebugVerbosity, AmlName};
use rsdp::Rsdp;
// use x86_64::instructions::port::Port;
use x86_64::structures::port::{PortRead, PortWrite};

use super::{Error, SystemHandles};
use log::debug;
use alloc::string::ToString;
use alloc::borrow::ToOwned;

static mut AML_CONTEXT: Option<AmlContext> = None;
static mut PCI_REGIONS: Option<PciConfigRegions> = None;


/// Parses the acpi tables and creates an aml context object to be
/// used when clalling acpi methods
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
    
    debug!("Locating the PCIe configuration space");
    let regions = PciConfigRegions::new(&tables)?;
    unsafe { PCI_REGIONS = Some(regions)};


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

    aml_ctx.initialize_objects()?;
    
    // Save the AML context object and the ACPI tables object
    unsafe { AML_CONTEXT = Some(aml_ctx) };

    Ok(())
}

/// More information in chapter 7 of the acpi specification
pub fn shutdown(mode: usize) -> Result<(), Error> {
    // Get the current Aml context
    let aml_ctx = unsafe { AML_CONTEXT.as_ref() }
        .ok_or(Error::NoAmlContext)?;
   
    let mode = "\\_S".to_owned() + mode.to_string().as_str();
    let mode = aml_ctx
        .namespace
        .get_by_path(&AmlName::from_str(&mode)?)?;
    debug!("{:?}", &mode);


    // Returns a PhysicalMapping<H, T>
    // let fadt = unsafe { tables.get_sdt::<Fadt>(Signature::FADT) }?
    //     .unwrap();
    

    // let mut port = Port::new(fadt.pm1a_control_block as u16);
    // unsafe { port.write(0b0011110000000000 as u16) };

    Ok(())
}

pub fn wakeup() -> Result<(), Error> {
    // let context = unsafe { AML_CONTEXT.as_ref() }
    //     .ok_or(Error::NoAmlContext)?;

    // Returns a PhysicalMapping<H, T>
    // let fadt = unsafe { tables.get_sdt::<Fadt>(Signature::FADT) }?
    //     .unwrap();

    // c.parse_table(fadt.virtual_start.as_ref());
    // let s5 = c.invoke_method(AmlName::from_str("\\_S5"))?;

    // let mut port = Port::new(fadt.pm1a_control_block as u16);
    // unsafe { port.write(0b0011110000000000 as u16) };

    Ok(())
}

#[derive(Clone, Copy)]
pub struct Handler;
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

    fn unmap_physical_region<T>(&self, _region: &PhysicalMapping<Self, T>) {}
}
impl aml::Handler for Handler {
    fn read_u8(&self, address: usize) -> u8 {
        unsafe { *(address as *const u8) }
    }

    fn read_u16(&self, address: usize) -> u16 {
        unsafe { *(address as *const u16) }
    }

    fn read_u32(&self, address: usize) -> u32 {
        unsafe { *(address as *const u32) }
    }

    fn read_u64(&self, address: usize) -> u64 {
        unsafe { *(address as *const u64) }
    }

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

    fn read_io_u8(&self, port: u16) -> u8 {
        unsafe { u8::read_from_port(port) }
    }

    fn read_io_u16(&self, port: u16) -> u16 {
        unsafe { u16::read_from_port(port) }
    }

    fn read_io_u32(&self, port: u16) -> u32 { 
        unsafe { u32::read_from_port(port) }
    }

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
        segment: u16,
        bus: u8,
        device: u8,
        function: u8,
        offset: u16,
    ) -> u8 {

        // Get the PCIe regions
        let pci_regions = unsafe { PCI_REGIONS.as_ref() }
            .expect("Could not find PCI Configuration Space");
        
        // Get the addressof the 
        let addr = pci_regions
            .physical_address(segment, bus, device, function)
            .expect("Cannot manage device") as usize;
        
        // Read the address
        let offset = offset as usize;
        self.read_u8(addr + offset)

    }

    fn read_pci_u16(
        &self,
        segment: u16,
        bus: u8,
        device: u8,
        function: u8,
        offset: u16,
    ) -> u16 {

        // Get the PCIe regions
        let pci_regions = unsafe { PCI_REGIONS.as_ref() }
            .expect("Could not find PCI Configuration Space");
        
        // Get the addressof the 
        let addr = pci_regions
            .physical_address(segment, bus, device, function)
            .expect("Cannot manage device") as usize;
        
        // Read the address
        let offset = offset as usize;
        self.read_u16(addr + offset)

    }

    fn read_pci_u32(
        &self,
        segment: u16,
        bus: u8,
        device: u8,
        function: u8,
        offset: u16,
    ) -> u32 {

        // Get the PCIe regions
        let pci_regions = unsafe { PCI_REGIONS.as_ref() }
            .expect("Could not find PCI Configuration Space");
        
        // Get the addressof the 
        let addr = pci_regions
            .physical_address(segment, bus, device, function)
            .expect("Cannot manage device") as usize;
        
        // Read the address
        let offset = offset as usize;
        self.read_u32(addr + offset)

    }

    fn write_pci_u8(
        &self,
        segment: u16,
        bus: u8,
        device: u8,
        function: u8,
        offset: u16,
        value: u8,
    ) {

        // Get the PCIe regions
        let pci_regions = unsafe { PCI_REGIONS.as_ref() }
            .expect("Could not find PCI Configuration Space");
        
        // Get the addressof the 
        let addr = pci_regions
            .physical_address(segment, bus, device, function)
            .expect("Cannot manage device") as usize;
        
        // Read the address
        let offset = offset as usize;
        unsafe { *((addr + offset) as *mut u8) = value }

    }

    fn write_pci_u16(
        &self,
        segment: u16,
        bus: u8,
        device: u8,
        function: u8,
        offset: u16,
        value: u16,
    ) {

        // Get the PCIe regions
        let pci_regions = unsafe { PCI_REGIONS.as_ref() }
            .expect("Could not find PCI Configuration Space");
        
        // Get the addressof the 
        let addr = pci_regions
            .physical_address(segment, bus, device, function)
            .expect("Cannot manage device") as usize;
        
        // Read the address
        let offset = offset as usize;
        unsafe { *((addr + offset) as *mut u16) = value }

    }

    fn write_pci_u32(
        &self,
        segment: u16,
        bus: u8,
        device: u8,
        function: u8,
        offset: u16,
        value: u32,
    ) {

        // Get the PCIe regions
        let pci_regions = unsafe { PCI_REGIONS.as_ref() }
            .expect("Could not find PCI Configuration Space");
        
        // Get the addressof the 
        let addr = pci_regions
            .physical_address(segment, bus, device, function)
            .expect("Cannot manage device") as usize;
        
        // Read the address
        let offset = offset as usize;
        unsafe { *((addr + offset) as *mut u32) = value }

    }
}
