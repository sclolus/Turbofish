[BITS 32]

segment .text
global user_stack_overflow
user_stack_overflow:
	push ebp
	mov ebp, esp

	mov eax, 0x80000001
	int 80h

	pop ebp
	ret
