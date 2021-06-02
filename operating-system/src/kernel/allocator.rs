use core::alloc::Layout;

use linked_list_allocator::LockedHeap;
use x86_64::structures::paging::mapper::MapToError;
use x86_64::structures::paging::{
    FrameAllocator, Mapper, Page, PageTableFlags, Size4KiB,
};
use x86_64::VirtAddr;

pub fn init(
    mapper: &mut impl Mapper<Size4KiB>,
    fa: &mut impl FrameAllocator<Size4KiB>,
    start_addr: usize,
    size: usize,
) -> Result<(), MapToError<Size4KiB>> {
    let page_range = {
        // Get the address of the start of the heap
        let start = VirtAddr::new(start_addr as u64);

        // Get the address of the end of the heap
        let end = start + size - 1u64;

        // Get pages from the start and end addresses
        let start = Page::containing_address(start);
        let end = Page::containing_address(end);

        // Get a range of pages from the start and end pages
        Page::range_inclusive(start, end)
    };
    // Map each page to physical memory
    for page in page_range {
        let frame = fa
            .allocate_frame()
            .ok_or(MapToError::FrameAllocationFailed)?;
        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
        unsafe { mapper.map_to(page, frame, flags, fa)?.flush() };
    }

    // Create an instance of the Allocator Struct
    unsafe { ALLOCATOR.lock().init(start_addr, size); }

    Ok(())
}
