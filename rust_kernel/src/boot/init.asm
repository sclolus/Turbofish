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

	; set up the stack pointer for a temporary stack
	; dont worry about overflow for stack, the first push will be at [temporary_stack - 4], not in [temporary_stack]
	mov esp, temporary_stack
	mov ebp, esp

	call .disable_cursor

	call _init_gdt

	; SS IS STACK SEGMENT REGISTER
	mov ax, 0x18
	mov ss, ax

	; set the base EIP on stack at 0x0, prevent infinite loop for backtrace

	; set up the main kernel stack
	;      stack frame 2             | stack frame 1             | stack frame 0
	; <--- (EBP EIP ARGx ... VARx ...) (EBP EIP ARGx ... VARx ...) ((ptr - 8) 0x0) | *** kernel_stack SYMBOL *** |
	;                                  <----     ESP EXPANSION   |    *ebp

	; This mechanism is for Panic handler. See details on 'panic.rs' file
	mov [kernel_stack - 4], dword 0x0
	mov esp, kernel_stack - 8
	mov ebp, esp

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

; 4kb for temporary stack
resb 1 << 12
temporary_stack:

; 1mo for the main kernel stack
resb 1 << 20
kernel_stack:
