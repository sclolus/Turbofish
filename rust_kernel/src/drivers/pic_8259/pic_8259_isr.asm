[BITS 32]

;; This file contains all asm code regarding the interrupt service routines of the 8259 PIC
;; See https://wiki.osdev.org/ISR

extern generic_interrupt_handler

segment .data

_pit_time dd 0

%define LOCKED 1
%define UNLOCKED 0
; The scheduler handler is not called until setting its variable to UNLOCKED
_preemptible_state dd LOCKED

_process_end_time dd 0

;                __ISR_TIMER__
;       PIT ACK ------|
;                     v
;                inc _pit_time                  _pit_time++
;                     |
;                     v
;    +----- NO Is_scheduler_preemptible ?     _preemptible_state == UNLOCKED
;    |            YES |
;    |                v
;    +----- NO Is_process_time_expired ?        _pit_time >= process_end_time
;    |            YES |
;    |                v
;    |            SCHEDULE_NEXT                 goto _schedule_next
;    |                |
;    v                v
; __IRET__         __IRET__

extern _schedule_next

segment .text

%macro GET_PIT_TIME 0
	lock cmpxchg dword [_pit_time], eax
%endmacro

%macro GET_PROCESS_END_TIME 0
	lock cmpxchg dword [_process_end_time], eax
%endmacro

; This function is automatically called when Interrupt are enable and PIC irq 0 is enabled
global _isr_timer
_isr_timer:
	push eax

	mov al, 0x20
	out 0x20, al

	; inc _pit_time
	lock inc dword [_pit_time]

	; Is_scheduler_preemptible ?
	xor eax, eax
	lock cmpxchg dword [_preemptible_state], eax
	pop eax
	jnz .end

	; Is_process_time_expired
	push eax
	push edx
	GET_PIT_TIME
	mov edx, eax
	GET_PROCESS_END_TIME
	cmp edx, eax
	pop edx
	pop eax
	; The schedule_next function MUST set the new _process_end_time value
	jae _schedule_next

.end:
	iret

; Avoid Atomically the preemptive call of the scheduler
; Return the current preemptive status
; 0 => UNLOCKED
; 1 => LOCKED
; fn _unpreemptible() -> u32;
global _unpreemptible
_unpreemptible:
	xor eax, eax
	mov edx, dword LOCKED
	lock cmpxchg dword [_preemptible_state], edx
	; if the destination operand [_preemptible_state] (=0) is egal to EAX (0). The EDX value (1: LOCKED) is loaded into [_preemptible_state]. EAX return stay 0.
	; if the destination operand [_preemptible_state] (=1) is different to EAX (0). EAX take the value of [_preemptible_state] so 1.
	; On return: if EAX == 0. The [_preemptible_state] wasn't locked before calling 'lock cmpxchg'
	;            if EAX == 1. The [_preemptible_state] was already on a locked state
	ret

; Allow Atomically the preemptive call of the scheduler
global _preemptible
_preemptible:
	lock and dword [_preemptible_state], UNLOCKED
	ret

; Get the actual preemptible state (good for debugging)
global _get_preemptible_state
_get_preemptible_state:
	xor eax, eax
	lock cmpxchg dword [_preemptible_state], eax
	ret

; Get atomically the actual pit time
global _get_pit_time
_get_pit_time:
	GET_PIT_TIME
	ret

; Get atomically the actual process_end_time
global _get_process_end_time
_get_process_end_time:
	GET_PROCESS_END_TIME
	ret

; Update Atomically the process_end_time value
; _process_end_time = _pit_time + arg
global _update_process_end_time
_update_process_end_time:
	GET_PIT_TIME
	add eax, dword [esp + 4]
	lock xchg dword [_process_end_time], eax

	ret

; KERNEL MODE ONLY: have a while to make coffee
global _sleep
_sleep:
	push ebp
	mov ebp, esp

	mov edx, [ebp + 8]
	GET_PIT_TIME
	add edx, eax

.loop:
	hlt

	GET_PIT_TIME
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
	mov eax, dword [_pic_handlers_array + %4 * 4]
	call eax
	add esp, 4 ; pop interrupt string
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
	mov al, 0x20
	out 0xa0, al
	out 0x20, al
%endmacro

; Remove that dummy spurious behavior if we would use lpt1
%macro SPURIOUS_IRQ7 0
%endmacro

; Remove that dummy spurious behavior if we would use secondary_hard_disk
%macro SPURIOUS_IRQ15 0
	MASTER
%endmacro

; TIPS: use nasm -E file to view source file on stdout after macro expansion
CREATE_ISR MASTER, keyboard, "Keyboard", 1
CREATE_ISR MASTER, cascade, "cascade, never used", 2 ; should never be raised
CREATE_ISR MASTER, com2, "COM2", 3
CREATE_ISR MASTER, com1, "COM1", 4
CREATE_ISR MASTER, lpt2, "LPT2", 5
CREATE_ISR MASTER, floppy_disk, "floppy_disk", 6
CREATE_ISR SPURIOUS_IRQ7, lpt1, "lpt1", 7

CREATE_ISR SLAVE, cmos, "CMOS real-time clock", 8
CREATE_ISR SLAVE, acpi, "ACPI", 9
CREATE_ISR SLAVE, irq10, "irq10", 10
CREATE_ISR SLAVE, irq11, "irq11", 11
CREATE_ISR SLAVE, ps2_mouse, "PS/2 mouse", 12
CREATE_ISR SLAVE, fpu_coproc, "FPU / Coproc / inter-processor", 13
CREATE_ISR SLAVE, primary_hard_disk, "Primary ATA hard disk", 14
CREATE_ISR SPURIOUS_IRQ15, secondary_hard_disk, "Secondary ATA hard disk", 15

segment .data
GLOBAL _pic_handlers_array
_pic_handlers_array: times 16 dd generic_interrupt_handler
