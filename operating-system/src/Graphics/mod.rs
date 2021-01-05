use uefi::proto::console::gop::FrameBuffer;
use uefi::proto::console::gop::GraphicsOutput;
use uefi::proto::console::gop::Mode;
use uefi::prelude::*;
use uefi::table::boot::BootServices;
use uefi::proto::console::gop::PixelFormat;
use core::option::Option;

pub struct GraphicsSystem<'a> {
    frame_buffer: FrameBuffer<'a>,
    current_mode: Mode,
}
impl GraphicsSystem<'_> {
    /// Set up and enter the graphics system
    pub fn init(bs: &BootServices) -> GraphicsSystem {
        

        // Get the graphics output protocol
        let graphics_output =
            unsafe {&mut *(bs.locate_protocol::<GraphicsOutput>()
            .unwrap_success().get()) };
        
        // Get the list of modes
        let mut modes = graphics_output.modes();


        let mut best_mode = Option::None;

        // Get the first mode on the list (This will be the highest mode)
        for mode in modes {
            
            let mode = mode.unwrap();

            match mode.info().pixel_format() {
                PixelFormat::RGB | PixelFormat::BGR => {
                    match best_mode {
                        None => best_mode = Option::Some(mode),
                        Some(ref m) => {
                        
                            // Get the best resolution mode
                            let (width, height) = mode.info().resolution();
                            let (best_width, best_height) = m.info().resolution();
                            if (width > best_width) || (height > best_height) {
                                best_mode = Some(mode);
                            }
                        },
                    }
                },
                _ => (),
            }
        }
        
        // Get best_mode from the option
        let best_mode = best_mode.unwrap();

        // Set the graphics mode to said mode
        graphics_output.set_mode(&best_mode).unwrap_success();

        // Make a structure out of the information
        GraphicsSystem {
            frame_buffer: graphics_output.frame_buffer(),
            current_mode: best_mode,
        }
    }

    /// Fill the screen with a certain color
    pub fn fill_screen() {
         
    }
}
