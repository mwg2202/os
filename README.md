# os
## Dependencies
- NASM (`apt install nasm`)
- Make (`apt install make`)

## To run without UEFI support through QEMU
`make run`

## To run with UEFI through QEMU
`make compileRun`

## To run with UEFI through QEMU (Depricated)
`make runUEFI`

## To clean the build folder
 `make clean`

## FAQ
- If an debian distribution throws the error that qemu-system-x86_64 could not be found, install the following packages: 
  - qemu-system
  - qemu-user
  - qemu-utils
