[BITS 32]

;; This file contains all asm code regarding the interrupt service routines of the 8259 PIC
;; See https://wiki.osdev.org/ISR

extern _align_stack
extern generic_interrupt_handler
extern debug_pit

extern keyboard_interrupt_handler

extern timer_interrupt_handler

segment .data
_pic_time dd 0

segment .text

extern process_a
global _process_a
_process_a:
	push ebp
	mov ebp, esp
	pushad
	push 0
	push process_a
	call _align_stack
	popad
.loop:
	hlt
	jmp .loop
	pop ebp
	ret

global _isr_timer
_isr_timer:
	push ebp
	mov ebp, esp
	pushad
	push eax
	lock inc dword [_pic_time]
	; send EOI master pic, irq0

	mov eax, ebp
	add eax, 4
	push eax
	push 4
	push timer_interrupt_handler
	call _align_stack
	add esp, 12

	mov al, 0x20
	out 0x20, al
	pop eax

	popad
	pop ebp
	iret

global _get_pic_time
_get_pic_time:
	lock cmpxchg [_pic_time], eax
	ret

; have a while to make coffee
global _sleep
_sleep:
	push ebp
	mov ebp, esp

	mov edx, [ebp + 8]
	call _get_pic_time
	add edx, eax

.loop:
	hlt

	call _get_pic_time
	cmp eax, edx

	jb .loop

	pop ebp
	ret

;; This generates the Interrupt service routines. The first paramater completes the indentifier
;; the first parameter identified if is a master pic or slave irq
;; the second parameter compose the name of the exported symbol
;; The third paramater is the name of the interrupt as a string
;; The fourth parameter is the rust function to call for handling the interrupt
%macro CREATE_ISR 4
segment .data
	isr_%2_str: db %3, " interrupt", 0
segment .text
global _isr_%2
_isr_%2:
	push ebp
	mov ebp, esp
	pushad
	push isr_%2_str
	push 4
	push %4
	call _align_stack
	add esp, 12 ;pop interrupt string
	%1
	popad
	pop ebp
	iret
%endmacro

%macro MASTER 0
	mov al, 0x20
	out 0x20, al
%endmacro

%macro SLAVE 0
	MASTER
	mov al, 0xa0
	out 0xa0, al
%endmacro

	; TIPS: use nasm -E file to view source file on stdout after macro expansion
	CREATE_ISR MASTER, keyboard, "Keyboard", keyboard_interrupt_handler
	CREATE_ISR MASTER, cascade, "cascade, never used", generic_interrupt_handler ; should never be raised
	CREATE_ISR MASTER, com2, "COM2", generic_interrupt_handler
	CREATE_ISR MASTER, com1, "COM1", generic_interrupt_handler
	CREATE_ISR MASTER, lpt2, "LPT2", generic_interrupt_handler
	CREATE_ISR MASTER, floppy_disk, "floppy_disk", generic_interrupt_handler
	CREATE_ISR MASTER, lpt1, "lpt1", generic_interrupt_handler ; unreliable, often a spurious interrupt

	CREATE_ISR SLAVE, cmos, "CMOS real-time clock", generic_interrupt_handler
	CREATE_ISR SLAVE, acpi, "ACPI", generic_interrupt_handler
	CREATE_ISR SLAVE, ps2_mouse, "PS/2 mouse", generic_interrupt_handler
	CREATE_ISR SLAVE, fpu_coproc, "FPU / Coproc / inter-processor", generic_interrupt_handler
	CREATE_ISR SLAVE, primary_hard_disk, "Primary ATA hard disk", generic_interrupt_handler
	CREATE_ISR SLAVE, secondary_hard_disk, "Secondary ATA hard disk", generic_interrupt_handler
