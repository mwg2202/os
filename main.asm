[BITS 16]
[ORG 0x7c00]

section .text
    global entryPoint
entryPoint:
    cli
    xor ax, ax
    mov ss, ax
    mov ds, ax
    mov fs, ax
    mov gs, ax
    mov sp, 0x9C00
    mov ax, 0xB800 ; 0xB800 : start of text screen video memory (alt: 0xB000)
    mov es, ax
    
    resetDrive:
        mov ah, 0
        int 13h
        or ah, ah
        jnz resetDrive

    jmp mainCode

%include "definitions.asm"
%include "libraries/flightos.asmlib"

mainCode:
    ; In Real Mode
    mov ah, 0x06
    mov bh, 0x3F
    mov cx, 0
    mov DX, 0x4F
    int 0x10
    print msg
    print msg2, bios_yellow

    ; Enter Protected Mode
    lgdt [gdt_desc]
    mov eax, cr0
    or eax, 1
    mov cr0, eax
    jmp 08h:clear_pipe ; Jumps to clear instruction pipeline of 16-bit instructions

[BITS 32]
clear_pipe:
    ; Fills segment registers
    mov ax, 10h
    mov ds, ax
    mov ss, ax
    mov esp, 090000h
    ; In Protected Mode
hang:
    jmp hang
    print msg3



msg db 'Bootloader is Running!', 0
msg2 db 'Entering Protected Mode...', 0
msg3 db 'Protected Mode Entered!', 0

gdt:
gdt_null: 
    dq 0
gdt_code:
    ; 1st Double Word
    dw 0FFFFh ; First 16 bits in the segment limiter
    dw 0 ; First 16 bits in the base address
    db 0
    ; 2nd Double Word
    db 10011010b ; access flag, readable?, conforming?, code(1) or data segment(0), privileged(0-3), present flag
    db 11001111b ;Last 3 bits in segment limit, 'Available to system programmers' bit, always zero bit, size bit (set if 32-bit not 16-bit), granularity 
    db 0 ; remaining base address
gdt_data:
    dw 0FFFFh
    dw 0
    db 0
    db 10010010b
    db 11001111b
    db 0
gdt_end:

gdt_desc:
    dw gdt_end - gdt - 1
    dd gdt


times 510-($-$$) db 0   ; pad with zeros
dw 0xAA55               ; boot sector identifier