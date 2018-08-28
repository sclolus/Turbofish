[BITS 32]

segment .text

GLOBAL asm_print
GLOBAL asm_print_2

EXTERN print

asm_print:
	mov [toto], byte 'C'
	mov [toto + 1], byte 'D'
	mov [toto + 2], byte 'E'
	lea eax, [toto]
	push eax
	call print
	add esp, 4
	ret

asm_print_2:
	push ebp
	mov ebp, esp
	mov eax, [ebp + 8]
	push eax
	call print
	add esp, 4
	pop ebp
ret

segment .data

toto: db "--- debut de chaine <--> toto2121", 13, 10, 0
