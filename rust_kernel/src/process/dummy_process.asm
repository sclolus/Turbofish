[BITS 32]

segment .text
align 16

global _dummy_process_code
global _dummy_process_len

%macro STO 1
	mov ax, %1
	stosb
%endmacro

_dummy_process_code:
	push ebp
	mov ebp, esp

	mov edi, 0x400100
	STO 'H'
	STO 'e'
	STO 'l'
	STO 'l'
	STO 'o'
	STO ' '
	STO 'w'
	STO 'o'
	STO 'r'
	STO 'l'
	STO 'd'
	STO ' '
	STO 'f'
	STO 'r'
	STO 'o'
	STO 'm'
	STO ' '
	STO 'u'
	STO 's'
	STO 'e'
	STO 'r'
	STO 's'
	STO 'p'
	STO 'a'
	STO 'c'
	STO 'e'
	STO ' '
	STO '!'
	STO 10

	mov eax, 4
	mov ebx, 1
	mov ecx, 0x400100
	mov edx, 29

.loop:
	int 80h
	jmp .loop

.ud2
	ud2

	jmp $

_dummy_process_len:    dd $-_dummy_process_code

;; fn _ring3_switch(ss: u16, esp: u32, cs: u16, eip: u32);
global _ring3_switch
_ring3_switch:
	push ebp
	mov ebp, esp

	; Disable interrupt temporaly
	cli

	; Push SS then ESP
	push dword [ebp + 8]
	push dword [ebp + 12]

	; Push EFLAGS
	pushf
	pop eax
	; Reactivation of interruption when we go in ring 3
	or eax, 0x200
	push eax

	; Push CS then EIP
	push dword [ebp + 16]
	push dword [ebp + 20]

	; Assign DS/ES/FS/GS segments for the future ring3 process
	mov ax, 0x28
	mov ds, ax
	mov es, ax
	mov fs, ax
	mov gs, ax

	; JMP to ring 3 process
	iret

	; ------------------------------
	; This could be never happened !
	mov ax, 0x10
	mov ds, ax
	mov es, ax
	mov fs, ax
	mov gs, ax

	add esp, 20
	sti
	pop ebp
	ret
