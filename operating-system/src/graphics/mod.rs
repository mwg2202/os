use uefi::proto::console::gop::GraphicsOutput;
use uefi::proto::console::gop::Mode;
use uefi::prelude::*;
use uefi::table::boot::BootServices;
use uefi::proto::console::gop::PixelFormat;
use core::convert::TryInto;

pub struct GraphicsSystem<'a> {
    current_mode: Mode,
    graphics_output: &'a mut GraphicsOutput<'a>,
    frame_buffer: *mut Color,
}
impl GraphicsSystem<'_> {
    /// Set up the the graphics system
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
                            let (bw, bh) = m.info().resolution();
                            
                            if (width > bw) || (height > bh) {
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
            current_mode: best_mode,
            frame_buffer: graphics_output.frame_buffer().as_mut_ptr() as *mut Color,
            graphics_output: graphics_output,
        }
    }

    /// Fill the screen with a certain color
    pub fn fill_screen(&mut self, color: Color) { 
        let (width, height) = self.current_mode.info().resolution();
        self.draw_rectangle(color, width, height, 0, 0);
    
    }

    /// Creates a new color that follows the format of the current graphics mode
    pub fn new_color(&self, red: u8, blue: u8, green: u8) -> Color {
    
        Color::new(red, blue, green, self.current_mode.info().pixel_format())
    
    }

    /// Draws a rectangle with the top-left corner at (x, y) 
    /// and with the specified height, width, and color
    pub fn draw_rectangle(&mut self, color: Color, 
        width: usize, height: usize, x: isize, y: isize) {
        
        let (screen_width, screen_height) = self.current_mode.info().resolution();
        let screen_width: isize = screen_width.try_into().unwrap();
        let screen_height: isize = screen_height.try_into().unwrap();

        // Check if the rectangle will be out of range
        let mut y = y;
        let mut x = x;
        let mut height: isize = height.try_into()
            .expect("A height too large was specified");
        let mut width: isize = width.try_into()
            .expect("A width too large was specified");

        // If the rectangle would be drawn partly off screen (top)
        if y < 0 {
            height += y;
                
            // If the rectangle would be drawn completely off screen (top)
            if height < 0 {
                return;
            }
            y = 0; 
        }

        // If the rectangle is drawn partly off screen (left)
        if x < 0 {
            width += x;

            // If the rectangle would be drawn completely off screen (left)
            if width < 0 {
                return;
            }
            x = 0;
        }
        
        // If the rectangle would be drawn completely off screen (bottom or right)
         if (y > screen_height) | (x > screen_width){
            return;
        }
            
        // If the rectangle is drawn partly off screen (bottom)
        if screen_height < (y + height) {
            height = screen_height - y;
        }

        // If the rectangle is drawn partly off screen (right)
        if screen_width < (x + width) {
            width = screen_width - x;
        }

        // Optimize if possible
        if (x == 0) & (width == screen_width) {
            let ptr = unsafe { self.frame_buffer.offset(y * screen_width) };
            for i in 0..(height * width) {
                unsafe { core::ptr::write_volatile(ptr.offset(i), color); }
            }
        } else {
            let mut ptr = unsafe { self.frame_buffer.offset(y * screen_width + x) };
            for _ in 0..height {
                for i in 0..width {
                    unsafe { core::ptr::write_volatile(ptr.offset(i), color); }
                }
                ptr = unsafe { ptr.offset(screen_width) };
            }
        }
    }

    /// Draws a circle with the center at (x, y) and with the specified
    /// radius and color.
    pub fn draw_circle(&mut self, color: Color, radius: usize, x: isize, y: isize) {
        
        let (screen_width, screen_height) = self.current_mode.info().resolution();
        let screen_width: isize = screen_width.try_into().unwrap();
        let screen_height: isize = screen_height.try_into().unwrap();
        let radius: isize = screen_height.try_into()
            .expect("A radius too large was spciefied");

        // This points to the far-left point of the circle
        let ptr = unsafe { self.frame_buffer.offset(y * screen_width + x - radius) };
        unsafe { core::ptr::write_volatile(ptr, color); }
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
