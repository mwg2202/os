use core::alloc::{Layout, GlobalAlloc, Allocator};
use crate::memory::uefi_allocator::UefiAllocator as A;
use core::ptr::{NonNull, null_mut};

// type A = UefiAllocator;

#[global_allocator]
pub static mut ALLOCATOR: GlobalAllocator = GlobalAllocator::new();

pub struct GlobalAllocator {
    allocator: A,
} impl GlobalAllocator {

    pub const fn new() -> Self { GlobalAllocator { allocator: A } }

} unsafe impl GlobalAlloc for GlobalAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        match self.allocator.allocate(layout) {
            Ok(ptr) => ptr.as_mut_ptr(),
            _ => null_mut()

        }
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let ptr = NonNull::new(ptr);
        if let Some(ptr) = ptr {
            self.allocator.deallocate(ptr, layout);
        }
    }
}
