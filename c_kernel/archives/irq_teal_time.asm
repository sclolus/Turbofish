
GLOBAL asm_real_time_clock_handler

asm_real_time_clock_handler:
    mov al, 0x0C
    out 0x70, al ; select register C

    in al, 0x71 ; read register c
; IRQ8 is managed by master and slave, so we must inform the two PICS

    mov al, 0x20
    out 0x20, al
    mov al, 0xA0
    out 0xA0, al
    iret

    // XXX The real time clock handler has trouble here !

associated like that in idt.c:
initialize_idt_seg(112,(u32)&asm_real_time_clock_handler, 0x8, INTGATE);
