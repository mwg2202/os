%macro print 1
    push edi
    mov esi, %1
    mov ah, 0x0F
    call LIB_PRINT
    pop edi
    add edi, 160
%endmacro

%macro print 2
    push edi
    mov esi, %1
    mov ah, 0x00
    add ah, %2
    call LIB_PRINT
    pop edi
    add edi, 160
%endmacro

%macro print 3
    push edi
    mov esi, %1
    mov ah, %2
    shl ah, 4
    add ah, %3
    call LIB_PRINT
    pop edi
    add edi, 160
%endmacro

; BIOS_PRINT
%macro bios_print 1
    push edi
    mov esi, %1
    mov ah, 0x0E
    call LIB_BIOS_PRINT
    pop edi
    add edi, 160
    
%endmacro

; First argument is the address, second is number of nibbles to print
%macro print_hex 2
    push edi
    mov esi, %1
    mov cl, %2
    ;mov edi, LIB_HEX
    ;add edi, 2
    call LIB_PRINT_HEX
    pop edi
    print LIB_HEX
%endmacro

LIB_RETURN:
    ret

; To call es must hold the location of the start of text screen video memory
; Atribute stored in ah
LIB_BIOS_PRINT:
    lodsb
    cmp al, 0x0
    je LIB_RETURN
    int 0x10
    jmp LIB_BIOS_PRINT

[BITS 32]

LIB_PRINT:
    lodsb
    cmp al, 0
    je LIB_PRINT_RETURN
    stosw
    add edi, -2
    jmp LIB_PRINT
    LIB_PRINT_RETURN: 
        ret

LIB_HEX_TABLE db "0123456789ABCDEF"
LIB_HEX dw "0x**********", 10, 13, 0 

LIB_PRINT_HEX:

    ; Load a byte
    lodsb
    ; Seperate nibbles
    xor ebx, ebx
    xor edx, edx
    mov bl, al
    mov dl, al
    ; Store nibble 1
    shr bl, 4
    mov al, [ebx + LIB_HEX_TABLE]
    stosb
    ; Store nibble 2
    and dl, 00001111b
    mov al, [edx + LIB_HEX_TABLE]
    stosb
    
    ; Check all nibbles were printed
    dec cl
    jne LIB_PRINT_HEX
    mov al, 0
    stosb
    ret