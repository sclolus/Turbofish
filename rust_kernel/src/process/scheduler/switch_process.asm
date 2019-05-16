[BITS 32]

extern scheduler_interrupt_handler
extern kernel_stack

segment .data
_eax: dd 0

_esp: dd 0
_eflags: dd 0
_cs: dd 0
_eip: dd 0

segment .text

; Get process values, move to kernel_stack and launch schedule
global _prepare_switch
_prepare_switch:
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

	call scheduler_interrupt_handler

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
