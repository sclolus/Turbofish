
[BITS 32]

GLOBAL _asm_irq_default_base
GLOBAL _asm_irq_default_master
GLOBAL _asm_irq_default_slave

GLOBAL _asm_irq_clock


extern print
extern putchar
extern jump_line
extern backspace
extern show_cursor
extern hide_cursor


GLOBAL CURSOR_FLAG

CURSOR_FLAG: dw 0x0000
%DEFINE cursor_time 6



_asm_irq_default_base:
    mov al,0x20
    out 0x20,al
iret

_asm_irq_default_master:
    mov al,0x20
    out 0x20,al
iret

_asm_irq_default_slave:
    mov al,0x20
    out 0x20,al
    out 0xA0,al
iret



_asm_irq_clock:
    mov ax, 0x10
    mov ds, ax
    mov es, ax

; SEQUENCE POUR LE CLIGNOTEMENT DU CURSEUR DE TEXTE
    mov bx, [CURSOR_FLAG]
    test bh, bh
je inc_clock_cursor_test

dec_clock_cursor_test:
    cmp bl, cursor_time
jne dec_clock_cursor
    call hide_cursor
dec_clock_cursor:
    dec bl
    test bl, bl
jne reg_cursor_status
    xor bh, bh
jmp reg_cursor_status


inc_clock_cursor_test:
    test bl, bl
jne inc_clock_cursor
    call show_cursor
inc_clock_cursor:
    inc bl
    cmp bl, cursor_time
jne reg_cursor_status
    mov bh, 1


reg_cursor_status:
    mov [CURSOR_FLAG], bx
; -------------------------------------------------

    mov al,0x20
    out 0x20,al
iret
