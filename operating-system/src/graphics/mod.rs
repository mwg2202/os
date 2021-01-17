use core::convert::TryInto;
use uefi::prelude::*;
use uefi::proto::console::gop::GraphicsOutput;
use uefi::proto::console::gop::PixelFormat;
use uefi::table::boot::BootServices;

/// Fills a buffer with a certain color
pub fn fill_buffer(buffer: &GraphicsBuffer, color: Pixel) {
    let (width, height) = buffer.size;
    for i in 0..(width * height) {
        unsafe {
            core::ptr::write_volatile(buffer.ptr.offset(i.try_into().unwrap()), color);
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

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
/// A structure representing a formatted color
pub struct Pixel {
    inner: [u8; 4],
}
impl Pixel {
    /// Creates a new color object following a given pixel-format
    pub fn new(red: u8, blue: u8, green: u8, format: PixelFormat) -> Pixel {
        match format {
            PixelFormat::RGB => Pixel {
                inner: [red, blue, green, 0],
            },
            PixelFormat::BGR => Pixel {
                inner: [blue, green, red, 0],
            },
            _ => panic!("Unkown format: {:?}", format),
        }
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
    pub fn new_color(&self, red: u8, blue: u8, green: u8) -> Pixel {
        Pixel::new(red, blue, green, self.format)
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
}
