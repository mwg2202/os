./build/main.bin diskimage.dd:src/main.asm src/include/bootloader.asm src/include/definitions.asm
	mkdir -p build
	nasm -fbin ./src/main.asm -o ./build/main.bin
	dd if=/dev/zero of=./build/diskimage.dd bs=1M count=10
	parted -s ./build/diskimage.dd unit s mklabel gpt mkpart primary 34 100
	dd if=./build/main.bin of=./build/diskimage.dd bs=512 count=1 conv=notrunc
	
clean:
	rm -r build

run:
	qemu-system-x86_64 -hda build/diskimage.dd
