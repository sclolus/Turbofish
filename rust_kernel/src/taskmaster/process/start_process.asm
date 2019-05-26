[BITS 32]

segment .text

;; This function can be launched for each new process
;; It prepares a IRET stack frame witch contains new process coordinates and set that data segments, eflags and base registers
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
;; | EIP    | <---- ESP before IRET instruction
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
;; +--------+ ---> pointer to CpuState Structure (arg n~1 of this function)
;;
;; fn _start_process(kernel_esp: u32);
global _start_process
_start_process:
	push ebp
	mov ebp, esp

	; Temporary disable interrupts
	cli

	; Follow the kernel_esp pointer (pass to breaking stack context)
	mov esp, [ebp + 8]

	add esp, 4                  ; skip stack_reserved field

	; Recover all purpose registers
	popad
	pop gs
	pop fs
	pop es
	pop ds

	; Return contains now new registers, new eflags, new esp and new eip
	iret
