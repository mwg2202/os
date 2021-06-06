use x86_64::structures::paging::mapper::{
    Mapper as MapperTrait,
    MapToError,
    MapperFlush,
    MapperFlushAll,
    UnmapError,
    FlagUpdateError,
    TranslateError,
};
use x86_64::structures::paging::page::{Page, Size4KiB};
use x86_64::structures::paging::frame::PhysFrame;
use x86_64::structures::paging::page_table::PageTableFlags;

pub struct Mapper;
impl MapperTrait<Size4KiB> for Mapper {
    unsafe fn map_to_with_table_flags<A: ?Sized>(
        &mut self,
        page: Page<Size4KiB>,
        frame: PhysFrame<Size4KiB>,
        flags: PageTableFlags,
        parent_table_flags: PageTableFlags,
        frame_allocator: &mut A
    ) -> Result<MapperFlush<Size4KiB>, MapToError<Size4KiB>> {
        unimplemented!();
    }

    fn unmap(
        &mut self,
        page: Page<Size4KiB>
    ) -> Result<(PhysFrame<Size4KiB>, MapperFlush<Size4KiB>), UnmapError> {
        unimplemented!();
    }

    unsafe fn update_flags(
        &mut self,
        page: Page<Size4KiB>,
        flags: PageTableFlags,
    ) -> Result<MapperFlush<Size4KiB>, FlagUpdateError> {
        unimplemented!();
    }

    unsafe fn set_flags_p4_entry(
        &mut self,
        page: Page<Size4KiB>,
        flags: PageTableFlags,
    ) -> Result<MapperFlushAll, FlagUpdateError> {
        unimplemented!();
    }
    
    unsafe fn set_flags_p3_entry(
        &mut self,
        page: Page<Size4KiB>,
        flags: PageTableFlags,
    ) -> Result<MapperFlushAll, FlagUpdateError> {
        unimplemented!();
    }
    
    unsafe fn set_flags_p2_entry(
        &mut self,
        page: Page<Size4KiB>,
        flags: PageTableFlags,
    ) -> Result<MapperFlushAll, FlagUpdateError> {
        unimplemented!();
    }
    
    fn translate_page(
        &self,
        page: Page<Size4KiB>,
    ) -> Result<PhysFrame<Size4KiB>, TranslateError> {
        unimplemented!();
    }
}
