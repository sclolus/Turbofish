[bits 32]

; http://www2.math.uni-wuppertal.de/~fpf/uebungen/gdr-ss02/opcode_f.html

section .text

; float atanf(float x);
; the 8087 must be activated
global atanf
atanf:
    push ebp
    mov ebp, esp

    ; load float
    fld dword [ebp + 8]
    fld1
    ; arc tangent
    fpatan
    ffree

    pop ebp
ret
