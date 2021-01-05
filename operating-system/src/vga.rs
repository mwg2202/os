use spinning_top::Spinlock;
use volatile::Volatile;
use lazy_static::lazy_static;


/// The colors supported by the vga protocol
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {

    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magneta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,

}

/// This represents the first byte of a cell in the vga buffer (the byte 
/// that tells the foreground and the background colors)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);
impl ColorCode {
    // Returns a byte with the first four bits representing the background color
    // and the next four representing the foreground color
    const fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

/// This repressents a letter (with its corresponding color) in the vga buffer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct BufferCell {
    ascii_character: u8,
    color_code: ColorCode,
}


// These represent the height width of the buffer
const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

/// This represents the vga buffer
#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<&'static mut BufferCell>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}


/// This represents the vga handler
pub struct Handler {

    // The current row and col to write at
    row: usize,
    col: usize,

    // The vga buffer
    buffer: &'static mut Buffer,

    // Holds the default color code
    color_code: ColorCode,

}
impl Handler {

    
    /// Writes an ascii char onto the vga buffer
    fn write_char(&mut self, c: u8, color_code: ColorCode) {
        
        self.buffer.chars[self.row][self.col].write(BufferCell{
            ascii_character: c,
            color_code,
        });
        self.col += 1;
    
    }
    
    /// Writes a string onto the vga buffer
    pub fn write_str(&mut self, s: &str) {

        for byte in s.bytes() {
            // If the byte is a letter output it, if not write 0xfe
            match byte {
                0x20..=0x7e | b'\n' => self.write_char(byte, self.color_code),
                _ => self.write_char(0xfe, self.color_code),
            }
        } 
    }
}

lazy_static! {
    pub static ref HANDLER: Spinlock<Handler> = Spinlock::new(
        Handler {
            row: 0,
            col: 0,
            buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
            color_code: ColorCode::new(Color::Yellow, Color::Black),
        }
    );
}
