[BITS 32]

%define a               QWORD [ebp+8]
%define b               QWORD [ebp+16]
%define result          DWORD [ebp+24]
%define ctrlWord        WORD [ebp-2]
%define tmp             DWORD [ebp-6]

segment .text
global __turbofish_pow

__turbofish_pow:
    push ebp
    mov ebp, esp
    sub esp, 6
    push ebx

    fstcw ctrlWord
    or ctrlWord, 110000000000b
    fldcw ctrlWord

    fld b
    fld a
    fyl2x

    fist tmp

    fild tmp
    fsub
    f2xm1
    fld1
    fadd
    fild tmp
    fxch
    fscale

    mov ebx, result
    fst QWORD [ebx]

    pop ebx
    mov esp, ebp
    pop ebp
    ret
