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
_OLD_EIP:	dd 0
_OLD_EAX:	dd 0
_OLD_ESP:	dd 0
_OLD_EFLAGS:	dd 0
_OLD_SEGMENT:	dd 0

; bool for activation of scheduler
global SCHEDULER_ACTIVE
SCHEDULER_ACTIVE:	db 0

segment .text
extern kernel_stack
global _isr_timer
_isr_timer:
	;; save values and move to kernel_stack
	mov [_OLD_EAX], eax
	pop eax
	mov [_OLD_EIP], eax
	pop eax
	mov [_OLD_SEGMENT], eax
	pop eax
	mov [_OLD_EFLAGS], eax

	;; PIT time
	lock inc dword [_pic_time]
	mov al, 0x20
	out 0x20, al
	cmp byte [SCHEDULER_ACTIVE], 1
	je .continue

	; return to kernel if scheduler not actif
	mov eax, [_OLD_EAX]
	push dword [_OLD_EFLAGS]
	push dword [_OLD_SEGMENT]
	push dword [_OLD_EIP]
	iret

.continue:
	; change stack to kernel stack
	mov eax, esp
	mov [_OLD_ESP], eax
	mov esp, kernel_stack

	mov eax, [_OLD_EAX]
	pushad

	mov eax, [_OLD_ESP]
	push eax

	mov eax, [_OLD_EFLAGS]
	push eax

	mov eax, [_OLD_SEGMENT]
	push eax

	mov eax, [_OLD_EIP]
	push eax

	push 8 * 4 + 4 + 4 + 4 + 4

	push timer_interrupt_handler
	call _align_stack

segment .data
TMP_EFLAGS:	dd 0
TMP_SEGMENT:	dd 0
TMP_EIP:	dd 0
TMP_ESP:	dd 0
segment .text
extern debug_process
global _switch_process
;; fn _switch_process(CpuState {eip: u32, segment: u32, eflags: u32, esp: u32, registers: BaseRegisters}) -> !;
_switch_process:
	push ebp
	mov ebp, esp

	mov eax, dword [ebp + 8]
	mov [TMP_EIP], eax

	mov eax, dword [ebp + 12]
	mov [TMP_SEGMENT], eax

	mov eax, dword [ebp + 16]
	mov [TMP_EFLAGS], eax

	mov eax, dword [ebp + 20]
	mov [TMP_ESP], eax

	mov eax, ebp
	add eax, 24
	mov esp, eax
	popad

	mov esp, [TMP_ESP]

	push dword [TMP_EFLAGS]
	push dword [TMP_SEGMENT]
	push dword [TMP_EIP]
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
