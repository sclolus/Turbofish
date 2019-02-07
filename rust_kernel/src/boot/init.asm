[BITS 32]

segment .text

extern kmain
extern _init_gdt
extern _align_stack

extern _set_sse
extern _set_avx
extern _set_fpu

global _init
_init:
	; block interrupts
	cli

	push ebp
	mov ebp, esp

	; set EIP of caller GRUB on stack at 0, prevent infinite loop for backtrace
	mov [ebp + 4], dword 0

	; set stack pointer for a temporary stack
	mov esp, stack_space

	call .disable_cursor

	call _init_gdt

	; SS IS STACK SEGMENT REGISTER
	mov ax, 0x18
	mov ss, ax

	; put the stack at 8MB
	mov esp, 0x800000

	call _set_sse
	call _set_avx
	call _set_fpu

	; EBX contain pointer to GRUB multiboot information (preserved register)
	push ebx
	push 4
	; kmain is called with EBX param
	push kmain

	call _align_stack

	add esp, 12

.idle:
	hlt
	jmp .idle

.disable_cursor:
	push eax
	push edx

	mov dx, 0x3D4
	; low cursor shape register
	mov al, 0xA
	out dx, al

	inc dx
	; bits 6-7 unused, bit 5 disables the cursor, bits 0-4 control the cursor shape
	mov al, 0x20
	out dx, al

	pop edx
	pop eax
	ret

segment .bss
; 8KB for temporary stack
resb 8192
stack_space:
