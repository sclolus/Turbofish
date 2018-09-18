[BITS 32]
segment .text
GLOBAL exit_panic
exit_panic:
    push ebp
    mov esp, ebp


    pop ebp
    ret
