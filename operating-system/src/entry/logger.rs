use log::{Record, Level, Metadata, LevelFilter, info};
use core::fmt::Write;
use super::ST;

pub static UEFI_LOGGER: UefiLogger = UefiLogger;

pub struct UefiLogger;
impl log::Log for UefiLogger {
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
            );
        }

        if record.level() == Level::Error {
            loop {}
        }
    }
    
    fn flush(&self) {}
}
