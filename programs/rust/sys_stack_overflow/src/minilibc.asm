[BITS 32]

; Calling convention: Platform System V i386
; ------------------------------------------
; Return Value | Parameter Registers | Additional Parameters | Stack Alignment
; eax, edx       none                  stack (right to left)   none
;
; Scratch Registers | Preserved Registers       | Call List
; eax, ecx, edx       ebx, esi, edi, ebp, esp     ebp

segment .text

global user_write
user_write:
	push ebp
	mov ebp, esp

	push ebx

	mov edx, [ebp + 16]
	mov ecx, [ebp + 12]
	mov ebx, [ebp + 8]

	mov eax, 4 ; system call number (sys_write)
	int 80h

	pop ebx

	pop ebp
	ret

global user_exit
user_exit:
	push ebp
	mov ebp, esp

	mov ebx, [ebp + 8]

	mov eax, 1 ; system call number (sys_exit)
	int 80h

global user_fork
user_fork:
	push ebp
	mov ebp, esp

	mov eax, 0x2
	int 0x80

	pop ebp
	ret
