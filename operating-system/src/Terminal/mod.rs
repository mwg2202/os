#![no_std]
#![feature(asm)]
#![feature(abi_efiapi)]

use alloc::string::String;

struct Terminal {
    input_string: mut String;
} impl Terminal {
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

struct Commands {
} impl Commands {

    fn shutdown(st: &SystemTable<Boot>) {
        shutdown(&st);
    }
    fn ls() {}
    fn cd() {}
    fn rm() {}
    fn load() {}
    fn gop() {}

    pub fn parse_command(st: &SystemTable<Boot>, args: &[&str]) {
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
