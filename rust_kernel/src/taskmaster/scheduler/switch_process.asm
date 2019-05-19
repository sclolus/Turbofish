[BITS 32]

extern scheduler_interrupt_handler

segment .text

;; Simple and Basic schedule function (not premptive at all !)
;; Scheduler MUST be not interruptible !
;;
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
global _schedule_next
_schedule_next:
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
	call scheduler_interrupt_handler
	; Set the new stack pointer
	mov esp, eax

	; Recover all purpose registers
	popad
	pop gs
	pop fs
	pop es
	pop ds

	; Return contains now new registers, new eflags, new esp and new eip
	iret
