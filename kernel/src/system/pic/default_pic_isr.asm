[BITS 32]
segment .text
GLOBAL asm_default_interrupt
GLOBAL asm_default_pic_master_interrupt
GLOBAL asm_default_pic_slave_interrupt

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
