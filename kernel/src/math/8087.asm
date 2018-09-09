
; http://www2.math.uni-wuppertal.de/~fpf/Uebungen/GdR-SS02/opcode_f.html

segment .data

result dd 0

segment .text
[BITS 32]
; int round(float)
GLOBAL round
round:
    push ebp
    mov ebp, esp

    finit                  ; initialise 8087

    fld dword [ebp + 8]    ; load float
    frndint                ; round to int
    fistp dword [result]   ; store int value and pop 8087 stack
    mov eax, [result]      ; assign to ret value

    pop ebp
ret
