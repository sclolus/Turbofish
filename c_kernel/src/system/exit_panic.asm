[BITS 32]
%define BASE_LOCATION 0x7C00
%define REBASE(x) BASE_LOCATION + x - exit_panic_begin_sub_sequence
%define REAL_MODE_STACK 0xF000

segment .text
GLOBAL exit_panic
exit_panic:
    push ebp
    mov ebp, esp

    ; XXX Ugly but work, send EOI to pic master and slave
    ; necessary for flushing PIC interrupt,
    ; A best way is to read IMR or ISR 8259 register to know what to do
    mov al, 0x20
    out 0x20, al
    mov al, 0xA0
    out 0xA0, al

    ; copy of content at BASE_LOCATION
    mov eax, exit_panic_end_sub_sequence
    sub eax, exit_panic_begin_sub_sequence
    mov ecx, eax
    mov esi, exit_panic_begin_sub_sequence
    mov edi, BASE_LOCATION
    rep movsb

; initialise temporary GDT
    mov eax, exit_panic_gdt_16_end
    sub eax, exit_panic_gdt_16
    mov word [REBASE(exit_panic_gdt_16_ptr)], ax

; store linear address of GDT
    mov eax, REBASE(exit_panic_gdt_16)
    mov dword [REBASE(exit_panic_gdt_16_ptr + 2)], eax

; go to jump location, no return is possible know
    jmp BASE_LOCATION

exit_panic_begin_sub_sequence:
    mov eax, cr0
    test eax, 0x80000000
    jz .skip_disable_paging

; disable paging (PG)
    mov eax, cr0
    and eax, 0x7fffffff
    mov cr0, eax

; fflush CR3 register
    xor eax, eax
    mov cr3, eax

.skip_disable_paging:

; load protected mode 16 GDT
    lgdt [REBASE(exit_panic_gdt_16_ptr)]

; jump to CS of 16 bits selector
    jmp 0x8:REBASE(.exit_panic_protected_16)
.exit_panic_protected_16:

; code is now in 16bits, because we are in 16 bits mode
;------------------------------------------------------
[BITS 16]
; set 16 bits protected mode data selector
    mov  ax, 0x10
    mov  ds, ax
    mov  es, ax
    mov  fs, ax
    mov  gs, ax
    mov  ss, ax

; load bios IDT
    lidt [REBASE(exit_panic_bios_idt)]

; disable protected bit
    mov eax, cr0
    and ax, 0xfffe
    mov cr0, eax

; configure CS in real mode
    jmp 0x0:REBASE(.exit_panic_real_16)
.exit_panic_real_16:

; configure DS, ES and SS in real mode
    xor ax, ax
    mov ds, ax
    mov es, ax
    mov ss, ax

; create a little real mode stack
    mov sp, REAL_MODE_STACK
    mov bp, sp

; PIC reinitialisation for BIOS 16bits real mode
; see bios vector table at http://www.bioscentral.com/misc/interrupts.htm

    jmp .exit_panic_icw_1
; |0|0|0|1|x|0|x|x|
;        |   | +--- with ICW4 (1) or without (0)
;        |   +----- one controller (1), or cascade (0)
;        +--------- triggering by level (level) (1) or by edge (edge) (0)
.exit_panic_icw_1: ; ICW1 (port 0x20 / port 0xA0)
    mov al, 0x11
    out 0x20, al  ; master
    out 0xA0, al  ; slave
    jmp .exit_panic_icw_2

; |x|x|x|x|x|0|0|0|
;  | | | | |
; +----------------- base address for interrupts vectors
.exit_panic_icw_2: ; ICW2 (port 0x21 / port 0xA1) Set vector offset. IRQ below 32 are processor reserved IRQ
    mov al, 0x08  ; 0x08 BIOS INTERRUPT VECTOR TABLE FOR MASTER
    out 0x21, al  ; master, begin at 32 (to 39)
    mov al, 0x70
    out 0xA1, al  ; slave, begin at 112 (to 119)
    jmp .exit_panic_icw_3

.exit_panic_icw_3: ; ICW3 (port 0x21 / port 0xA1) set how are connected pic master and slave
; |x|x|x|x|x|x|x|x|  for master
;  | | | | | | | |
;  +------------------ slave controller connected to the port yes (1), or no (0)
    mov al, 0x04  ; master is connector 3 of slave
    out 0x21, al

; |0|0|0|0|0|x|x|x|  for slave
;            | | |
;            +-+-+----- Slave ID which is equal to the master port
    mov al, 0x02  ; slave is connector 2 of master
    out 0xA1, al
    jmp .exit_panic_icw_4

; |0|0|0|x|x|x|x|1|
;        | | | +------ mode "automatic end of interrupt" AEOI (1)
;        | | +-------- mode buffered slave (0) or master (1)
;        | +---------- mode buffered (1)
;        +------------ mode "fully nested" (1)
.exit_panic_icw_4: ; ICW4 (port 0x21 / port 0xA1)
    mov al, 0x01
    out 0x21, al
    out 0xA1, al
    jmp .exit_panic_ocw_1

; |x|x|x|x|x|x|x|x|
;  | | | | | | | |
;  +-+-+-+-+-+-+-+---- for each IRQ : interrupt mask actif (1) or not (0)

.exit_panic_ocw_1:           ; Interrupt mask
    mov al, 0x00  ; All MASTER hardware interrupts are handled
    out 0x21, al  ; store IMR

    mov al, 0x00  ; All SLAVE hardware interrupts are handled
    out 0xA1, al  ; store IMR

    jmp .exit_panic_end_init_pic
.exit_panic_end_init_pic:

; reenable all interrupts
    sti

; wait CTRL + ALT + DEL keychain
; @DOC: http://www.ctyme.com/intr/int-16.htm
.exit_panic_key_loop_ctrl_alt_del:
    mov ah, 0x22
    mov al, 1
    mov ebx, 0x0708
    mov ecx, 0x0910
    mov edx, 0x1122
    int 16h
    cmp al, 0
    jne .exit_panic_key_loop_ctrl_alt_del

; sleep for 0.5 sec
    mov ax, 0x8600
    mov cx, 5
    mov dx, 0
    int 15h

; process a 8082 (keyboard controler) reset
.exit_panic_reset_computer_loop:
    in al, 0x64
    and al, 0x2
    cmp al, 0x2
    je .exit_panic_reset_computer_loop
    mov al, 0xFE
    out 0x64, al

; if reboot fail, dont do anything
    cli
.exit_panic_end_loop:
    hlt
    jmp .exit_panic_end_loop

exit_panic_bios_idt:
    dw 0x3ff ; limit
    dd 0     ; base

exit_panic_gdt_16:
    db 0, 0, 0, 0, 0, 0, 0, 0
.exit_panic_gdt_16b_cs:
    dw 0xFFFF, 0x0000
    db 0x00, 0x9A, 0x0, 0x0
.exit_panic_gdt_16b_ds:
    dw 0xFFFF, 0x0000
    db 0x00, 0x92, 0x0, 0x0
exit_panic_gdt_16_end:

exit_panic_gdt_16_ptr:
    dw 0  ; limit
    dd 0  ; base

exit_panic_end_sub_sequence:
