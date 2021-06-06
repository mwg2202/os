use core::alloc::AllocError;
use core::alloc::Allocator;
use core::alloc::Layout;
use core::slice;
use core::ptr::NonNull;
use uefi::table::boot::MemoryType;
use crate::ST;

pub struct UefiAllocator;
unsafe impl Allocator for UefiAllocator {
    fn allocate(&self, layout: Layout) 
        -> Result<NonNull<[u8]>, AllocError> {

        // Get the system table
        if let Some(st) = unsafe {ST.as_ref()} {

            // Allocate a pool of memory
            let ptr = st.boot_services().allocate_pool(
                MemoryType::LOADER_DATA,
                layout.size(),
            );

            if let Ok(ptr) = ptr {
                let (_, ptr) = ptr.split();

                let ptr = unsafe{
                    slice::from_raw_parts_mut(ptr, layout.size())
                };

                return Ok(NonNull::new(ptr).unwrap());
            }
        }

        return Err(AllocError);
    }

    #[allow(unused_must_use)]
    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        // Get the system table
        if let Some(st) = unsafe {ST.as_ref()} {

            // Allocate a pool of memory
            st.boot_services().free_pool(ptr.as_ptr() as *mut u8);
        }
    }
}
