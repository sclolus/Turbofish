[BITS 32]

extern syscall_interrupt_handler
; extern kernel_stack

segment .data
; _eip: dd 0
; _eax: dd 0
; _esp: dd 0
; _eflags: dd 0
; _cs: dd 0

segment .text
global _isr_syscall

_isr_syscall:
	; save all registers (except ESP, SS, EFLAGS, EIP and CS (changed by interrupt: Handled by IRQ & TSS))
	; TODO: I think the exploitation of a stackframe here could be very difficult
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

	; restore all registers
	popad
	pop gs
	pop fs
	pop es
	pop ds

	; After IRET, ESP, SS, EFLAGS, EIP and CS will be return as process values
	iret

	; already on stack: ss, sp, flags, cs, ip.
	; pop dword [_eip]
	; pop dword [_cs]
	; pop dword [_eflags]

	; Save the process stack and change stack to kernel stack
	; mov [_eax], eax
	; mov eax, esp
	; mov [_esp], eax
	; mov esp, kernel_stack
	; mov eax, [_eax]

	; Push all the process purpose registers
	; pushad
	; push dword [_esp]
	; push dword [_eflags]
	; push dword [_cs]
	; push dword [_eip]

	; call syscall_interrupt_handler
	; no return
	; ud2
