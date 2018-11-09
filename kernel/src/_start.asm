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
    mov esp, 0x700000

    call set_sse2

    ; EBX contain pointer to GRUB multiboot information (preserved register)
    push ebx
    call kmain                      ; kmain is called with this param
    add esp, 4

    jmp $

set_sse2:
    push ebp
    mov ebp, esp
    pushad

    mov eax, 0x1
    cpuid
    test edx, 1 << 26   ; test if SSE2 feature exist
    jz .end_set_sse2

    mov eax, cr0
    and ax, 0xFFFB		; clear coprocessor emulation CR0.EM
    or ax, 0x2			; set coprocessor monitoring  CR0.MP
    mov cr0, eax
    mov eax, cr4
    or ax, 3 << 9		; set CR4.OSFXSR and CR4.OSXMMEXCPT at the same time
    mov cr4, eax

.end_set_sse2:
    popad
    pop ebp
    ret

section .bss
resb 8192                           ; 8KB for temporary stack
stack_space:
