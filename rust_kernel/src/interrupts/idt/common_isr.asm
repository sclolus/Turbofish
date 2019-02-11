[BITS 32]

;; This file contains all asm code regarding the interrupt service routines
;; For now. just a generic ISR wrapper
;; See https://wiki.osdev.org/ISR

extern _align_stack
extern generic_interrupt_handler
extern debug_pit

segment .data
_pic_time dd 0

segment .text
global _isr_timer
_isr_timer:
	push eax
	lock inc dword [_pic_time]
	; send EOI master pic, irq0

	pushad

	call _get_pic_time
	push eax
	push 4
	push debug_pit
	call _align_stack
	add esp, 12

	popad

	mov al, 0x20
	out 0x20, al
	pop eax
	iret

global _get_pic_time
_get_pic_time:
	lock cmpxchg [_pic_time], eax
	ret

global _default_isr
_default_isr:
    iret

;; This generates the Interrupt service routines. The first paramater completes the indentifier
;; The second paramater is the name of the interrupt as a string
;; The third parameter is the rust function to call for handling the interrupt
%macro CREATE_MASTER_ISR 3
segment .data
	isr_%1_str:
	db %2, " interrupt", 0
segment .text
global _isr_%1
	_isr_%1:
	push ebp
	mov	ebp, esp
	pushad
	push isr_%1_str
	push 4
	push %3
	call _align_stack
	add esp, 12 ;pop interrupt string

	mov al, 0x20
	out 0x20, al

	popad
	pop	ebp
	iret
%endmacro

%macro CREATE_SLAVE_ISR 3
segment .data
	isr_%1_str:
	db %2, " interrupt", 0
segment .text
global _isr_%1
	_isr_%1:
	push ebp
	mov	ebp, esp
	pushad
	push isr_%1_str
	push 4
	push %3
	call _align_stack
	add esp, 12 ;pop interrupt string

	mov al, 0x20
	out 0x20, al
	mov al, 0xa0
	out 0xa0, al

	popad
	pop	ebp
	iret
%endmacro

	CREATE_MASTER_ISR keyboard, "Keyboard", generic_interrupt_handler
	CREATE_MASTER_ISR cascade, "cascade, never used", generic_interrupt_handler ; should never be raised
	CREATE_MASTER_ISR com2, "COM2", generic_interrupt_handler
	CREATE_MASTER_ISR com1, "COM1", generic_interrupt_handler
	CREATE_MASTER_ISR lpt2, "LPT2", generic_interrupt_handler
	CREATE_MASTER_ISR floppy_disk, "floppy_disk", generic_interrupt_handler
	CREATE_MASTER_ISR lpt1, "lpt1", generic_interrupt_handler 		; unreliable, often a spurious interrupt

	CREATE_SLAVE_ISR cmos, "CMOS real-time clock", generic_interrupt_handler
	CREATE_SLAVE_ISR acpi, "ACPI", generic_interrupt_handler
	CREATE_SLAVE_ISR ps2_mouse, "PS/2 mouse", generic_interrupt_handler
	CREATE_SLAVE_ISR fpu_coproc, "FPU / Coproc / inter-processor", generic_interrupt_handler
	CREATE_SLAVE_ISR primary_hard_disk, "Primary ATA hard disk", generic_interrupt_handler
	CREATE_SLAVE_ISR secondary_hard_disk, "Secondary ATA hard disk", generic_interrupt_handler

