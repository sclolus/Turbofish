[BITS 32]

extern _align_stack
extern syscall_interrupt_handler

global _isr_syscall

_isr_syscall:
    ; already on stack: ss, sp, flags, cs, ip.
	push ebp
	mov ebp, esp
	;; push ss
	;; push ds
	;; push es
	;; push fs
	;; push gs
	;; pushad
	push ebp
	push edi
	push esi
	push edx
	push ecx
	push ebx
	push eax
	push 4 * 7
	push syscall_interrupt_handler
	call _align_stack
	add esp, 8
	; ignore eax, as eax is the return value of the syscall
	add esp, 4
	pop ebx
	pop ecx
	pop edx
	pop esi
	pop edi
	pop ebp
	;; add esp, 28
	;; popad
	;; pop gs
	;; pop fs
	;; pop es
	;; pop ds
	;; pop ss
	pop ebp
	iret
