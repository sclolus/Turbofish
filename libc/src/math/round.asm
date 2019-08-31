[BITS 32]

; http://www2.math.uni-wuppertal.de/~fpf/Uebungen/GdR-SS02/opcode_f.html

section .data

result dd 0

section .text

; float roundf(float)
GLOBAL roundf
roundf:
    push ebp
    mov ebp, esp

    ; initialise 8087
    finit

    ; load float
    fld dword [ebp + 8]
    ; Round to the nearest integer
    frndint

    ; When a function returns a float. keep it on ST1
    pop ebp
ret
