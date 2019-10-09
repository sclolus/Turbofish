[bits 32]

; http://www2.math.uni-wuppertal.de/~fpf/uebungen/gdr-ss02/opcode_f.html

section .text

; double atan(double x);
; the 8087 must be activated
global atan
atan:
    push ebp
    mov ebp, esp

    ; load doubles
    fld qword [ebp + 8]
    fld1
    ; arc tangent
    fpatan
    ffree

    pop ebp
ret
