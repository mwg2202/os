use x86_64::{
    PhysAddr, VirtAddr, 
    structures::paging::{Page, PhysFrame, Mapper, Size4KiB, FrameAllocator}
};

struct Mapper;
unsafe impl FrameAllocator<Size4KiB> for EmptyFrameAllocator {
    memory_map: &'static,
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        None
    }
}
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
