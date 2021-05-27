# operating-system
### An attempt to create a custom operating system built on Rust

### Dependencies
  - rust (nightly)
  - cargo
  - cargo-make

### Running in qemu using the UEFI bootloader:
  1. Run `cd operating-system`
  2. Run `cargo make emulate`

### Running using a bootable USB drive (or any drive):
  1. `cd operating-system`
  2. `cargo make drive`
  3. Then copy the contents of the drive directory to a newly formatted FAT12/FAT16/FAT32 formatted drive

## Important information:
  - If an debian distribution throws the error that qemu-system-x86_64 could not be found, install the following packages: 
    - qemu-system
    - qemu-user
    - qemu-utils
  - While compiling, you might encounter an error to prompting to add rust-src as a component to rustc. Run the command that it gives you.
