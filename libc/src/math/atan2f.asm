[bits 32]

; http://www2.math.uni-wuppertal.de/~fpf/uebungen/gdr-ss02/opcode_f.html

section .text

; float atan2f(float y, float x);
; the 8087 must be activated
global atan2f
atan2f:
    push ebp
    mov ebp, esp

    ; load floats
    fld dword [ebp + 8]
    fld dword [ebp + 12]
    ; partial arc tangent
    fpatan
    ffree

    pop ebp
ret
