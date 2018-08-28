
[BITS 32]

GLOBAL in
GLOBAL in16
GLOBAL out

out:                        ; void out(u16 port,u32 value);
    push ebp
    mov ebp, esp

    mov edx, [ebp+8]
    mov eax, [ebp+12]

    out dx, eax

    mov esp, ebp
    pop ebp
ret

in:                         ; u32 in(u16 port)
    push ebp
    mov ebp, esp

    mov edx, [ebp+8]

    in eax, dx

    mov esp, ebp
    pop ebp
ret

in16:                         ; u32 in(u16 port)
    push ebp
    mov ebp, esp

    mov edx, [ebp+8]

    in ax, dx

    mov esp, ebp
    pop ebp
ret
