[BITS 32]

; Calling convention: Platform System V i386
; ------------------------------------------
; Return Value | Parameter Registers | Additional Parameters | Stack Alignment
; eax, edx       none                  stack (right to left)   none
;
; Scratch Registers | Preserved Registers       | Call List
; eax, ecx, edx       ebx, esi, edi, ebp, esp     ebp
;
; Syscall convention INT 80H. INTEL => Parameters are passed by registers. SysNum: EAX
; Parameters registers order: EBX, ECX, EDX, ESI, EDI, EBP
; Return on EAX (no carry flag feature)

segment .text

global user_nanosleep
user_nanosleep:
	push ebp
	mov ebp, esp

	push ebx

	mov ecx, [ebp + 12]
	mov ebx, [ebp + 8]

	mov eax, 162 ; system call number (sys_nanosleep)
	int 80h

	pop ebx

	pop ebp
	ret
