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



pub struct Buffer {
    /// The pixels making up the buffer
    pixels: Vec<Vec<Pixel>>,

    /// The format of each pixel in the buffer
    format: PixelFormat,
}
impl Buffer {
    pub fn new(size: Size, format: PixelFormat) -> Buffer {

        // Creates a black pixel to be used with the vec! macro
        let color = Color::new(0, 0, 0);
        let pixel = Pixel::new(color, format);

        // Creates the buffer object
        Buffer {
            pixels: vec![vec![pixel; size.width]; size.height],
            format,
        }
    }
}
impl GraphicsBuffer for Buffer {
    fn size(&self) -> Size {
        self.size
    }
    fn ptr(&self) -> *mut Pixel {
        self.ptr
    }
    fn format(&self) -> PixelFormat {
        self.fmt
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

