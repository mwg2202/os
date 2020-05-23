disk=./build/diskimage.dd
parted=parted -s $(disk) unit s
CC=x86_64-w64-mingw32-gcc
CFLAGS=-ffreestanding -Ignu-efi/inc -Ignu-efi/inc/x86_64 -Ignu-efi/inc/protocol -Ignu-efi/lib -c
qemu=qemu-system-x86_64
LDFLAGS=-nostdlib -Wl,-dll -shared -Wl,--subsystem,10 -e efi_main

build:clean
	mkdir -p build
	nasm -fbin ./src/bootloader.asm -o ./build/bootloader.bin
	dd if=/dev/zero of=$(disk) bs=1M count=10
	$(parted) mklabel gpt 
	$(parted) mkpart primary 34 100
	dd if=./build/bootloader.bin of=$(disk) bs=512 count=1 conv=notrunc

clean:
	if test -d build; then rm -r build; fi

run:build
	$(qemu) -hda $(disk)

buildUEFI:clean
	mkdir -p build
	nasm -fbin ./src/bootloader.asm -o ./build/bootloader.bin
	dd if=/dev/zero of=$(disk) bs=1M count=10
	$(parted) mklabel gpt 
	$(parted) mkpart primary 100 300
	$(parted) set 1 esp on


runUEFI:buildUEFI
	$(qemu) -hda $(disk) -bios OVMF.fd -net none

compile:clean
	mkdir -p build
	# compile: (flags before -o become CFLAGS in your Makefile)
	$(CC) $(CFLAGS) -o build/kernel.o src/kernel.c
	$(CC) $(CFLAGS) -o build/data.o src/data.c
	# link: (flags before -o become LDFLAGS in your Makefile)
	$(CC) $(LDFLAGS) -o build/BOOTX64.EFI build/kernel.o build/data.o -lgcc
	dd if=/dev/zero of=build/fat.img bs=1k count=1440
	mformat -i build/fat.img -f 1440 ::
	mmd -i build/fat.img ::/EFI
	mmd -i build/fat.img ::/EFI/BOOT
	mcopy -i build/fat.img build/BOOTX64.EFI ::/EFI/BOOT

compileRun:compile
	$(qemu) -bios OVMF.fd -usb build/fat.img -net none
