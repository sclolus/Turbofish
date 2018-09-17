[BITS 32]
section .text
        ; GRUB multiboot spec
        align 4
        dd 0x1BADB002                ; magic
        dd 0x0                       ; flags
        dd - (0x1BADB002 + 0x0)      ; checksum. m+f+c should be zero

extern kmain
extern init_gdt
extern g_multiboot_info
%define MULTIBOOT_INFO_LENGTH 116

global _start
_start:
    cli                             ; block interrupts

    push ebp
    mov ebp, esp

    ; set EIP of caller GRUB on stack at 0, prevent infinite loop for backtrace
    mov [ebp + 4], dword 0

    mov esp, stack_space            ; set stack pointer for a temporary stack

    push 0x0
    call init_gdt

    mov ax, 0x20                    ; create the main kernel stack
    mov ss, ax
    mov esp, 0x300000

    ; EBX contain pointer to GRUB multiboot information (preserved register)
    push ebx
    call kmain                      ; kmain is called with this param
    add esp, 4

.halt:
    hlt                             ; halt the CPU until next interrupt
    jmp .halt

section .bss
resb 8192                           ; 8KB for temporary stack
stack_space:
