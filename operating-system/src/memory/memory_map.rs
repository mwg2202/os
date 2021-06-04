use crate::ST;
use core::mem::size_of;
use alloc::vec::Vec;
use alloc::borrow::ToOwned;
use alloc::{vec, format};
use alloc::string::String;
use uefi::table::boot::{MemoryType, MemoryDescriptor};
use uefi::ResultExt;

pub type MemoryMap = Vec<MemoryDescriptor>;

/// Gets a memory map
pub fn get_memory_map() -> Result<(MemoryMap, usize), ()> {
    if let Some(st) = unsafe { ST.as_ref() } {
        let map_size = st.boot_services().memory_map_size();
        let buf_size = map_size + 8 * size_of::<MemoryDescriptor>();

        let mut mmap_storage 
            = vec![0; buf_size].into_boxed_slice();

        let (_key, iter) = st
            .boot_services()
            .memory_map(&mut mmap_storage[..])
            .expect_success("Failed to get memory map");
            
        Ok((iter.copied().collect(), map_size))
    } else { return Err(()) }
}

pub fn count_descriptors(map: &MemoryMap) -> String {
    let mut string = "Descriptor Types:".to_owned();
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
        let count = map.iter().filter(|d| d.ty == mt ).count();
        string += &format!("\n    {} {:?}", count, mt);
    }
    return string;
}

