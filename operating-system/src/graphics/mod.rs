use core::convert::TryInto;
use uefi::prelude::*;
use uefi::proto::console::gop::GraphicsOutput;
use uefi::proto::console::gop::PixelFormat;
use uefi::table::boot::BootServices;
use rusttype::{Font, Scale, point};
use libm::ceilf;
pub mod fonts;
pub mod vga;
use core::convert::FloatToInt;

/// Fills a buffer with a certain color
pub fn fill_buffer(buffer: &GraphicsBuffer, color: Color) {
    let pixel = buffer.new_pixel(color);
    let (width, height) = buffer.size;
    for i in 0..(width * height) {
        unsafe {
            core::ptr::write_volatile(buffer.ptr.offset(i.try_into().unwrap()), pixel);
        }
    }
}

/// Copies src_buffer to dst_buffer
/// This treats src_buffer as a "block" of memory and copies
/// it as if it was drawing a rectangle to dst_buffer
pub fn block_transfer(
    src_buffer: &GraphicsBuffer,
    dst_buffer: &GraphicsBuffer,
    mut x: isize,
    mut y: isize,
) {
    let (dst_width, dst_height) = dst_buffer.size;
    let (block_width, block_height) = src_buffer.size;
    let dst_width: isize = dst_width.try_into().unwrap();
    let dst_height: isize = dst_height.try_into().unwrap();
    let mut block_width: isize = block_width.try_into().unwrap();
    let mut block_height: isize = block_height.try_into().unwrap();

    // If the rectangle would be drawn partly off screen (top)
    if y < 0 {
        block_height += y;

        // If the rectangle would be drawn completely off screen (top)
        if block_height < 0 {
            return;
        }
        y = 0;
    }

    // If the rectangle is drawn partly off screen (left)
    if x < 0 {
        block_width += x;

        // If the rectangle would be drawn completely off screen (left)
        if block_width < 0 {
            return;
        }
        x = 0;
    }

    // If the rectangle would be drawn completely off screen (bottom or right)
    if (y > dst_height) | (x > dst_width) {
        return;
    }

    // If the rectangle is drawn partly off screen (bottom)
    if dst_height < (y + block_height) {
        block_height = dst_height - y;
    }

    // If the rectangle is drawn partly off screen (right)
    if dst_width < (x + block_width) {
        block_width = dst_width - x;
    }

    let mut dst_ptr = unsafe { dst_buffer.ptr.offset(y * dst_width + x) };
    let mut src_ptr = src_buffer.ptr;
    for _ in 0..block_height {
        for i in 0..block_width {
            // Write a pixel from the destination buffer to the source buffer
            unsafe {
                core::ptr::write_volatile(dst_ptr.offset(i), *src_ptr.offset(i));
            }
        }
        dst_ptr = unsafe { dst_ptr.offset(dst_width) };
        src_ptr = unsafe { src_ptr.offset(block_width) };
    }
}


pub struct GraphicsBuffer {
    /// A pointer to the buffer
    ptr: *mut Pixel,
    /// The size of the buffer in pixels in (width, height)
    size: (usize, usize),
    /// The format of each pixel in the buffer
    format: PixelFormat,
}
impl GraphicsBuffer {
    /// Creates a new color that follows the format of the buffer
    pub fn new_pixel(&self, color: Color) -> Pixel {
        Pixel::new(color, self.format)
    }

    pub fn color_from_pixel(&self, p: Pixel) -> Color {
        match self.format {
            PixelFormat::RGB => Color {
                red: p.inner[0],
                green: p.inner[1],
                blue: p.inner[2],
            },
            PixelFormat::BGR => Color {
                blue: p.inner[0],
                green: p.inner[1],
                red: p.inner[2],
            },
            _ => panic!("Unkown format: {:?}", self.format),
        }
    }
    pub fn init(bs: &BootServices) -> GraphicsBuffer {
        // Get the graphics output protocol
        let graphics_output = bs
            .locate_protocol::<GraphicsOutput>()
            .unwrap_success()
            .get();

        let graphics_output = unsafe { &mut *(graphics_output) };

        // Get the list of modes
        let modes = graphics_output.modes();

        // Get the highest resolution mode that follows either RGB or BGR
        let mut best_mode = None;
        for mode in modes {
            let mode = mode.log();
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
                        }
                    }
                }
                _ => (),
            }
        }

        // Get best_mode from the option
        let best_mode = best_mode.unwrap();

        // Set the graphics mode to said mode
        graphics_output.set_mode(&best_mode).unwrap_success();

        // Make a structure out of the information
        GraphicsBuffer {
            ptr: graphics_output.frame_buffer().as_mut_ptr() as *mut Pixel,
            size: best_mode.info().resolution(),
            format: best_mode.info().pixel_format(),
        }
    }

    /// Write a string to a buffer
    pub fn write_text(&self, string: &str, x: isize, 
                      y: isize, font: &Font, height: f32, c: Color) {
        
        let (buffer_width, buffer_height) = self.size;
        let buffer_width = buffer_width as isize;

        let ptr = unsafe { 
            self.ptr.offset(y * buffer_width + x) 
        };

        let scale = Scale {
            x: height,
            y: height,
        };
        let v_metrics = font.v_metrics(scale);
        //let offset = point(0.0, v_metrics.ascent); 
        let offset = point(0.0, 0.0);
        let glyphs = font.layout(string, scale, offset);
        /*
        let width = glyphs.rev().map(|g| g.position().x as f32 + 
            g.unpositioned().h_metrics().advance_width).next().unwrap_or(0.0)
            .ceil() as usize;
        */
    
        // Go through each glyph in the string
        for g in glyphs {
            if let Some(bb) = g.pixel_bounding_box() {
                
                // Draw the glyph
                g.draw(|x, y, v| {
                    let x = (x as i32 + bb.min.x) as isize;
                    let y = (y as i32 + bb.min.y) as isize;
                    
                    // Gets the pixel that was previously in this location
                    let old_p = unsafe { 
                        core::ptr::read_volatile(ptr.offset(x + y * buffer_width))
                    };
                    let old_c = self.color_from_pixel(old_p);

                    // Gets the old colors from the pixel
                    let or = old_c.red as f32;
                    let og = old_c.green as f32;
                    let ob = old_c.blue as f32;

                    // Gets the font-color
                    let r = c.red as f32;
                    let g = c.green as f32;
                    let b = c.blue as f32;

                    // Calculates the new color values
                    let nr = ((or * (1.0-v)) + (r * v)) as u8;
                    let ng = ((og * (1.0-v)) + (g * v)) as u8;
                    let nb = ((ob * (1.0-v)) + (b * v)) as u8;

                    unsafe {
                        core::ptr::write_volatile(
                            ptr.offset(x + y * buffer_width), 
                            self.new_pixel(Color::new(nr, ng, nb))
                        );
                    }
                })
            }
        }
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
/// A structure representing a formatted color
pub struct Pixel {
    inner: [u8; 4],
}
impl Pixel {
    /// Creates a new color object following a given pixel-format
    pub fn new(c: Color, format: PixelFormat) -> Pixel {
        match format {
            PixelFormat::RGB => Pixel {
                inner: [c.red, c.green, c.blue, 0],
            },
            PixelFormat::BGR => Pixel {
                inner: [c.blue, c.green, c.red, 0],
            },
            _ => panic!("Unkown format: {:?}", format),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
} impl Color {
    pub fn new(red: u8, green: u8, blue: u8) -> Color {
        Color {
            red,
            green,
            blue,
        }
    }
}
