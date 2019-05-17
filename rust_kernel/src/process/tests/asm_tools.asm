[BITS 32]

segment .data

global _get_stack
_get_stack:
	mov eax, esp
	ret
