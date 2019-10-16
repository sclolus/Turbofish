[BITS 32]

; http://www2.math.uni-wuppertal.de/~fpf/Uebungen/GdR-SS02/opcode_f.html

section .text

; float sqrt(float x);
; The 8087 must be activated
GLOBAL sqrtf
sqrtf:
    push ebp
    mov ebp, esp

    ; load float
    fld dword [ebp + 8]
    ; square root
    fsqrt

    pop ebp
ret
