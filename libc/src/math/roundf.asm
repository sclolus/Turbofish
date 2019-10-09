[BITS 32]

; http://www2.math.uni-wuppertal.de/~fpf/Uebungen/GdR-SS02/opcode_f.html

section .text

; float roundf(float)
; The 8087 must be activated
GLOBAL roundf
roundf:
    push ebp
    mov ebp, esp

    ; load float
    fld dword [ebp + 8]
    ; Round to the nearest integer
    frndint

    ; When a function returns a float. keep it on ST1
    pop ebp
ret
