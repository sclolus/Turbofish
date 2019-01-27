[BITS 32]

;; This file contains all asm code regarding the interrupt service routines
;; For now. just a generic ISR wrapper
;; See https://wiki.osdev.org/ISR

;; Currently no reason have macro for those, but could be usefull at some point
%macro PUSH_ALL_REGISTERS 0
	pushad
%endmacro

%macro POP_ALL_REGISTERS 0
	popad
%endmacro

extern generic_interrupt_handler

;; This generates the Interrupt service routines. The first paramater completes the indentifier
;; The second paramater is the name of the interrupt as a string
;; The third parameter is the rust function to call for handling the interrupt
%macro CREATE_ISR 3
segment .data
	isr_%1_str:
	db %2, " interrupt", 0
segment .text
global _isr_%1
	_isr_%1:
	push	ebp
	mov	ebp, esp
	PUSH_ALL_REGISTERS
	push	isr_%1_str
	call	%3
	add	esp, 4 					;pop interrupt string
	POP_ALL_REGISTERS
	pop	ebp
	iret
%endmacro

	CREATE_ISR timer, "Timer", generic_interrupt_handler
	CREATE_ISR keyboard, "Keyboard", generic_interrupt_handler
	CREATE_ISR cascade, "cascade, never used", generic_interrupt_handler ; should never be raised
	CREATE_ISR com2, "COM2", generic_interrupt_handler
	CREATE_ISR com1, "COM1", generic_interrupt_handler
	CREATE_ISR lpt2, "LPT2", generic_interrupt_handler
	CREATE_ISR floppy_disk, "floppy_disk", generic_interrupt_handler
	CREATE_ISR lpt1, "lpt1", generic_interrupt_handler 		; unreliable, often a spurious interrupt
	CREATE_ISR cmos, "CMOS real-time clock", generic_interrupt_handler
	CREATE_ISR acpi, "ACPI", generic_interrupt_handler
	CREATE_ISR ps2_mouse, "PS/2 mouse", generic_interrupt_handler
	CREATE_ISR fpu_coproc, "FPU / Coproc / inter-processor", generic_interrupt_handler
	CREATE_ISR primary_hard_disk, "Primary ATA hard disk", generic_interrupt_handler
	CREATE_ISR secondary_hard_disk, "Secondary ATA hard disk", generic_interrupt_handler

