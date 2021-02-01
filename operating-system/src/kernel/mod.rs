//extern crate alloc;
mod allocator;
//mod graphics;
pub mod system;
use system::SystemHandles;
//use system::Errors;
//use aml::{AmlContext, DebugVerbosity};
//use graphics::{fonts, Color, BufferTrait, Size, 
//    Location, WindowManager, PixelFormat};

pub fn start(h: SystemHandles) {
    /* 
    let platform_info = system::get_platform_info(&sys_handles)
        .map_err(system::crash);
    
    let apic = system::find_apic(&platform_info).map_err(system::crash);
    
    let mut gb = graphics::Screen::init(&st.boot_services());

    let system_font = fonts::init()
        .or_else(system::crash(Errors::CouldNotFindSystemFont));
    */
    /*
    let mut wm = WindowManager::new();
    gb.fill(Color::new(0, 255, 0));
    gb.write_text("System Successfully Loaded!", Location {x:50, y:50},
                  &system_font, 50.0, Color::new(255, 255, 255));
    
    wm.create_window(0, Size {width:100, height:100}, 
                     Location {x:100, y:100}, gb.fmt());
    wm.draw(&mut gb);
    */
    loop {}

}

use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}


