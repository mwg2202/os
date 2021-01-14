pub struct ApplicationInstance {
    aid: u8,
    /// A vector of the windows owned by the application
    child_windows: Vec<&Window>,
    page_table
}
impl Program {
    pub fn start() {
    }
    pub fn end() {
        match window {
            Some(window) => match window.status {
                maximized => (),
                minimized => (),
                fullscreen => (),
            },
            None => (),
        }
    }
}

pub struct Window {
    /// The Program ID of the program that owns the window
    pid: u8,
    /// A unique identifier for this window
    wid: u8,
    width: u16,
    height: u16,
    /// The (x, y) location of the window
    location: (i16, i16),
    /// A pointer to the buffer that the window can draw to
    buffer: *mut [Color],
    status: WindowStatus,
}

enum WindowStatus {
    open,
    minimized,
    fullscreen,
}

// Hardware Device
pub trait InputDevice {
    pub fn write(data: wrtie);
}
pub trait OutputDevice {
    pub fn read(data: data);
}

pub struct WindowManager {
    windows: Vec<Window>,
} impl WindowManager {
    create_window(aid: u8) {

    }
    destroy_window(wid: u8) {

    }
}
