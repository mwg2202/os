#![no_std]
#![no_main]
#![feature(asm)]
#![feature(abi_efiapi)]
#![feature(type_ascription)]

extern crate alloc;
use uefi::prelude::*;
use uefi::proto::media::fs::SimpleFileSystem;
use uefi::proto::console::gop::GraphicsOutput;
use uefi::proto::pi::mp::MPServices;
use alloc::string::String;

#[entry]
fn efi_main(_img: uefi::Handle, st: SystemTable<Boot>) -> Status {
    // Initialize utilities (logging, memory allocation...)
    uefi_services::init(&st).expect_success("Failed to initialize utilities");

    // Reset the console
    st.stdout()
        .reset(false)
        .expect_success("Failed to reset output buffer");

    let bs = st.boot_services();
    let rs = st.runtime_services();
    let stdin = st.stdin();
    
    // get protocols
    let mut fs = unsafe { &mut *(bs.locate_protocol::<SimpleFileSystem>()
        .unwrap_success().get()) };
    let mut gop = unsafe { &mut *(bs.locate_protocol::<GraphicsOutput>()
        .unwrap_success().get()) };
    let mut mp = unsafe { &mut *(bs.locate_protocol::<MPServices>()
        .unwrap_success().get()) };


    // Print some information
    log::info!("Welcome...");
    log::info!("UEFI Version: {:?}", st.uefi_revision());
    log::info!("Time: {:?}", rs.get_time().unwrap_success()); 
    
    // Wait for commands
    use core::fmt::Write;
    use uefi::proto::console::text::Key;
    use uefi::proto::console::text::ScanCode;
    st.stdout().enable_cursor(true)
        .expect_success("The output device does not support making the cursor visible/invisible");
    loop {
        write!(st.stdout(), ">\t").unwrap();
        let mut input_string = String::new();
        loop {
            bs.wait_for_event(&mut [st.stdin().wait_for_key_event()])
                .unwrap_success();
            match stdin.read_key().unwrap_success().unwrap() {
                Key::Printable(c) => {
                    write!(st.stdout(), "{}", c).unwrap(); 
                    if (c.into(): char) == '\r' {
                        let (col, row) = st.stdout().cursor_position();
                        if row < 50 {
                            st.stdout().set_cursor_position(col, row + 1)
                                .unwrap_success();
                        }
                        break;
                    } else if (c.into(): char) == '\u{8}' {
                        input_string.pop();    
                    } else {
                        input_string.push(c.into());
                    }
                }
                Key::Special(ScanCode::LEFT) => {
                    let (col, row) = st.stdout().cursor_position();
                    if col > 0 {
                        st.stdout().set_cursor_position(col - 1, row)
                            .unwrap_success();
                    }
                },
                Key::Special(ScanCode::RIGHT) => {
                    let (col, row) = st.stdout().cursor_position();
                    if col < 50 {
                        st.stdout().set_cursor_position(col + 1, row)
                            .unwrap_success();
                    }
                },
                Key::Special(ScanCode::UP) => {
                    let (col, row) = st.stdout().cursor_position();
                    if row > 0 {
                        st.stdout().set_cursor_position(col, row - 1)
                            .unwrap_success();
                    }
                },
                Key::Special(ScanCode::DOWN) => {
                let (col, row) = st.stdout().cursor_position();
                    if row < 50 {
                        st.stdout().set_cursor_position(col, row + 1)
                            .unwrap_success();
                    }
                },
                Key::Special(c) => write!(st.stdout(), "{:?}", c).unwrap(),
            }
        }
        execute_command(&st, &input_string);
        input_string.clear();
    }
}

fn execute_command(st: &SystemTable<Boot>, input_string: &String) {
    let mut args = input_string.split_whitespace();
    let command = args.next();
    match command {

        // Shutdown the system
        Some("shutdown") => {
            let curr_arg = args.next(); 
            match curr_arg {
                None => st.runtime_services()
                    .reset(ResetType::Shutdown, Status::SUCCESS, None),
                _ => log::info!("Unrecognized argument \"{}\" for command \"{}\".", curr_arg.unwrap(), command.unwrap()),
            }
        },


        Some("ls") => log::info!("Not implemented"),
        Some("cd") => log::info!("Not implemented"),
        Some("mv") => log::info!("Not implemented"),
        Some("rm") => log::info!("Not implemented"),
        Some("load") => log::info!("Not implemented"),
        _ => log::info!("Command Not Found: {}", input_string),
    }
}

use uefi::table::runtime::ResetType;
fn shutdown_on_keypress(st: &SystemTable<Boot>) {
    // Shutdown device after key press
    st.boot_services()
        .wait_for_event(&mut [st.stdin().wait_for_key_event()])
        .unwrap_success();

    let status: Status = Status::SUCCESS;
    st.runtime_services()
        .reset(ResetType::Shutdown, status, None);
}

fn print_graphics_modes(st: &SystemTable<Boot>) {
    let protocol_ptr = st.boot_services().locate_protocol::<GraphicsOutput>()
        .expect_success("Failed to locate the GraphicsOutput protocall");
    for mode in unsafe { &*protocol_ptr.get() }.modes() {
        log::info!("Mode: {:?}", mode.unwrap().info());
    }
}

fn change_graphics_mode(st: &SystemTable<Boot>, index: u16) {
    let protocol_ptr = st.boot_services().locate_protocol::<GraphicsOutput>()
        .expect_success("Failed to locate the GraphicsOutput protocall");
    let mut iter = unsafe{ &*protocol_ptr.get() }.modes();
     
    for i in 0..index {
        iter.next();
        log::info!("#{}", i);
    }

    unsafe { &mut * protocol_ptr.get() }.set_mode(&iter.next().unwrap().unwrap());
} 
