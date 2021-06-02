use log::info;
use super::ST;
use core::mem::size_of;
use alloc::vec::Vec;
use alloc::borrow::ToOwned;
use alloc::{vec, format};
use uefi::table::boot::{MemoryType, MemoryDescriptor};
use uefi::ResultExt;

pub type MemoryMap = Vec<MemoryDescriptor>;

/// Gets a memory map
pub fn get_memory_map() -> Result<MemoryMap, ()> {
    if let Some(st) = unsafe { ST.as_ref() } {
        let max_mmap_size =
            st.boot_services().memory_map_size() 
            + 8 * size_of::<MemoryDescriptor>();

        info!("{:?}", &max_mmap_size); // REMOVE LATER
        info!("{:?}", &size_of::<MemoryDescriptor>());

        let mut mmap_storage 
            = vec![0; max_mmap_size].into_boxed_slice();

        let (_key, iter) = st
            .boot_services()
            .memory_map(&mut mmap_storage[..])
            .expect_success("Failed to get memory map");
            
        Ok(iter.copied().collect())
    } else { return Err(()) }
}

/// Ends UEFI Boot Services and returns a final memory map
pub fn exit_uefi(image: uefi::Handle) -> Result<MemoryMap, ()> {
    let mut mmap_storage = {
        if let Some(st) = unsafe { ST.as_ref() } {
            let max_mmap_size =
                st.boot_services().memory_map_size() 
                + 8 * size_of::<MemoryDescriptor>();

            info!("{:?}", &max_mmap_size); // REMOVE LATER
            info!("{:?}", &size_of::<MemoryDescriptor>());

            vec![0; max_mmap_size].into_boxed_slice()
        } else { return Err(()) }
    };

    if let Some(st) = unsafe { ST.take() } {
        let (_st, iter) = st
            .exit_boot_services(image, &mut mmap_storage[..])
            .expect_success("Failed to exit boot services");
        
        Ok(iter.copied().collect())
    } else { Err(()) }
}

pub fn read_memory_map(mmap: &MemoryMap) {
    let mut string = "MEMORY MAP DATA:".to_owned();
    // List of all the memory types
    let mts = [
        MemoryType::RESERVED, MemoryType::LOADER_CODE, 
        MemoryType::LOADER_DATA, MemoryType::BOOT_SERVICES_CODE,
        MemoryType::BOOT_SERVICES_DATA, 
        MemoryType::RUNTIME_SERVICES_CODE,
        MemoryType::RUNTIME_SERVICES_DATA, 
        MemoryType::CONVENTIONAL, MemoryType::UNUSABLE,
        MemoryType::ACPI_RECLAIM, MemoryType::ACPI_NON_VOLATILE, 
        MemoryType::MMIO, MemoryType::MMIO_PORT_SPACE, 
        MemoryType::PAL_CODE, MemoryType::PERSISTENT_MEMORY
    ];

    for mt in mts {
        let count = mmap.iter().filter(|d| d.ty == mt ).count();
        string += &format!("\n    {} {:?}", count, mt);
    }

    info!("{}", string);
}

