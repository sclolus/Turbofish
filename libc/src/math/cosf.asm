[BITS 32]

; http://www2.math.uni-wuppertal.de/~fpf/Uebungen/GdR-SS02/opcode_f.html

section .text

; float cosf(float x)	
; The 8087 must be activated
GLOBAL cosf
cosf:
    push ebp
    mov ebp, esp

    ; load float
    fld dword [ebp + 8]
    ; cosinus
    fcos

    ; When a function returns a float. keep it on ST1
    pop ebp
ret
