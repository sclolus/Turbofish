[BITS 32]

extern syscall_interrupt_handler
extern kernel_stack

segment .data
_pic_time dd 0
_OLD_EIP:	dd 0
_OLD_EAX:	dd 0
_OLD_ESP:	dd 0
_OLD_EFLAGS:	dd 0
_OLD_SEGMENT:	dd 0

segment .text
global _isr_syscall

_isr_syscall:
    ; already on stack: ss, sp, flags, cs, ip.
	mov [_OLD_EAX], eax
	pop eax
	mov [_OLD_EIP], eax
	pop eax
	mov [_OLD_SEGMENT], eax
	pop eax
	mov [_OLD_EFLAGS], eax

	mov eax, esp
	mov [_OLD_ESP], eax
	mov esp, kernel_stack

	mov eax, [_OLD_EAX]
	pushad

	mov eax, [_OLD_ESP]
	push eax

	mov eax, [_OLD_EFLAGS]
	push eax

	mov eax, [_OLD_SEGMENT]
	push eax

	mov eax, [_OLD_EIP]
	push eax

	call syscall_interrupt_handler
	; no return
	ud2
