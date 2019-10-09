[bits 32]

; http://www2.math.uni-wuppertal.de/~fpf/uebungen/gdr-ss02/opcode_f.html

section .text

; double tan(double x);
; the 8087 must be activated
global tan
tan:
    push ebp
    mov ebp, esp

    ; load double
    fld qword [ebp + 8]
    ; tangente
    fptan
    fxch
    ffree

    pop ebp
ret
