[bits 32]

; http://www2.math.uni-wuppertal.de/~fpf/uebungen/gdr-ss02/opcode_f.html

section .text

; float tanf(float x);
; the 8087 must be activated
global tanf
tanf:
    push ebp
    mov ebp, esp

    ; load float
    fld dword [ebp + 8]
    ; tangente
    fptan
    fxch
    ffree

    pop ebp
ret
