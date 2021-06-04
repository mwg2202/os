use log::{Record, Level, Metadata, LevelFilter};
use core::fmt::{Write, Debug};
use super::ST;
use uefi::ResultExt;

pub static UEFI_LOGGER: UefiLogger = UefiLogger;

pub struct UefiLogger;
impl UefiLogger {
    pub fn init() {
        if let Some(st) = unsafe { ST.as_ref() } {
            // Reset the console
            st.stdout()
                .reset(false)
                .expect_success("Failed to reset output buffer");
            
            // Set the default logger
            log::set_logger(&UEFI_LOGGER)
                .map(|()| log::set_max_level(LevelFilter::Trace))
                .expect("Failed to setup logging using UEFI");
        }
    }
} impl log::Log for UefiLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }
    fn log(&self, record: &Record) {
        if let Some(st) = unsafe { ST.as_ref() } {
            writeln!(
                st.stdout(),
                "[{}] {}",
                record.level(),
                record.args()
            ).unwrap();
        }
        if record.level() == Level::Error { loop {} }
    }
    fn flush(&self) {}
}

pub fn _crash(string: &dyn Debug) -> ! {
    if let Some(st) = unsafe { ST.as_ref() } {
        writeln!(st.stdout(), "FATAL ERROR: {:?}", string).unwrap();
    }
    loop {}
}
