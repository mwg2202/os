// mod graphics;
// mod mapper;
mod system;
pub use system::{Error, SystemHandles};
use log::{debug, trace};
use crate::entry::memory_map::{ MemoryMap, get_free_memory };

// use graphics::{fonts, Color, BufferTrait, Size,
//    Location, WindowManager, PixelFormat};

pub fn start(h: SystemHandles, mmap: MemoryMap) -> ! {

    let _free_mem = get_free_memory(mmap);
    // for desc in free_mem { info!("{:?}", &desc); }
    
    // Get the system font
    // let system_font = graphics::fonts::init().or_else(||
    //     crash(Error::CouldNotFindSystemFont));

    // let mut gb = graphics::Screen::init(&st.boot_services());
    // let mut wm = WindowManager::new();
    // gb.fill(Color::new(0, 255, 0));
    // gb.write_text("System Working!", Location {x:50, y:50},
    // &system_font, 50.0, Color::new(255, 255, 255));
    
    // wm.create_window(0, Size {width:100, height:100},
    // Location {x:100, y:100}, gb.fmt());
    // wm.draw(&mut gb);
    
    debug!("Initializing ACPI methods");
    system::init_acpi(&h).expect("Could not initialize ACPI methods");

    debug!("Shutting down the system");
    match system::shutdown(5) {
        Ok(_) => debug!("Successfully shut down system"),
        Err(err) => {
            trace!("{:?}", err);
            panic!("Could not shutdown system");
        },
    }

    loop {}
}
