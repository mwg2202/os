use log::info;
use super::ST;
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

pub fn count_descriptors(mmap: &MemoryMap) -> String {
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
        let count = mmap.iter().filter(|d| d.ty == mt ).count();
        string += &format!("\n    {} {:?}", count, mt);
    }

    return string;
}

pub fn get_free_memory(mmap: MemoryMap) -> MemoryMap {
    // Determines whether or not a descriptor is free
    let is_free = |desc: MemoryDescriptor| -> bool {
        desc.ty == MemoryType::CONVENTIONAL
            || desc.ty == MemoryType::BOOT_SERVICES_DATA
            || desc.ty == MemoryType::BOOT_SERVICES_CODE
    };

    // Gets the last element of a vector
    let last = |vec: &mut Vec<_>| { vec[vec.len() - 1] };
   
    // Create a vector to hold the free_memory
    let mut free_mem = MemoryMap::new();

    // Go through each descriptor in the memory map to find
    // descriptors which represent regions of free memory
    for mut desc in mmap {
        
        // If the current descriptor represents free memory
        // add it to the vector or combine it with the previous
        // descriptor if possible
        if is_free(desc)
                && !free_mem.is_empty() 
                && (
                    last(&mut free_mem).phys_start
                    + last(&mut free_mem).page_count * 4 * 1024
                    == desc.phys_start
                ) && last(&mut free_mem).att == desc.att {

            // Update the last descriptor
            last(&mut free_mem).page_count += desc.page_count;

        } else if is_free(desc) {

            // Change the memory type to conventional
            desc.ty = MemoryType::CONVENTIONAL;
            
            // Push the descriptor onto free_mem
            free_mem.push(desc);
            
        }
    } 
    return free_mem;
}
