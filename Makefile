disk=./build/diskimage.dd
parted=parted -s $(disk) unit s
CC=x86_64-w64-mingw32-gcc
CFLAGS=-Ipath/to/gnu-efi/inc -Ipath/to/gnu-efi/inc/x86_64 -Ipath/to/gnu-efi/inc/protocol
qemu=qemu-system-x86_64 -hda $(disk)
LDFLAGS=--nostdlib -Wl,-dll -shared -Wl, --subsystem,10 -e efi_main

build:
	mkdir -p build
	nasm -fbin ./src/bootloader.asm -o ./build/bootloader.bin
	dd if=/dev/zero of=$(disk) bs=1M count=10
	$(parted) mklabel gpt 
	$(parted) mkpart primary 34 100
	dd if=./build/bootloader.bin of=$(disk) bs=512 count=1 conv=notrunc

clean:build
	rm -r build

run:$(disk)
	$(qemu)

buildUEFI:
	mkdir -p build
	nasm -fbin ./src/bootloader.asm -o ./build/bootloader.bin
	dd if=/dev/zero of=$(disk) bs=1M count=10
	$(parted) mklabel gpt 
	$(parted) mkpart primary 100 300
	$(parted) set 1 esp on


runUEFI:$(disk) OVMF-pure-efi.fd
	$(qemu) -bios OVMF-pure-efi.fd -net none

compile:
	# compile: (flags before -o become CFLAGS in your Makefile)
	$(CC) $(CFLAGS) -o hello.o hello.c
	$(CC) $(CFLAGS) -o data.o path/to/gnu-efi/lib/data.c
	# link: (flags before -o become LDFLAGS in your Makefile)
	$(CC) $(LDFLAGS) -o BOOTX64.EFI hello.o data.o -lgcc
