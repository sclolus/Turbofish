[BITS 32]

section .text
        ; GRUB multiboot spec
        align 4
        dd 0x1BADB002                ; magic
        dd 0x0                       ; flags
        dd - (0x1BADB002 + 0x0)      ; checksum. m+f+c should be zero

extern kmain

global _start
_start:
    cli                             ; block interrupts

    push ebp
    mov ebp, esp

    ; set EIP of caller GRUB on stack at 0, prevent infinite loop for backtrace
    mov [ebp + 4], dword 0

    mov esp, stack_space            ; set stack pointer for a temporary stack

    ; EBX contain pointer to GRUB multiboot information (preserved register)
    push ebx

    mov al, 'A'
    mov edi, 0xb8000
    stosb

    call kmain					; kmain is called with this param
    add esp, 4

    jmp $

section .bss
resb 8192                           ; 8KB for temporary stack
stack_space:
