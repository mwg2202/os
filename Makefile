main.bin:main.asm ./libraries/flightos.asmlib definitions.asm
	nasm -fbin main.asm -o main.bin

clean:
	rm main.bin

run:
	qemu-system-x86_64 main.bin
