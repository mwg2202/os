#![no_std]
#![no_main]
#![feature(asm)]
#![feature(abi_efiapi)]
// extern crate rlibc;
use uefi::prelude::*;
use uefi::proto::console::gop::GraphicsOutput;
use core::fmt::Write;

#[entry]
fn efi_main(img: uefi::Handle, st: SystemTable<Boot>) -> Status {
    // Initialize utilities (logging, memory allocation...)
    uefi_services::init(&st).expect_success("Failed to initialize utilities");

    // Reset the console
    st.stdout()
        .reset(false)
        .expect_success("Failed to reset output buffer");

    // Print out UEFI revision number
    let bs = st.boot_services();
    let rs = st.runtime_services();

    let gout = bs.locate_protocol::<GraphicsOutput>()
        .expect_success("Failed to locate the GraphicsOutput protocall.");
    
    for mode in unsafe { &*gout.get() }.modes() {
        log::info!("mode: {:?}", mode.unwrap().info());
    }

    // Print some information
    log::info!("UEFI Version {:?}", st.uefi_revision());
    log::info!("Time: {:?}", rs.get_time().unwrap_success());
    log::info!("Memory Map Size: {}", bs.memory_map_size());

    // Shutdown device after key press
    st.boot_services()
        .wait_for_event(&mut [st.stdin().wait_for_key_event()])
        .unwrap_success();

    let status: Status = Status::SUCCESS;
    use uefi::table::runtime::ResetType;
    st.runtime_services()
        .reset(ResetType::Shutdown, status, None);
}
