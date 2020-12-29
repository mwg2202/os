#![no_std]
#![no_main]
#![feature(asm)]
#![feature(abi_efiapi)]
// extern crate rlibc;
use uefi::prelude::*;

#[entry]
fn efi_main(_img: uefi::Handle, st: SystemTable<Boot>) -> Status {
    // Initialize utilities (logging, memory allocation...)
    uefi_services::init(&st).expect_success("Failed to initialize utilities");

    // Reset the console
    st.stdout()
        .reset(false)
        .expect_success("Failed to reset output buffer");

    // Print out UEFI revision number
    {
        let rev = st.uefi_revision();
        let (major, minor) = (rev.major(), rev.minor());

        log::info!("UEFI {}.{}", major, minor);
    }

    // Print some information
    use core::fmt::Write;
    write!(st.stdout(), "\nCurrent Date: ").unwrap();
    print_date(&st);
    write!(st.stdout(), "\nCurrent Time: ").unwrap();
    print_time(&st);
    write!(st.stdout(), "\nMemory Map Size: ").unwrap();
    print_memory_map_size(&st);

    // Shutdown device after key press
    st.boot_services()
        .wait_for_event(&mut [st.stdin().wait_for_key_event()])
        .unwrap_success();

    let status: uefi::prelude::Status = uefi::prelude::Status::SUCCESS;
    use uefi::table::runtime::ResetType;
    st.runtime_services()
        .reset(ResetType::Shutdown, status, None);
}

fn current_time(st: &SystemTable<Boot>) -> uefi::table::runtime::Time {
    return st.runtime_services().get_time().unwrap_success();
}

fn print_time(st: &SystemTable<Boot>) {
    use core::fmt::Write;
    use uefi::table::runtime::Time;
    let ct: Time = current_time(&st);
    write!(st.stdout(), "{}/{}/{}", ct.month(), ct.day(), ct.year()).unwrap();
}

fn print_date(st: &SystemTable<Boot>) {
    use core::fmt::Write;
    use uefi::table::runtime::Time;
    let ct: Time = current_time(&st);
    write!(st.stdout(), "{}:{}:{}", ct.hour(), ct.minute(), ct.second()).unwrap();
}

fn print_memory_map_size(st: &SystemTable<Boot>) {
    use core::fmt::Write;
    write!(st.stdout(), "{}", st.boot_services().memory_map_size()).unwrap();
}
