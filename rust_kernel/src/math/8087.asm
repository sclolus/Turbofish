[BITS 32]

; 80x87 Instruction Set (x87 - Pentium)
; http://www2.math.uni-wuppertal.de/~fpf/Uebungen/GdR-SS02/opcode_f.html

segment .data

result: times 8 db 0
zero: times 8 db 0
exponent: dd 0

segment .text

; _trunc(v: f64) -> f64
global _trunc
_trunc:
    push ebp
    mov ebp, esp
    push ecx
    push edx

    finit

    fld qword [ebp + 8]    ; load double
    fst qword [result]     ; store result

    fxtract                ; extract signifiant and exponent
    fxch                   ; move exponent in ST0

    fistp dword [exponent] ; store exponent as int result and pop 8087 stack
    fdecstp                ; decremente the fpu stack (fpu registers will be empty now)

    mov eax, [exponent]    ; get exponent as int

    cmp eax, 0x80000000    ; check if NaN or INFINITY then return NaN or INFINITY
    je .get_result

    cmp eax, 0             ; (e < 0) negatif exponent. truncate result is 0
    jl .zero_result

    cmp eax, 52            ; (e >= 52) -> already an integer
    jge .get_result

; low section 0x________
    mov edx, 0xffffffff     ; initial positive mask

    xor ecx, ecx
    cmp eax, 20             ; (e <= 20) -> erase all low_section cl = 0
    jle .next_low_section

    mov ecx, eax            ; (20 < e < 52) -> (0 < ecx < 32)
    sub ecx, 20
.next_low_section:

    shr edx, cl             ; shift positive mask
    not edx                 ; create negative mask

    ; apply mask low
    and dword [result], edx

; high section 0xfff_____
    mov edx, 0xffffffff     ; initial positive mask
    cmp eax, 20             ; test if e > 20
    jl .next_high_section
    mov eax, 20             ; e = 20 where (e > 20)
.next_high_section:
    mov ecx, eax            ; set initial ecx to minimal exponent

    add ecx, 12             ; preserve sign (1b) + exponent (11b) -> 0xfff00000
    shr edx, cl             ; shift positive mask
    not edx                 ; create negative mask

    ; apply mask high
    and dword [result + 4], edx

.get_result:
    fld qword [result]      ; get the final result
    jmp .end

.zero_result:
    fld qword [zero]        ; get zero

.end:
    ; result as double is stored in ST0 register
    pop edx
    pop ecx
    pop ebp
    ret

; simple and basic trigonometry functions

global _cos
_cos:
    push ebp
    mov ebp, esp

    finit

    fld qword [ebp + 8]    		; load double

    fcos

    pop ebp
    ret

global _sin
_sin:
    push ebp
    mov ebp, esp

    finit

    fld qword [ebp + 8]    		; load double

    fsin

    pop ebp
    ret

global _tan
_tan:
    push ebp
    mov ebp, esp

    finit

    fld qword [ebp + 8]    		; load double

    fptan

    pop ebp
    ret
