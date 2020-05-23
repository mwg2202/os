# os
## Dependencies
- NASM (`apt install nasm`)
- CMake (`apt install cmake`)

## To run without UEFI support through QEMU
`make && make run`

## To run with UEFI through QEMU
`make UEFI && make runUEFI`

## FAQ
- If an debian distribution throws the error that qemu-system-x86_64 could not be found, install the following packages: 
  - qemu-system
  - qemu-user
  - qemu-utils