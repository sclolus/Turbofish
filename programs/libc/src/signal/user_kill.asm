[bits 32]

; calling convention: platform system v i386
; ------------------------------------------
; return value | parameter registers | additional parameters | stack alignment
; eax, edx       none                  stack (right to left)   none
;
; scratch registers | preserved registers       | call list
; eax, ecx, edx       ebx, esi, edi, ebp, esp     ebp
;
; syscall convention int 80h. intel => parameters are passed by registers. sysnum: eax
; parameters registers order: ebx, ecx, edx, esi, edi, ebp
; return on eax (no carry flag feature)

segment .text

global user_kill
user_kill:
	push ebp
	mov ebp, esp

	push ebx

	mov ecx, [ebp + 12]
	mov ebx, [ebp + 8]

	mov eax, 37 ; system call number (sys_kill)
	int 80h

	pop ebx

	pop ebp
	ret
