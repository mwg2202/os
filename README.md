# os
## Dependencies
- make (`apt install make`)
- rust

## Common Issues
- If you are booting from a USB drive make sure that the drive is FAT12/FAT16/FAT32 formatted
- If an debian distribution throws the error that qemu-system-x86_64 could not be found, install the following packages: 
  - qemu-system
  - qemu-user
  - qemu-utils
