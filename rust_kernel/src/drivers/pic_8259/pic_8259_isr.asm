[BITS 32]

;; This file contains all asm code regarding the interrupt service routines of the 8259 PIC
;; See https://wiki.osdev.org/ISR

extern generic_interrupt_handler
extern debug_pit

extern keyboard_interrupt_handler

extern timer_interrupt_handler

segment .data
_pic_time dd 0

_eax: dd 0

_esp: dd 0
_eflags: dd 0
_cs: dd 0
_eip: dd 0

; bool for activation of scheduler
global SCHEDULER_ACTIVE
SCHEDULER_ACTIVE: db 0

segment .text
extern kernel_stack
global _isr_timer
_isr_timer:
	; PIT time
	lock inc dword [_pic_time]
	push eax
	mov al, 0x20
	out 0x20, al
	pop eax
	cmp byte [SCHEDULER_ACTIVE], 1
	je .prepare_switch
	; Return to kernel if scheduler not actif
	iret

; Get process values, move to kernel_stack and launch schedule
.prepare_switch:
	; Get EIP, CS and EFLAGS of current process before interrupt
	; TODO from ring 3: SS & ESP must be taken
	pop dword [_eip]
	pop dword [_cs]
	pop dword [_eflags]

	; Save the process stack and change stack to kernel stack
	mov [_eax], eax
	mov eax, esp
	mov [_esp], eax
	mov eax, [_eax]
	; TODO With TSS segment, it will be useless to manually set the kernel stack pointer
	mov esp, kernel_stack

	; Push all the process purpose registers
	pushad
	push dword [_esp]
	push dword [_eflags]
	push dword [_cs]
	push dword [_eip]

	call timer_interrupt_handler

segment .text
extern debug_process

; fn _switch_process(CpuState {eip: u32, cs: u32, eflags: u32, esp: u32, registers: BaseRegisters}) -> !;
global _switch_process
_switch_process:
	push ebp
	mov ebp, esp

	; Get all the passed arguments
	add esp, 8
	pop dword [_eip]
	pop dword [_cs]
	pop dword [_eflags]
	pop dword [_esp]
	popad

	mov esp, [_esp]

	; Do the IRET switch
	; WARNING: IRET does not handle SS & ESP ?
	push dword [_eflags]
	push dword [_cs]
	push dword [_eip]
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
