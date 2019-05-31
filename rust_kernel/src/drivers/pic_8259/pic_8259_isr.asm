[BITS 32]

;; This file contains all asm code regarding the interrupt service routines of the 8259 PIC
;; See https://wiki.osdev.org/ISR

extern generic_interrupt_handler
extern keyboard_interrupt_handler

extern primary_hard_disk_interrupt_handler
extern secondary_hard_disk_interrupt_handler

segment .data

_pit_time dd 0

%define LOCKED 1
%define UNLOCKED 0
; The scheduler handler is not called until setting its variable to UNLOCKED
_interruptible_state dd LOCKED

_next_quantum dd 0

;                __ISR_TIMER__
;       PIT ACK ------|
;                     v
;                inc _pit_time                  _pit_time++
;                     |
;                     v
;    +----- NO Is_scheduler_interruptible ?     _interruptible_state == UNLOCKED
;    |            YES |
;    |                v
;    +----- NO Is_process_time_expired ?        _pit_time >= next_quantum
;    |            YES |
;    |                v
;    |            SCHEDULE_NEXT                 goto _schedule_next
;    |                |
;    v                v
; __IRET__         __IRET__

extern _schedule_next

segment .text

; This function is automatically called when Interrupt are enable and PIC irq 0 is enabled
global _isr_timer
_isr_timer:
	push eax

	mov al, 0x20
	out 0x20, al

	; inc _pit_time
	lock inc dword [_pit_time]

	; Is_scheduler_interruptible ?
	xor eax, eax
	lock cmpxchg dword [_interruptible_state], eax
	pop eax
	jnz .end

	; Is_process_time_expired
	push eax
	push edx
	lock cmpxchg dword [_pit_time], eax
	mov edx, eax
	lock cmpxchg dword [_next_quantum], eax
	cmp edx, eax
	pop edx
	pop eax
	; The schedule_next function MUST set the new _next_quantum value
	jae _schedule_next

.end:
	iret

; Avoid Atomically the preemptive call of the scheduler
global _no_interruptible
_no_interruptible:
	lock or dword [_interruptible_state], LOCKED
	ret

; Allow Atomically the preemptive call of the scheduler
global _interruptible
_interruptible:
	lock and dword [_interruptible_state], UNLOCKED
	ret

; Get atomically the actual pit time
global _get_pit_time
_get_pit_time:
	lock cmpxchg dword [_pit_time], eax
	ret

; Get atomically the actual next_quantum
global _get_next_quantum
_get_next_quantum:
	lock cmpxchg dword [_next_quantum], eax
	ret

; Update Atomically the next_quantum value
global _update_next_quantum
_update_next_quantum:
	call _get_pit_time
	add eax, dword [esp + 4]
	mov edx, eax
	call _get_next_quantum
	sub edx, eax

	; Update the next quantum
	lock add dword [_next_quantum], edx

	ret

; KERNEL MODE ONLY: have a while to make coffee
global _sleep
_sleep:
	push ebp
	mov ebp, esp

	mov edx, [ebp + 8]
	call _get_pit_time
	add edx, eax

.loop:
	hlt

	call _get_pit_time
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
	mov al, 0x20
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
CREATE_ISR SLAVE, primary_hard_disk, "Primary ATA hard disk", primary_hard_disk_interrupt_handler
CREATE_ISR SLAVE, secondary_hard_disk, "Secondary ATA hard disk", secondary_hard_disk_interrupt_handler
