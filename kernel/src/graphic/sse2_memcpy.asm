
[BITS 32]
segment .text

GLOBAL sse2_memcpy
sse2_memcpy:
    push ebp
    mov ebp, esp
    push esi
    push edi
    push ebx

    mov edi, [ebp + 8]      ; dest
    mov esi, [ebp + 12]     ; src
    mov ebx, [ebp + 16]     ; count
    shr ebx, 7              ; divide by 128 (8 * 128bit registers)

.memcpy_loop_copy:
    prefetchnta [ESI + 128] ; SSE2 prefetch
    prefetchnta [ESI + 160]
    prefetchnta [ESI + 192]
    prefetchnta [ESI + 224]

    movdqa xmm0, [ESI]      ; move data from src to registers
    movdqa xmm1, [ESI + 16]
    movdqa xmm2, [ESI + 32]
    movdqa xmm3, [ESI + 48]
    movdqa xmm4, [ESI + 64]
    movdqa xmm5, [ESI + 80]
    movdqa xmm6, [ESI + 96]
    movdqa xmm7, [ESI + 112]

    movntdq [EDI], xmm0      ; move data from registers to dest
    movntdq [EDI + 16], xmm1
    movntdq [EDI + 32], xmm2
    movntdq [EDI + 48], xmm3
    movntdq [EDI + 64], xmm4
    movntdq [EDI + 80], xmm5
    movntdq [EDI + 96], xmm6
    movntdq [EDI + 112], xmm7

    add esi, 128
    add edi, 128
    dec ebx

    jnz .memcpy_loop_copy            ; loop please

    pop ebx
    pop edi
    pop esi
    pop ebp
    ret
