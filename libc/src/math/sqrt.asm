[BITS 32]

; http://www2.math.uni-wuppertal.de/~fpf/Uebungen/GdR-SS02/opcode_f.html

section .text

; double sqrt(double x);
; The 8087 must be activated
GLOBAL sqrt
sqrt:
    push ebp
    mov ebp, esp

    ; load double
    fld qword [ebp + 8]
    ; square root
    fsqrt

    pop ebp
ret
