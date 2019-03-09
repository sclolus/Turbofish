[BITS 32]

segment .text

extern kmain

extern _set_sse
extern _set_avx
extern _set_fpu

; This function is launched in high half memory area
global _init_kernel
_init_kernel:
	cli

	push ebp
	mov ebp, esp

	; Get the pointer to the grub multiboot header
	mov eax, [ebp + 8]
	; Get the pointer to the device memory map
	mov ebx, [ebp + 12]

	; Set the base EIP on stack at 0x0, prevent infinite loop for backtrace

	; set up the main kernel stack
	;      stack frame 2             | stack frame 1             | stack frame 0
	; <--- (EBP EIP ARGx ... VARx ...) (EBP EIP ARGx ... VARx ...) ((ptr - 8) 0x0) | *** kernel_stack SYMBOL *** |
	;                                  <----     ESP EXPANSION   |    *ebp

	; This mechanism is for Panic handler. See details on 'panic.rs' file
	; dont worry about overflow for stack, the first push will be at [temporary_stack - 4], not in [temporary_stack]
	mov [kernel_stack - 4], dword 0x0
	mov esp, kernel_stack - 8
	mov ebp, esp

	; Push the arguments pointers
	push ebx
	push eax

	; Initialize advanced features
	call _set_sse
	call _set_avx
	call _set_fpu

	; Finally call the RUST kernel
	call kmain

	add esp, 8

.idle:
	hlt
	jmp .idle

segment .bss
align 16

; 1mo for the main kernel stack
resb 1 << 20
kernel_stack:
