extern crate alloc;
use super::{Size, PixelFormat, Pixel, Color, Location};
use alloc::vec::Vec;
use alloc::vec;
use rusttype::{Font, point, Scale};

#[derive(Debug)]
pub struct Buffer {
    /// The pixels making up the buffer
    pixels: Vec<Pixel>,

    /// The size of the buffer
    size: Size,

    /// The format of each pixel in the buffer
    fmt: PixelFormat,
}

impl Buffer {
    pub fn new(size: Size, fmt: PixelFormat) -> Buffer {
        // Creates a black pixel to be used with the vec! macro
        let color = Color::new(0, 0, 0);
        let pixel = Pixel::new(color, fmt);

        // Creates the buffer object
        Buffer {
            pixels: vec![pixel; size.width * size.height],
            size,
            fmt,
        }
    }
}
impl BufferTrait for Buffer {
    fn size(&self) -> Size {
        self.size
    }
    fn ptr(&mut self) -> *mut Pixel {
        self.pixels.as_mut_ptr()
    }
    fn fmt(&self) -> PixelFormat {
        self.fmt
    }
}
pub trait BufferTrait {
    fn size(&self) -> Size;
    fn ptr(&mut self) -> *mut Pixel;
    fn fmt(&self) -> PixelFormat;
    
    /// Creates a new color that follows the format of the buffer
    fn new_pixel(&self, color: Color) -> Pixel {
        Pixel::new(color, self.fmt())
    }

    fn color_from_pixel(&self, p: Pixel) -> Color {
        match self.fmt() {
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
            _ => panic!("Unkown format: {:?}", self.fmt()),
        }
    }

	/// Fills a buffer with a certain color
	fn fill(&mut self, color: Color) {
	    let pixel = self.new_pixel(color);
	    let size = self.size();
		let ptr = self.ptr();
	    for i in 0..(size.width * size.height) {
	        unsafe {
	            core::ptr::write_volatile(ptr.offset(i as isize), pixel);
	        }
	    }
	}

    /// Write a string to a buffer
    fn write_text(
		&mut self, 
		string: &str, 
		loc: Location, 
		font: &Font, 
		height: f32, 
		c: Color
	) {        
        let (buffer_width, _h) = self.size().tuple();
        let buffer_width = buffer_width as isize;

        let ptr = unsafe { 
            self.ptr().offset(loc.y * buffer_width + loc.x) 
        };

        let scale = Scale {
            x: height,
            y: height,
        };
        
        let offset = point(0.0, 0.0);
        let glyphs = font.layout(string, scale, offset);
    
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

    /// Copies src_buffer to self
    /// This treats src_buffer as a "block" of memory and copies
    /// it as if it was drawing a rectangle to dst_buffer
    fn block_transfer(&mut self, src_buffer: &mut dyn BufferTrait, loc: Location) {
    	let (mut x, mut y) = loc.tuple();
        let (dst_width, dst_height) = self.size().tuple();
        let (block_width, block_height) = src_buffer.size().tuple();
        let dst_width = dst_width as isize;
        let dst_height = dst_height as isize;
        let mut block_width = block_width as isize;
        let mut block_height = block_height as isize;
    
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
    
        let mut dst_ptr = unsafe { self.ptr().offset(y * dst_width + x) };
        let mut src_ptr = src_buffer.ptr();
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
}
