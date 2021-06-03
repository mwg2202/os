// mod allocator;
// mod graphics;
// mod mapper;
mod system;
// use mapper::Mapper;
pub use system::{Error, SystemHandles};
// use x86_64::structures::paging::Translate;
// use x86_64::VirtAddr;
use log::info;
use alloc::vec::Vec;
use uefi::table::boot::{MemoryType, MemoryDescriptor};
use crate::entry::memory_map::{
    MemoryMap, 
    count_descriptors,
    get_free_memory
};

// use aml::{AmlContext, DebugVerbosity};
// use graphics::{fonts, Color, BufferTrait, Size,
//    Location, WindowManager, PixelFormat};

pub fn start(h: SystemHandles, mmap: MemoryMap) -> ! {

    let free_mem = get_free_memory(mmap);
    // read_memory_map(&free_mem);
    // for desc in free_mem { info!("{:?}", &desc); }
    // allocator::init(&mut Mapper, &mut Mapper, 0x10000, 1024);
    // system::init_acpi(&h); //.map_err(crash);
    // let system_font = graphics::fonts::init().or_else(||
    // crash(Error::CouldNotFindSystemFont));

    // let mut gb = graphics::Screen::init(&st.boot_services());
    // let mut wm = WindowManager::new();
    // gb.fill(Color::new(0, 255, 0));
    // gb.write_text("System Successfully Loaded!", Location {x:50, y:50},
    // &system_font, 50.0, Color::new(255, 255, 255));
    //
    // wm.create_window(0, Size {width:100, height:100},
    // Location {x:100, y:100}, gb.fmt());
    // wm.draw(&mut gb);

    // info("Calling Shutdown");
    // system::shutdown();
    // info("Complete");
    loop {}
}
