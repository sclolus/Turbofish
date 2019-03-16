
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
