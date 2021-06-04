use linked_list_allocator::LockedHeap;
use core::alloc::Layout;
use core::alloc::GlobalAlloc;

#[global_allocator]
pub static mut ALLOCATOR: Allocator = Allocator::new();

pub struct Allocator(LockedHeap);
impl Allocator {
    pub const fn new() -> Self { Allocator(LockedHeap::empty()) }
    pub fn init(&self) {
        unsafe { self.0.lock().init(100000, 100_000_000); }
    }
} unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.0.alloc(layout)
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.0.dealloc(ptr, layout)
    }

}
