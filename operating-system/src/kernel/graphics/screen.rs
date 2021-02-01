use super::{Pixel, PixelFormat, Size, BufferTrait};
use uefi::table::boot::BootServices;
use uefi::proto::console::gop::GraphicsOutput;
use uefi::ResultExt;

#[derive(Debug)]
pub struct Screen {
    ptr: *mut Pixel,
    size: Size,
    fmt: PixelFormat,
}

impl Screen {
    pub fn init(bs: &BootServices) -> Screen {
        // Get the graphics output protocol
        let graphics_output = bs.locate_protocol::<GraphicsOutput>()
            .unwrap_success().get();

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

        let (width, height) = best_mode.info().resolution();
        let size = Size {
            width,
            height,
        };

        // Make a structure out of the information
        Screen {
            ptr: graphics_output.frame_buffer().as_mut_ptr() as *mut Pixel,
            size,
            fmt: best_mode.info().pixel_format(),
        }
    }
}
impl BufferTrait for Screen {
    fn size(&self) -> Size {
        self.size
    }
    fn ptr(&mut self) -> *mut Pixel {
        self.ptr
    }
    fn fmt(&self) -> PixelFormat {
        self.fmt
    }
}
