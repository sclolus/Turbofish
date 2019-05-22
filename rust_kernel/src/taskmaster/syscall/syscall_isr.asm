[BITS 32]

extern syscall_interrupt_handler

segment .text

global _isr_syscall
_isr_syscall:
	; save all registers (except ESP, SS, EFLAGS, EIP and CS (changed by interrupt: Handled by IRQ & TSS))
	; TODO: I think the exploitation of a stackframe here could be very difficult
	push ebp
	mov ebp, esp

	push ds
	push es
	push fs
	push gs
	; We will use this pushad as main argument for syscall_interrupt_handler
	pushad

	mov ax, 0x10
	mov ds, ax
	mov es, ax
	mov fs, ax
	mov gs, ax

	call syscall_interrupt_handler

	; Restore all registers
	popad
	pop gs
	pop fs
	pop es
	pop ds

	pop ebp

	; After that iret op, ESP, SS, EFLAGS, EIP and CS will be return as process values
	iret
