[BITS 32]

segment .text

;; This function can be launched by the scheduler for each new process
;; It prepares a IRET stack frame witch contains new process coordinates and set that data segments, eflags and base registers
;;
;; +--------+               ^ (to high memory)
;; | SS     |               |
;; +--------+                    * Illustration of the kernel stack just before IRET
;; | ESP    |
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
;; +--------+ ---> pointer to CpuState Structure (arg n~1 of this function)
;;
;; fn _launch_process(cpu_state: *const CpuState);
global _launch_process
_launch_process:
	push ebp
	mov ebp, esp

	; Temporary disable interrupts
	cli

	; Follow the CpuState pointer (pass to breaking stack context)
	mov esp, [ebp + 8]

	; Recover all purpose registers
	popad
	pop gs
	pop fs
	pop es
	pop ds

	; Return contains now new registers, new eflags, new esp and new eip
	iret
