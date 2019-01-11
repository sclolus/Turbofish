
%define PAGING_BIT (1 << 31)

extern putnbr_base

[BITS 32]
segment .text
GLOBAL asm_paging_set_page_directory_address
asm_paging_set_page_directory_address:
    push ebp
    mov ebp, esp

    mov eax, [ebp + 8]
    mov cr3, eax

    pop ebp
ret

GLOBAL asm_paging_enable
asm_paging_enable:
    push ebp
    mov ebp, esp

    mov eax, cr0
    or eax, PAGING_BIT
    mov cr0, eax

    pop ebp
ret

GLOBAL asm_paging_disable
asm_paging_disable:
    push ebp
    mov ebp, esp

    mov eax, cr0
    xor eax, PAGING_BIT
    mov cr0, eax

    pop ebp
ret
