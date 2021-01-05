use uefi::proto::console::gop::FrameBuffer;
use uefi::proto::console::gop::GraphicsOutput;
use uefi::proto::console::gop::Mode;
use uefi::prelude::*;
use uefi::table::boot::BootServices;
use uefi::proto::console::gop::PixelFormat;
use alloc::vec::Vec;
use volatile::Volatile;

pub struct GraphicsSystem {
    frame_buffer: *mut u8,
    current_mode: Mode,
}
impl GraphicsSystem {
    /// Set up and enter the graphics system
    pub fn init(bs: &BootServices) -> GraphicsSystem {
        
        // Get the graphics output protocol
        let graphics_output =
            unsafe {&mut *(bs.locate_protocol::<GraphicsOutput>()
            .unwrap_success().get()) };
        
        // Get the list of modes
        let modes = graphics_output.modes();

        // Get the highest resolution mode that follows either RGB or BGR
        let mut best_mode = None;
        for mode in modes {
            
            let mode = mode.unwrap();
            match mode.info().pixel_format() {
                PixelFormat::RGB | PixelFormat::BGR => {
                    match best_mode {
                        None => best_mode = Some(mode),
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
            frame_buffer: Volatile::new(graphics_output.frame_buffer().as_mut_ptr(),
            current_mode: best_mode);
        }
    }

    /// Fill the screen with a certain color
    pub fn fill_screen(&self, color: Color) {
        let (width, height) = self.current_mode.info().resolution();
        draw_rectangle(color, width, height, 0, 0);
    }

    /// Creates a new color that follows the format of the current graphics mode
    pub fn new_color(&self, red: u8, blue: u8, green: u8) -> Color {
        Color::new(red, blue, green, self.current_mode.info().pixel_format())
    }

    /// Draws a rectangle with the top-left corner at (x, y) 
    /// and with the specified height, width, and color
    pub fn draw_rectangle(&mut self, color: Color, 
                          height: usize, width: usize, x: usize, y: usize) {
        let (screen_width, screen_height) = self.current_mode.info().resolution();
        
        // Optimize if possible
        if (x == 0) & (width == screen_width) {
        }
        
        let vec = vec![[color; width]; height];
    }

    /// Draws a circle with the center at (x, y) and with the specified
    /// radius and color
    pub fn draw_circle(&mut self, color: Color, radius: usize, x: usize, y: usize) {

    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
/// A structure representing a formatted color
pub struct Color {
    inner: [u8; 4],
}
impl Color {
    /// Creates a new color object following a given pixel-format
    pub fn new(red: u8, blue: u8, green: u8, format: PixelFormat) -> Color {
        match format {
            PixelFormat::RGB => Color { inner: [red, blue, green, 0] },
            PixelFormat::BGR => Color { inner: [blue, green, red, 0] },
            _ => panic!("Unkown format: {:?}", format),
        }
    }
}
