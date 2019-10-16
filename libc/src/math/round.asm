[BITS 32]

; http://www2.math.uni-wuppertal.de/~fpf/Uebungen/GdR-SS02/opcode_f.html

section .text

; double round(double)
; The 8087 must be activated
GLOBAL round
round:
    push ebp
    mov ebp, esp

    ; load double
    fld qword [ebp + 8]
    ; Round to the nearest integer
    frndint

    pop ebp
ret
