use core::alloc::{Layout, GlobalAlloc};
use core::ptr::null_mut;
use super::frame_allocator::{Frame, FRAME_ALLOCATOR};
use x86_64::structures::paging::FrameAllocator;
use crate::memory::uefi_allocator::UefiAllocator;
use alloc::vec::Vec;
use x86_64::addr::PhysAddr;
use log::trace;

#[global_allocator]
pub static mut ALLOCATOR: Allocator = Allocator;

static mut ALLOCATIONS: Allocations = 
    Allocations::new_in(UefiAllocator);
static mut FREE_REGIONS: Allocations = 
    Allocations::new_in(UefiAllocator);

type Allocations = Vec::<Allocation, UefiAllocator>;

#[derive(PartialEq)]
pub struct Allocation {
    pub start: PhysAddr,
    pub size: usize,
}

pub struct Allocator;
impl Allocator {

    fn init() {
        unimplemented!();
    }

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

    fn get_frame(&self, layout: Layout) -> *mut u8 {
        trace!("Allocator::get_frame called!");
        
        let frame = unsafe { FRAME_ALLOCATOR.allocate_frame() };
        if frame.is_none() {
            trace!("Allocator::get_frame failed!");
            return null_mut()
        }
        let frame = frame.unwrap();

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

        trace!("Allocator::get_frame successful!");
        return reg.start.as_u64() as *mut u8;
    }

} unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        trace!("Allocator::alloc called!");

        if layout.size() > 4096 {panic!("Allocator::alloc Layout too big");}

        // Check if the allocation can be made from
        // already known free regions of memory
        let free_ptr = self.alloc_from_free(layout);
        if !free_ptr.is_null() { return free_ptr; }

        // Use a new frame
        let free_ptr = self.get_frame(layout);
        if !free_ptr.is_null() { return free_ptr; }

        // This temporarily return a null pointer
        trace!("Allocator::alloc failed!");
        return null_mut();
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        trace!("Allocator::dealloc called!");

        // Make an allocation from the layout to test against
        let test_alloc = Allocation {
            start: PhysAddr::new(ptr as u64),
            size: layout.size(),
        };

        // Get the index of the allocation if it exists.
        // If it doesn't exist return
        let index = ALLOCATIONS.iter()
            .position(|alloc| *alloc == test_alloc);
        if index.is_none() {
            trace!("Allocator::dealloc called!");
            return;
        }
        let index = index.unwrap();
       
        // Recycle the allocation
        let new_free = ALLOCATIONS.remove(index);
        FREE_REGIONS.push(new_free);
    }
}

fn align_up(addr: usize, align: usize) -> usize {
    // Align the address
    (addr + align - 1) & !(align - 1)
}
