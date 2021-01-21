/// An instance of a process
pub struct ProcessInstance {
    /// The Process ID associated with the instance
    pid: u8,
    
    /// A vector of the windows owned by the application
    windows: Vec<Window>,
}

/// A window
pub struct Window {
    /// The Process ID of the program that owns the window
    pid: u8,
    
    width: u16,
    height: u16,
    
    /// The (x, y) location of the window
    location: (i16, i16),
    
    /// A pointer to the buffer that the window can draw to
    buffer: Box<Buffer>,
    
    status: WindowStatus,
} 

enum WindowStatus {
    open,
    minimized,
    fullscreen,
}


pub struct WindowManager {
    screen: Screen,
    windows: Vec<Window>,
} impl WindowManager {
    /// Creates a new WindowManager object
    pub fn new(screen: Screen) {
        WindowManager {
            screen,
            windows: Vec<Box<Window>>::new(),
        }
    }

    pub fn create_window(&self, pid: u8, size: Size, location: Location) {
        window = Window {
            pid,
            size,
            location,
            buffer: Box(Buffer::new(size)),
            status: WindowStatus::open,
        }
        self.windows.push(Box::new(window));
    }
    pub fn destroy_window(&self, window: &Window) {
        for (i, w) in self.windows.enumerate() {
            if &w == &window {
                self.windows.pop(i);
            }
        }
    }
    fn draw(&self) {
        // Draw the gui
        fill_buffer(self.screen, Color::new(0, 0, 0));
        
        // Draw the windows
        for window in windows {
            if window.status == minimized {continue;}
            block_transfer(window.buffer, screen, window.location)
        }
    }
}

pub struct Location {
    x: isize,
    y: isize,
}

pub struct Size {
    width: usize,
    height: usize,
}
