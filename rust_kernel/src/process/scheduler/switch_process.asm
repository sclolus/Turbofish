[BITS 32]

extern scheduler_interrupt_handler

segment .data

_eip: dd 0
_eflags: dd 0
_esp: dd 0

segment .text

;; Get process values, move to kernel_stack and launch schedule
;; Scheduler MUST be not interruptible !
global _schedule_next
_schedule_next:
	; Get EIP, EFLAGS and ESP of current process before switch
	pop dword [_eip]
	add esp, 4 					; CS is present here
	pop dword [_eflags]
	pop dword [_esp]
	; pop dword [_ss]

	sub esp, 16

	; Generate the struct CpuState on the stack :)
	pushad
	push dword [_eflags]
	push dword [_esp]
	push dword [_eip]
	; --- MUST PASS POINTER TO THAT STRUCTURE ---
	push esp
	call scheduler_interrupt_handler
	; Skip last arg
	add esp, 4

	; Recover all the newest states for the next process
	pop dword [_eip]
	pop dword [_esp]
	pop dword [_eflags]
	popad

	; Apply the coordinates of the new process for below IRET
	push eax
	mov eax, [_eip]
	mov [esp + 4], eax
	mov eax, [_eflags]
	mov [esp + 12], eax
	mov eax, [_esp]
	mov [esp + 16], eax
	pop eax

	; Return contains now new registers, new eflags, new esp and new eip
	iret
