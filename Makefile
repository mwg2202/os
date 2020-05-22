./build/main.bin diskimage.dd: src/bootloader.asm src/include/macros.asm src/include/definitions.asm
	mkdir -p build
	nasm -fbin ./src/bootloader.asm -o ./build/bootloader.bin
	dd if=/dev/zero of=./build/diskimage.dd bs=1M count=10
	parted -s ./build/diskimage.dd unit s mklabel gpt mkpart primary 34 100
	dd if=./build/bootloader.bin of=./build/diskimage.dd bs=512 count=1 conv=notrunc
	
clean:
	rm -r build

run:
	qemu-system-x86_64 -hda build/diskimage.dd
