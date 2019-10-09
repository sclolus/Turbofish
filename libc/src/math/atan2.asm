[bits 32]

; http://www2.math.uni-wuppertal.de/~fpf/uebungen/gdr-ss02/opcode_f.html

section .text

; double atan2(double y, double x);
; the 8087 must be activated
global atan2
atan2:
    push ebp
    mov ebp, esp

    ; load floats
    fld qword [ebp + 8]
    fld qword [ebp + 16]
    ; partial arc tangent
    fpatan
    ffree

    pop ebp
ret
