[BITS 32]

segment .text

global user_wait
user_wait:
	push ebp
	mov ebp, esp

	push ebx

	mov ebx, [ebp + 8]

	mov eax, 114 ; system call number (sys_munmap)
	int 80h

	pop ebx

	pop ebp
	ret
