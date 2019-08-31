[BITS 32]

segment .text
align 16

global _dummy_asm_process_code_a
global _dummy_asm_process_len_a

%macro STO 1
	mov ax, %1
	stosb
%endmacro

_dummy_asm_process_code_a:
	push ebp
	mov ebp, esp

	mov edi, 0x400100
	STO 'H'
	STO 'e'
	STO 'l'
	STO 'l'
	STO 'o'
	STO ' '
	STO 'w'
	STO 'o'
	STO 'r'
	STO 'l'
	STO 'd'
	STO ' '
	STO 'f'
	STO 'r'
	STO 'o'
	STO 'm'
	STO ' '
	STO 'u'
	STO 's'
	STO 'e'
	STO 'r'
	STO 's'
	STO 'p'
	STO 'a'
	STO 'c'
	STO 'e'
	STO ' '
	STO '!'
	STO 10

	mov ebx, 1
	mov ecx, 0x400100
	mov edx, 29

.loop:
	mov eax, 4
	int 80h

	jmp .loop

.ud2:
	ud2

	jmp $

_dummy_asm_process_len_a:    dd $-_dummy_asm_process_code_a

global _dummy_asm_process_code_b
global _dummy_asm_process_len_b

_dummy_asm_process_code_b:
	push ebp
	mov ebp, esp

	mov edi, 0x400100
	STO 'L'
	STO 'e'
	STO 's'
	STO ' '
	STO 'c'
	STO 'a'
	STO 'r'
	STO 'o'
	STO 't'
	STO 't'
	STO 'e'
	STO 's'
	STO ' '
	STO 's'
	STO 'o'
	STO 'n'
	STO 't'
	STO ' '
	STO 'c'
	STO 'u'
	STO 'i'
	STO 't'
	STO 'e'
	STO 's'
	STO ' '
	STO '!'
	STO 10

	mov ebx, 1
	mov ecx, 0x400100
	mov edx, 27

.loop:
	mov eax, 4
	int 80h

	jmp .loop

.ud2:
	ud2

	jmp $

_dummy_asm_process_len_b:    dd $-_dummy_asm_process_code_b
