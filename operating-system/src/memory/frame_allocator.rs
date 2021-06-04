use crate::memory::memory_map::MemoryMap;
use x86_64::structures::paging::FrameAllocator as FrameAllocatorTrait;
use x86_64::structures::paging::frame::PhysFrame;
use x86_64::structures::paging::page::Size4KiB;
use x86_64::PhysAddr;
use uefi::table::boot::MemoryType;
use alloc::vec::Vec;

pub type Frame = PhysFrame::<Size4KiB>;

pub struct FrameAllocator(Vec::<(Frame, bool)>);
impl FrameAllocator {
    
    /// Creates an empty frame allocator
    pub fn new() -> Self {
        Self(Vec::<(Frame, bool)>::new())
    }
   
    /// Reclaims UEFI memory by creating allocatable frames
    /// for descriptors of a certain type
    pub fn reclaim(&mut self, map: MemoryMap, mt: MemoryType) {
        for desc in map {
            if desc.ty == mt {
                for i in 0..desc.page_count {
                    let addr = PhysAddr::new(
                        desc.phys_start + i*4096);
                    let frame = PhysFrame::containing_address(addr);
                    self.0.push( (frame, true) );
                }
            }
        }
    }

} unsafe impl FrameAllocatorTrait::<Size4KiB> for FrameAllocator {
    fn allocate_frame(&mut self) -> Option<Frame> {
        self.0.iter().find(|(_, free)| *free).map(|f| f.clone().0)
    }
}
