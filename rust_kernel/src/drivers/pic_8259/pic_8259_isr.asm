[BITS 32]

;; This file contains all asm code regarding the interrupt service routines of the 8259 PIC
;; See https://wiki.osdev.org/ISR

extern generic_interrupt_handler
extern keyboard_interrupt_handler
extern _schedule_next

segment .data
_pic_time dd 0

; i32 for activation/divisor of scheduler
global SCHEDULER_COUNTER
SCHEDULER_COUNTER: dd 0

segment .text
global _isr_timer
_isr_timer:
	; PIT time
	lock inc dword [_pic_time]

	push eax
	mov al, 0x20
	out 0x20, al
	pop eax
	cmp dword [SCHEDULER_COUNTER], 0
	jl .end ; perform an signed comparaison: if SCHEDULER_COUNTER < 0, scheduler is not active

	; Return to kernel if scheduler not actif
	dec dword [SCHEDULER_COUNTER]
	jz _schedule_next

.end:
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
	call %4
	add esp, 4 ;pop interrupt string
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
