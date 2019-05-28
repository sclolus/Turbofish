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

global user_fork
user_fork:
	push ebp
	mov ebp, esp

	mov eax, 0x2 ; system call number (sys_fork)
	int 0x80

	pop ebp
	ret
