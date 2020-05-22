%define DEBUG 1
[BITS 16]
[ORG 0x7C00]

boot:
    cld
    cli
    resetDrive:
        mov ah, 0
        int 13h
        or ah, ah
        jnz resetDrive
    xor ax, ax
    mov ss, ax
    mov ds, ax
    mov fs, ax
    mov gs, ax
    mov sp, 0x9C00
    
    mov ax, 0xB800
    mov es, ax

    ; Method 1
    mov ax, 0x2401
    int 0x15
    
    ; Set VGA Text Mode to 3
    mov ax, 0x3
    int 0x10 

    jmp mainCode

%include "src/include/definitions.asm"
%include "src/include/macros.asm"

mainCode:
    cli
    ; In real mode
    bios_print msg
    ; Enter Protected Mode
    lgdt [gdt_pointer]
    mov eax, cr0
    or eax, 0x1
    mov cr0, eax
    jmp 08h:boot2 ; Jumps to clear instruction pipeline of 16-bit instructions

gdt_start:
gdt_null:
    dq 0
gdt_code:
    ; 1st Double Word
    dw 0xFFFF ; First 16 bits in the segment limiter
    dw 0x0 ; First 16 bits in the base address
    db 0x0
    ; 2nd Double Word
    db 10011010b ; access flag, readable?, conforming?, code(1) or data segment(0), privileged(0-3), present flag
    db 11001111b ;Last 3 bits in segment limit, 'Available to system programmers' bit, always zero bit, size bit (set if 32-bit not 16-bit), granularity 
    db 0x0 ; remaining base address
gdt_data:
    dw 0xFFFF
    dw 0x0
    db 0x0
    db 10010010b
    db 11001111b
    db 0x0
gdt_end:
gdt_pointer:
    dw gdt_end - gdt_start - 1
    dd gdt_start

CODE_SEG equ gdt_code - gdt_start
DATA_SEG equ gdt_data - gdt_start

[BITS 32]

boot2:
    ; Fills segment registers
    mov ax, DATA_SEG
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    mov ss, ax
    add edi, 0xB8000

    ; In Protected Mode
    print msg2
    print_hex MagicNumber, 4
    print_hex 0x150, 8

    ; Check if A20 is enabled
    pushad
    mov edi, 0x112345
    mov esi, 0x012345
    mov [esi], esi
    mov [edi], edi
    cmpsd
    popad
    jne A20Enabled
    jmp halt
    A20Enabled:
        print msg3
    halt:
        hlt

msg: db 'Bootloader is Running!', 10, 13, 0
msg2: db 'Protected Mode Entered!', 0
msg3: db 'A20 is Enabled!', 0

times 510-($-$$) db 0   ; pad with zeros
MagicNumber dw 0xAA55   ; boot sector identifier
