[BITS 32]

segment .text
align 16

global _idle_process_code
global _idle_process_len

; When it is launched, The idle process takes a function to call between each `hlt` instruction
_idle_process_code:
	mov ebx, eax
	hlt
	call ebx
	jmp _idle_process_code

_idle_process_len:    dd $-_idle_process_code
