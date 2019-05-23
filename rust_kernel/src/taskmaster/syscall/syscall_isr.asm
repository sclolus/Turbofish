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
;; +--------+ ---> pointer to CpuState Structure

global _isr_syscall
_isr_syscall:
	; Generate the struct CpuState on the stack :)
	push ds
	push es
	push fs
	push gs
	pushad

	; Assign kernel data segments
	mov ax, 0x10
	mov ds, ax
	mov es, ax
	mov fs, ax
	mov gs, ax

	; --- MUST PASS POINTER TO THAT STRUCTURE ---
	push esp
	call syscall_interrupt_handler
	add esp, 4

	; Recover all purpose registers
	popad
	pop gs
	pop fs
	pop es
	pop ds

	; Return contains now new registers, new eflags, new esp and new eip
	iret
