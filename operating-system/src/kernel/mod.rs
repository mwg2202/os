mod allocator;
mod graphics;
mod system;
pub use system::{SystemHandles, Error};
use super::crash;
//use aml::{AmlContext, DebugVerbosity};
//use graphics::{fonts, Color, BufferTrait, Size, 
//    Location, WindowManager, PixelFormat};

pub fn start(h: SystemHandles) -> ! {
    system::init_acpi(&h).map_err(crash);
    let system_font = graphics::fonts::init().or_else(|| crash(Error::CouldNotFindSystemFont));
    
    /* 
    let mut gb = graphics::Screen::init(&st.boot_services());
    let mut wm = WindowManager::new();
    gb.fill(Color::new(0, 255, 0));
    gb.write_text("System Successfully Loaded!", Location {x:50, y:50},
                  &system_font, 50.0, Color::new(255, 255, 255));
    
    wm.create_window(0, Size {width:100, height:100}, 
                     Location {x:100, y:100}, gb.fmt());
    wm.draw(&mut gb);
    */

    system::shutdown();
    loop {}
}

use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
