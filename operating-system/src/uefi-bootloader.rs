#![no_std]
#![no_main]
#![feature(asm)]
#![feature(abi_efiapi)]

extern crate alloc;
use uefi::prelude::*;
//use uefi::proto::media::fs::SimpleFileSystem;
use uefi::proto::console::gop::GraphicsOutput;
//use uefi::proto::pi::mp::MPServices;
use alloc::string::String;
use alloc::vec::Vec;
mod Graphics;
use Graphics::GraphicsSystem;

#[entry]
fn efi_main(image: uefi::Handle, st: SystemTable<Boot>) -> Status {
    // Initialize utilities (logging, memory allocation...)
    uefi_services::init(&st).expect_success("Failed to initialize utilities");

    // Reset the console
    st.stdout()
        .reset(false)
        .expect_success("Failed to reset output buffer");

    let bs = st.boot_services();
    let rs = st.runtime_services();
    
    // Print some information
    log::info!("Welcome...");
    log::info!("UEFI Version: {:?}", st.uefi_revision());
    log::info!("Time: {:?}", rs.get_time().unwrap_success());
    
    // Initialize the graphics system
    let mut gs = GraphicsSystem::init(&bs);

    TextModeServices::get_command(&st);
    Status::SUCCESS
}

/// Exits uefi boot services
fn exit_boot_services(st: SystemTable<Boot>, image: uefi::Handle) {
    use uefi::table::boot::MemoryDescriptor;
    use core::mem;
    use alloc::vec;
    let max_mmap_size =
        st.boot_services().memory_map_size()+8*mem::size_of::<MemoryDescriptor>();
    let mut mmap_storage = vec![0; max_mmap_size].into_boxed_slice();
    let (st, _iter) = st
        .exit_boot_services(image, &mut mmap_storage[..])
        .expect_success("Failed to exit boot services");
}

/// Shutdowns the device after the next key press
pub fn shutdown_on_keypress(st: &SystemTable<Boot>) {
    use uefi::table::runtime::ResetType;
    st.boot_services()
        .wait_for_event(&mut [st.stdin().wait_for_key_event()])
        .unwrap_success();

    st.runtime_services().reset(ResetType::Shutdown, Status::SUCCESS, None);
}

/// Shutsdown the device
pub fn shutdown(st: &SystemTable<Boot>) {
    use uefi::table::runtime::ResetType;
    st.runtime_services().reset(ResetType::Shutdown, Status::SUCCESS, None);    
}

struct Commands {}
impl Commands {
    fn shutdown(st: &SystemTable<Boot>) {
        shutdown(&st);
    }
    fn ls() {}
    fn cd() {}
    fn rm() {}
    fn load() {}
    fn gop() {}

    pub fn execute_command(st: &SystemTable<Boot>, args: &[&str]) {
        match args {
            ["shutdown"] => Commands::shutdown(&st),
            ["ls"] => Commands::ls(),
            ["cd"] => Commands::cd(),
            ["rm"] => Commands::rm(),
            ["load"] => Commands::load(),
            ["gop"] => Commands::gop(),
            _ => log::info!("Command Not Found: {}", args[0]),
        }
    }
}

struct GOPMethods {}
impl GOPMethods {
    pub fn print_graphics_modes(st: &SystemTable<Boot>) {
        use core::fmt::Write;
        let protocol_ptr = st.boot_services().locate_protocol::<GraphicsOutput>()
            .expect_success("Failed to locate the GraphicsOutput protocal");
        for mode in unsafe { &*protocol_ptr.get() }.modes() {
            write!(st.stdout(), "Mode: {:?}", mode.unwrap().info());
        }
    }
}

struct TextModeServices {}
impl TextModeServices {
    // Returns an Error if not in text mode
    pub fn get_command(st: &SystemTable<Boot>) {
        use core::fmt::Write;
        use uefi::proto::console::text::Key;
        st.stdout().enable_cursor(true)
            .expect_success("The output device does not support making the cursor visible/invisible");
        loop {
            write!(st.stdout(), ">\t").unwrap();
            let mut input_string = String::new();
            loop {
                st.boot_services()
                    .wait_for_event(&mut [st.stdin().wait_for_key_event()] )
                    .unwrap_success();
                match st.stdin().read_key().unwrap_success().unwrap() {
                    Key::Printable(c) => {
                        write!(st.stdout(), "{}", c).unwrap(); 
                        match Into::<char>::into(c) {
                            '\r' => {
                                let (col, row) = st.stdout().cursor_position();
                                let _ = st.stdout().set_cursor_position(col, row + 1);
                                break;
                            },
                            '\u{8}' => drop(input_string.pop()),
                            foo => input_string.push(foo),
                        }
                    }
                    _ => (),
                }
            }
            Commands::execute_command(&st, 
                &input_string.split_whitespace().collect::<Vec<_>>());

            input_string.clear();
        }
    }
}
