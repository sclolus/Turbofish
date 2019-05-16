[BITS 32]

segment .text
align 16

global _dummy_asm_process_code
global _dummy_asm_process_len

%macro STO 1
	mov ax, %1
	stosb
%endmacro

_dummy_asm_process_code:
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

	mov eax, 4
	mov ebx, 1
	mov ecx, 0x400100
	mov edx, 29

.loop:
	int 80h
	jmp .loop

.ud2
	ud2

	jmp $

_dummy_asm_process_len:    dd $-_dummy_asm_process_code
