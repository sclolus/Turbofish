[BITS 32]

section .text
	; GRUB multiboot spec
align 4
	dd 0x1BADB002                ; magic
	dd 0b11                      ; flags
	dd - (0x1BADB002 + 0b11)     ; checksum. m+f+c should be zero

extern kmain
extern init_gdt

; Hack of _Unwind_Resume for Rust linking
global _Unwind_Resume
_Unwind_Resume:
	push ebp
	mov ebp, esp

	jmp $

; Hack of rust_eh_personnality for rust compilation in debug mode
global rust_eh_personality
rust_eh_personality:
	push ebp
	mov ebp, esp

	jmp $

global _start
global _start_after_init_gdt
_start:
	cli                             ; block interrupts

	push ebp
	mov ebp, esp

	; set EIP of caller GRUB on stack at 0, prevent infinite loop for backtrace
	mov [ebp + 4], dword 0

	mov esp, stack_space            ; set stack pointer for a temporary stack

	call disable_cursor
	jmp init_gdt
_start_after_init_gdt:	

	call set_sse2

	call rust_ebp_wrapper
	; --------------------------------------------------------------------

rust_ebp_wrapper:
	push ebp
	mov ebp, esp

	; EBX contain pointer to GRUB multiboot information (preserved register)
	push ebx
	call kmain                      ; kmain is called with this param

	; ---------------------------------------------------------------------
	jmp $

set_sse2:
	push ebp
	mov ebp, esp
	pushad

	mov eax, 0x1
	cpuid
	test edx, 1 << 26     ; test if SSE2 feature exist
	jz .end_set_sse2

	mov eax, cr0
	and ax, 0xFFFB        ; clear coprocessor emulation CR0.EM
	or ax, 0x2            ; set coprocessor monitoring  CR0.MP
	mov cr0, eax
	mov eax, cr4
	or ax, 3 << 9         ; set CR4.OSFXSR and CR4.OSXMMEXCPT at the same time
	mov cr4, eax

	.end_set_sse2:
	popad
	pop ebp
	ret

disable_cursor:
	pushf
	push eax
	push edx
 
	mov dx, 0x3D4
	mov al, 0xA	; low cursor shape register
	out dx, al
 
	inc dx
	mov al, 0x20	; bits 6-7 unused, bit 5 disables the cursor, bits 0-4 control the cursor shape
	out dx, al
 
	pop edx
	pop eax
	popf
	ret

section .bss
resb 8192                 ; 8KB for temporary stack
stack_space:
