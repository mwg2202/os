#![no_std]
#![no_main]
#![feature(asm)]
#![feature(abi_efiapi)]

extern crate rlibc;
use uefi::prelude::*;

#[entry]
fn efi_main(ih: uefi::Handle, st: SystemTable<Boot>) -> Status {
	uefi_services::init(&st).expect_success("Failed to initialize utils");
	
	// reset console before doing anything else
	st.stdout().reset(false).expect_success("Failed to reset output buffer");
	
	// Print out UEFI revision number
	{
    	let rev = st.uefi_revision();
    	let (major, minor) = (rev.major(), rev.minor());
	
    	log::info!("UEFI {}.{}", major, minor);
	}
    
	loop {}
}
