[BITS 32]
segment .text
GLOBAL asm_default_interrupt
GLOBAL asm_default_pic_master_interrupt
GLOBAL asm_default_pic_slave_interrupt
GLOBAL asm_keyboard_handler

asm_default_interrupt:
    iret

asm_default_pic_master_interrupt:
    mov al, 0x20
    out 0x20, al
    iret

asm_default_pic_slave_interrupt:
; IRQ8 is managed by master and slave, so we must inform the two PICS
    mov al, 0x20
    out 0x20, al
    mov al, 0xA0
    out 0xA0, al
    iret

extern process_keyboard

; 8042 chipset
; 60h read or transmit data
; 64h compute status or emmit command
asm_keyboard_handler:
    pushad

    in al, 0x64
    mov edx, eax
    and edx, 0x1
    cmp edx, 0
    je asm_keyboard_handler ; wait after kerboard buffer is full

    xor eax, eax
    in al, 0x60 ; read scan_code

    push eax
    call process_keyboard
    add esp, 4

    mov al, 0x20
    out 0x20, al

    popad
    iret
