[BITS 32]
segment .data

segment .text
GLOBAL asm_default_interrupt
GLOBAL asm_page_fault
GLOBAL asm_default_pic_master_interrupt
GLOBAL asm_default_pic_slave_interrupt
GLOBAL asm_clock_handler
GLOBAL asm_keyboard_handler
GLOBAL asm_real_time_clock_handler

asm_default_interrupt:
    iret

; Normaly, CS, EFLAG, EIP and other think like that are pushed/poped by the cpu
; page fault POP on the stack the error code [ebp + 4], to execute IRET corectly
; we must add esp by 4 or pop something.
extern page_fault_handler
asm_page_fault:
    push ebp
    mov ebp, esp
    pushad
    push ds
    push es

    mov ebx, cr2
    push ebx
    mov eax, [ebp + 4]
    push eax
    call page_fault_handler
    add esp, 8

    pop es
    pop ds
    popad
    pop ebp

    ; bypass the error code
    add esp, 4
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

extern putstr
asm_clock_handler:
	push eax
    mov al, 0x20
    out 0x20, al
	pop eax
    iret

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

extern process_keyboard

; 8042 chipset
; 60h read or transmit data
; 64h compute status or emmit command
asm_keyboard_handler:
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
    iret
