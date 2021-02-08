extern crate alloc;
use super::{Color, Screen, Size, Location, Buffer, BufferTrait, PixelFormat};
use alloc::vec::Vec;

#[derive(Debug)]
pub struct ProcessInstance {
    /// The Process ID associated with the instance
    pid: u8,
    
    /// A vector of the windows owned by the application
    windows: Vec<Window>,
}

#[derive(Debug)]
pub struct Window {
    /// The Process ID of the program that owns the window
    pid: u8,
    
    /// The location of the top-left corner of the window
    location: Location,
    
    /// A pointer to the buffer that the window can draw to
    buffer: Buffer,
   
    /// The current status of the window
    status: WindowStatus,
} 

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum WindowStatus {
    Open,
    Minimized,
    Fullscreen,
}


#[derive(Debug)]
pub struct WindowManager {
    windows: Vec<Window>,
} impl WindowManager {
    /// Creates a new WindowManager object
    pub fn new() -> WindowManager {
        WindowManager {
            windows: Vec::<Window>::new(),
        }
    }
    pub fn create_window(&mut self, pid: u8, size: Size, location: Location, fmt: PixelFormat) {
        let mut buffer = Buffer::new(size, fmt);
        buffer.fill(Color::new(255, 255, 255));
        let window = Window {
            pid,
            location,
            buffer,
            status: WindowStatus::Open,
        };
        self.windows.push(window);
    }
    pub fn destroy_window(&mut self, pid: u8) {
        // Get the index of the specified window
        let index = self.windows.iter().position(|w| w.pid == pid);

        // Delete the window from the vector
        match index {
            Some(index) => {self.windows.remove(index);},
            None => (),
        }
    }
    pub fn draw(&mut self, screen: &mut Screen) {
        // Draw the gui
        screen.fill(Color::new(0, 0, 0));
        
        // Draw the windows
        for i in 0..self.windows.len() {
            if self.windows[i].status == WindowStatus::Minimized {continue;}
            let loc = self.windows[i].location;
            screen.block_transfer(&mut self.windows[i].buffer, loc);
        }
    }
}
