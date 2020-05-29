disk	=	./build/diskimage.dd
parted	=	parted -s $(disk) unit s
CC		=	x86_64-w64-mingw32-gcc
LD		=	ld
qemu	=	qemu-system-x86_64

arch	=	$(shell uname -m | sed s,i[3456789]86,ia32,)
efiInc	=	build/gnu-efi/include/efi
efiIncs =	-I$(efiInc) -I$(efiInc)/x86_64 -I$(efiInc)/protocol
efiLib	=	build/gnu-efi/lib
efiCrt  =	$(efiLib)/crt0-efi-$(arch).o
efiLds	= 	$(efiLib)/elf_$(arch)_efi.lds
CFLAGS	=	$(efiIncs) -fno-stack-protector -fpic \
			-fshort-wchar -mno-red-zone -Wall -c
LDFLAGS	=	-nostdlib -T $(efiLds) -shared \
			-Bsymbolic -L $(efiLib) $(efiCrt)

ifeq ($(arch),x86_64)
	CFLAGS += -DEFI_FUNCTION_WRAPPER
endif

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

buildGnuEFI:clean
	mkdir -p build
	#cp edk2/Build/OvmfX64/DEBUG_GCC5/FV/OVMF.fd build/OVMF.fd
	cd gnu-efi && make install
	$(CC) $(CFLAGS) -o build/kernel.o src/kernel.c
	$(LD) $(LDFLAGS) -o build/BOOTX64.EFI build/kernel.o -lefi -lgnuefi
	dd if=/dev/zero of=build/fat.img bs=1k count=1440
	mformat -i build/fat.img -f 1440 ::
	mmd -i build/fat.img ::/EFI
	mmd -i build/fat.img ::/EFI/BOOT
	mcopy -i build/fat.img build/BOOTX64.EFI ::/EFI/BOOT

runGnuEFI:buildGnuEFI
	$(qemu) -pflash OVMF.fd -usb build/fat.img -net none

buildEDK2:clean
	mkdir -p build
	#cp edk2/OvmfPkg/Include build/Include
	dd if=/dev/zero of=build/fat.img bs=1k count=1440
	mformat -i build/fat.img -f 1440 ::
	mmd -i build/fat.img ::/EFI
	mmd -i build/fat.img ::/EFI/BOOT
	mcopy -i build/fat.img src/bootx64.efi ::/EFI/BOOT
	
runEDK2:buildEDK2
	$(qemu) -pflash OVMF.fd -usb build/fat.img -net none
