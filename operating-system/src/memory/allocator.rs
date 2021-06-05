use core::alloc::{Layout, GlobalAlloc};
use core::ptr::null_mut;
use super::frame_allocator::FRAME_ALLOCATOR;
use x86_64::structures::paging::FrameAllocator;
use alloc::vec::Vec;
use x86_64::addr::PhysAddr;

#[global_allocator]
pub static mut ALLOCATOR: Allocator = Allocator;

static mut ALLOCATIONS: Allocations = Allocations::new();
static mut FREE_REGIONS: Allocations = Allocations::new();

type Allocations = Vec::<Allocation>;

#[derive(PartialEq)]
pub struct Allocation {
    pub start: PhysAddr,
    pub size: usize,
}

pub struct Allocator;
impl Allocator {

    /// Tries to allocate a region of memory using known
    /// free regions. Return a null pointer if there
    /// is no compatable free region.
    fn alloc_from_free(&self, layout: Layout) -> *mut u8 {
        // Use a free region that is of the same size
        // if possible
        let index = unsafe { FREE_REGIONS.iter() }
            .position(|alloc| alloc.size == layout.size());

        if let Some(index) = index {
            let reg = unsafe { FREE_REGIONS.remove(index) };
            let addr = reg.start.as_u64() as *mut u8;
            unsafe { ALLOCATIONS.push(reg) };
            return addr;
        }
        
        // Use a free region that is larger than 
        // the size needed
        let index = unsafe { FREE_REGIONS.iter() }
            .position(|alloc| alloc.size < layout.size());

        if let Some(index) = index {

            // Get the found free region
            let reg = unsafe { FREE_REGIONS.remove(index) };

            // Split the region into a free and unfree
            // region
            let new_free = Allocation {
                start: reg.start + layout.size(),
                size: reg.size - layout.size(),
            };
            let new_alloc = Allocation {
                start: reg.start,
                size: layout.size(),
            };

            // Add the regions to their respective vectors
            unsafe{ FREE_REGIONS.push(new_free) };
            unsafe{ ALLOCATIONS.push(new_alloc) };

            return reg.start.as_u64() as *mut u8;
        }

        // If a compatible free region can not be found
        // return a nullptr
        return null_mut();
    }

    fn get_frame(layout: Layout) -> *mut u8 {
        
        let frame = unsafe { FRAME_ALLOCATOR.allocate_frame() }
            .unwrap_or(return null_mut());

        let reg = Allocation {
            start: frame.start_address(),
            size: frame.size() as usize,
        };
        
        // Split the region into a free and unfree
        // region
        let new_free = Allocation {
            start: reg.start + layout.size(),
            size: reg.size - layout.size(),
        };
        let new_alloc = Allocation {
            start: reg.start,
            size: layout.size(),
        };

        unsafe{ FREE_REGIONS.push(new_free) };

        return reg.start.as_u64() as *mut u8;
    }

} unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {

        // Check if the allocation can be made from
        // already known free regions of memory
        let free_ptr = self.alloc_from_free(layout);
        
        // If an allocation can be made from
        // known free regions, return a pointer to it
        if !free_ptr.is_null() { return free_ptr; }

        // This temporarily return a null pointer
        else { return null_mut() };
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {

        // Make an allocation from the layout to test against
        let test_alloc = Allocation {
            start: PhysAddr::new(ptr as u64),
            size: layout.size(),
        };

        // Get the index of the allocation if it exists.
        // If it doesn't exist return
        let index = ALLOCATIONS.iter()
            .position(|alloc| *alloc == test_alloc)
            .unwrap_or(return);
       
        // Recycle the allocation
        let new_free = ALLOCATIONS.remove(index);
        FREE_REGIONS.push(new_free);
    }
}
