[BITS 32]

extern process_keyboard

; 8042 chipset
; 60h read or transmit data
; 64h compute status or emmit command

GLOBAL asm_keyboard_isr
asm_keyboard_isr:
    pushad

    in al, 0x64
    mov edx, eax
    and edx, 0x1
    cmp edx, 0
    je asm_keyboard_isr ; wait after kerboard buffer is full

    xor eax, eax
    in al, 0x60 ; read scan_code

    push eax
    call process_keyboard
    add esp, 4

    mov al, 0x20
    out 0x20, al

    popad
    iret
