.PHONY: clean run

disk		=	./build/diskimage.dd
parted		=	parted -s $(disk) unit s
targetArch  = 	x86_64
outputFile	=	target/x86_64-unknown-uefi/debug/os.efi
qemu 		= 	qemu-system-$(targetArch)


# Flags from https://gil0mendes.io/blog/an-efi-app-a-bit-rusty/
qemuFlags 	=	-nodefaults -vga std -machine q35 -m 128M \
	-drive if=pflash,format=raw,readonly,file=OVMF_CODE.fd\
	-drive if=pflash,format=raw,file=OVMF_VARS-1024x768.fd\
	-drive format=raw,file=fat:rw:build\
	-serial stdio -monitor vc:1024x768

clean:
	rm -rf build

build/EFI/BOOT/BootX64.efi:src/main.rs
	mkdir -p build/EFI/BOOT
	cargo build -p uefi-bootloader --target x86_64-unknown-uefi -Z build-std=core,compiler_builtins,alloc
	cp startup.nsh build/
	cp $(outputFile) build/EFI/BOOT/BOOTX64.efi

run:build/EFI/BOOT/BootX64.efi
	$(qemu) $(qemuFlags)

