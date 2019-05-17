[BITS 32]
align 16

segment .data

_esp: dd 0

segment .text

;; This function can be launched by the scheduler for each new process
;; It prepares a IRET stack frame witch contains new process coordinates and set that data segments, eflags and base registers
;;
;; +--------+
;; | SS     |
;; +--------+                   * Illustration of the kernel stack just before IRET
;; | ESP    |
;; +--------+
;; | EFLAGS |
;; +--------+
;; | CS     |
;; +--------+
;; | EIP    | <---- ESP
;; +--------+
;;
;; fn _launch_process(ss: u16, esp: u32, cs: u16, eip: u32, data_segment: u32, eflags: u32, registers: *BaseRegisters);
global _launch_process
_launch_process:
	push ebp
	mov ebp, esp

	; Temporary disable interrupts
	cli

	; Push SS then ESP
	push dword [ebp + 8]
	push dword [ebp + 12]

	; Push EFLAGS (must contains interrupt enable flag (bit 9))
	push dword [ebp + 28]

	; Push CS then EIP
	push dword [ebp + 16]
	push dword [ebp + 20]

	; Assign DS/ES/FS/GS segments for the future process
	mov ax, [ebp + 24]
	mov ds, ax
	mov es, ax
	mov fs, ax
	mov gs, ax

	; Put ESP on the BaseRegisters pointer to get all process's base registers (only ESP will be different)
	mov [_esp], esp
	mov esp, [ebp + 28]
	popad
	mov esp, [_esp]

	; JMP to process
	iret
