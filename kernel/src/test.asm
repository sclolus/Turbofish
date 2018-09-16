
section .data

msg: db "A critical test is hapened", 0

section .text
extern panic
GLOBAL test_fn
test_fn:
    push ebp
    mov ebp, esp

    mov eax, 0xFF;
    mov edx, 0x89ABCDEF;

    pushf
    push ss
    push es
    push ds
    pushad
    push msg

    call panic
