pub use uefi::proto::console::gop::PixelFormat;

#[derive(Debug, Copy, Clone)]
pub struct Location {
    pub x: isize,
    pub y: isize,
}
impl Location {
    pub fn tuple(&self) -> (isize, isize) { (self.x, self.y) }
}

#[derive(Debug, Copy, Clone)]
pub struct Size {
    pub width: usize,
    pub height: usize,
}
impl Size {
    pub fn tuple(&self) -> (usize, usize) { (self.width, self.height) }
}

#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}
impl Color {
    pub fn new(red: u8, green: u8, blue: u8) -> Color { Color { red, green, blue } }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
/// A structure representing a formatted color
pub struct Pixel {
    pub inner: [u8; 4],
}
impl Pixel {
    /// Creates a new color object following a given pixel-format
    /// This method only works for RGB and BGR pixel-formats
    pub fn new(c: Color, fmt: PixelFormat) -> Pixel {
        match fmt {
            PixelFormat::Rgb => Pixel {
                inner: [c.red, c.green, c.blue, 0],
            },
            PixelFormat::Bgr => Pixel {
                inner: [c.blue, c.green, c.red, 0],
            },
            _ => panic!("Unkown format: {:?}", fmt),
        }
    }
}
