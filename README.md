# os
## Dependencies
- rust
- cargo
- cargo-make

## To run in qemu using the UEFI bootloader
- `cd operating-system`
- `cargo make qemu`

## To run using a bootable USB drive (or any drive)
- `cd operating-system`
- `cargo make drive`
- Then copy the contents of the build directory to a newly formatted FAT12/FAT16/FAT32 formatted drive

## Common Issues
- If an debian distribution throws the error that qemu-system-x86_64 could not be found, install the following packages: 
  - qemu-system
  - qemu-user
  - qemu-utils
