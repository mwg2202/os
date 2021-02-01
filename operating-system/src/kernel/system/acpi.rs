use acpi::sdt::Signature;
use aml::{AmlContext, DebugVerbosity};

/// Singleton to handle ACPI
pub struct Acpi {
    aml_context: AmlContext,
    acpi_tables: AcpiTables,
}
impl Acpi {
    pub fn new(h: &SystemHandles) -> Result<Acpi, Error> {
        // Get the ACPI Tables
        let tables = {
            let rsdp = h.acpi2.or(sys_handles.acpi).ok_or(Error::AcpiHandleNotFound)?;
            let handler = AcpiHandlerTraitImpl { number: 0 };
            unsafe {
                AcpiTables::from_rsdp(handler, (rsdp as *const Rsdp) as usize)
            }
        }?;
        
        // Create a new aml_context
        let handler;
        let aml_context = AmlContext::new(handler, false, DebugVerbosity::All);
        
        // Construct the structure
        Ok(Acpi {
            aml_context,
            acpi_tables,
        })
    }
    
    pub fn find_apic(&self) -> Result<&Apic, Error> {
        match &self.tables.platform_info()?.interrupt_model {
            acpi::InterruptModel::Apic(apic) => Ok(apic)
            _ => Err(Error::CouldNotFindApic),
        }
    }

    /// More information in chapter 7 of the acpi specification
    pub fn shutdown(&self) -> Result<AmlValue, AmlError> {
        let c = self.aml_context;
        c.parse_table(a.get_sdt(Signature::FADT) as &[u8]);
        c.invoke_method(AmlName::from_str("\\_TTS"), 5);
        
        // Notify Device Drivers
        c.invoke_method(AmlName::from_str("\\_PTS"), 5);
        let s5 = c.invoke_method(AmlName::from_str("\\_S5"));
        
        // Writes the sleep vector
    }
}

#[derive(Clone, Copy)]
struct AcpiHandlerTraitImpl {
    number: u8,
}
impl acpi::AcpiHandler for AcpiHandlerTraitImpl {
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
