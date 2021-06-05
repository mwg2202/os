use crate::ST;
use core::mem::size_of;
use uefi::table::boot::{MemoryType, MemoryDescriptor};
use uefi::ResultExt;
use core::slice;

pub type MemoryMap = impl ExactSizeIterator
    <Item=&'static MemoryDescriptor>;

/// Gets a memory map using UEFI allocation
pub fn get() -> Option<MemoryMap> {

    // Get the system table
    let st = unsafe { ST.as_ref() }?;

    // Get the size of the memory map
    let map_size = st.boot_services().memory_map_size();

    // Add some extra space in case the memory map changes
    let buf_size = map_size + 8 * size_of::<MemoryDescriptor>();

    // Get the memory map
    let ptr = st.boot_services()
        .allocate_pool(MemoryType::BOOT_SERVICES_DATA, buf_size)
        .expect_success("Could not allocate pool for memory map");

    // Update the map size variable
    let map_size = st.boot_services().memory_map_size();

    // Turn the ptr into a proper buffer
    let mmap_storage = unsafe {
        slice::from_raw_parts_mut(ptr, map_size)
    };

    // Get the memory map
    let (_key, iter) = st
        .boot_services()
        .memory_map(mmap_storage)
        .expect_success("Failed to get memory map");
        
    Some(iter)
}
