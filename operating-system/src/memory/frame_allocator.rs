use crate::memory::memory_map::MemoryMap;
use crate::memory::memory_map;
use crate::memory::allocator::Allocator;
use x86_64::structures::paging::FrameAllocator as FrameAllocatorTrait;
use x86_64::structures::paging::frame::PhysFrame;
use x86_64::structures::paging::page::Size4KiB;
use x86_64::PhysAddr;
use uefi::table::boot::{MemoryType, MemoryDescriptor};
use alloc::vec::Vec;

pub type Frame = PhysFrame::<Size4KiB>;

pub static mut FRAME_ALLOCATOR: FrameAllocator 
    = FrameAllocator::new();

pub struct FrameAllocator(Vec::<(Frame, bool)>);
impl FrameAllocator {
    
    /// Creates an empty frame allocator
    pub const fn new() -> Self { Self(Vec::<(Frame, bool)>::new()) }

    pub fn init(&mut self) {

        // Get the UEFI memory map
        let map = memory_map::get().unwrap();
        
        // Reclaim UEFI memory designated as conventional
        map.filter(|desc| desc.ty == MemoryType::CONVENTIONAL)
            .for_each(|desc| self.reclaim(desc));
    }

    /// Reclaims a region of memory described by a MemoryDescriptor
    pub fn reclaim(&mut self, desc: &MemoryDescriptor) {
        for i in 0..desc.page_count {
            let addr = PhysAddr::new(
                desc.phys_start + i*4096);
            let frame = PhysFrame::containing_address(addr);

            if self.0.is_empty() {
                Allocator::init(&frame);
            } else {
                self.0.push( (frame, true) );
            }
        }
    }
   
    /// Reclaims all memory of a certain type in a MemoryMap
    pub fn reclaim_type(&mut self, map: MemoryMap, mt: MemoryType) {
        map.filter(|desc| desc.ty == mt)
            .for_each(|desc| self.reclaim(desc));
    }

} unsafe impl FrameAllocatorTrait::<Size4KiB> for FrameAllocator {
    fn allocate_frame(&mut self) -> Option<Frame> {
        self.0.iter().find(|(_, free)| *free).map(|f| f.clone().0)
    }
}
