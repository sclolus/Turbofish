[BITS 32]

extern syscall_interrupt_handler

segment .text

;; +--------+               ^ (to high memory)
;; | SS     | TSS ONLY      |
;; +--------+                    * Illustration of the kernel stack just before IRET
;; | ESP    | TSS ONLY
;; +--------+
;; | EFLAGS |
;; +--------+
;; | CS     |
;; +--------+
;; | EIP    | <---- ESP on the first instruction ---
;; +--------+
;; | DS     |
;; +--------+
;; | ES     |
;; +--------+
;; | FS     |
;; +--------+
;; | GS     |
;; +--------+
;; | REGS   |
;; |    ... |
;; |    ... |
;; +--------+
;; | 0x0    |
;; +--------+ ---> pointer to CpuState Structure
global _isr_syscall
_isr_syscall:
	; Generate the struct CpuState on the stack :)
	push ds
	push es
	push fs
	push gs
	pushad

	; Push 0x0 for backtrace endpoint
	push dword 0

	; Assign kernel data segments
	mov ax, 0x10
	mov ds, ax
	mov es, ax
	mov fs, ax
	mov gs, ax

	; --- MUST PASS POINTER TO THAT STRUCTURE ---
	push esp
	mov ebp, esp				; set the backtrace endpoint
	call syscall_interrupt_handler
	add esp, 4

	add esp, 4					; skip stack reserved field

	; Recover all purpose registers
	popad
	pop gs
	pop fs
	pop es
	pop ds

	; Return contains now new registers, new eflags, new esp and new eip
	iret
