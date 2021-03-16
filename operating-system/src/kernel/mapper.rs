use x86_64::{
    PhysAddr, VirtAddr,
    structures::paging::{
        Page, PhysFrame, Size4KiB, FrameAllocator, PageTable, page_table::FrameError, 
        mapper::{Translate, TranslateResult, MappedFrame}
    }
};
use x86_64::registers::control::Cr3;
use acpi::PhysicalMapping;
use super::info;

pub struct Mapper;
impl Mapper {
    pub fn translate(addr: VirtAddr) -> Option<PhysAddr> {
        // read the active level 4 frame from the CR3 register
        let (frame, flags) = Cr3::read();
        info("Read");
        let table_indexes = [
            addr.p4_index(), addr.p3_index(), addr.p2_index(), addr.p1_index()
        ];
        let mut frame = frame;
        
        info("begin loop");
        // traverse the mulit-level page table
        for &index in &table_indexes {
            let virt = frame.start_address().as_u64();
            let table_ptr = virt as *const PageTable;
            let table = unsafe {&*table_ptr};

            let entry = &table[index];
            frame = match entry.frame() {
                Ok(frame) => frame,
                Err(FrameError::FrameNotPresent) => return None,
                Err(FrameError::HugeFrame) => panic!("huge pages not supported")
            };
        info("looped");
        }
        Some(frame.start_address() + u64::from(addr.page_offset()))
    }
}
unsafe impl FrameAllocator<Size4KiB> for Mapper {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        None
    }
}
/*
impl acpi::AcpiHandler for Mapper {
    unsafe fn map_physical_region<T>(
        &self,
        physical_start: usize,
        size: usize,
    ) -> PhysicalMapping<Self, T> {
        use core::ptr::NonNull;

        PhysicalMapping {
            physical_start,
            virtual_start: NonNull::new_unchecked(physical_start as *mut T),
            region_length: core::mem::size_of::<T>(),
            mapped_length: core::mem::size_of::<T>(),
            handler: *self,
        }
    }

    fn unmap_physical_region<T>(&self, region: &PhysicalMapping<Self, T>) {}
}
*/
