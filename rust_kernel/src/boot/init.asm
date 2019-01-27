[BITS 32]

segment .text

extern kmain
extern init_gdt

extern debug_center
global _start_after_init_gdt

global init
init:
	cli							; block interrupts

	push ebp
	mov ebp, esp

	; set EIP of caller GRUB on stack at 0, prevent infinite loop for backtrace
	mov [ebp + 4], dword 0

	mov esp, stack_space            ; set stack pointer for a temporary stack

	call disable_cursor
	jmp init_gdt
_start_after_init_gdt:
;	call debug_center
	call set_sse2
	call enable_avx

	finit						; init the FPU

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
	or eax, 3 << 9         ; set CR4.OSFXSR and CR4.OSXMMEXCPT at the same time
	or eax, 1 << 18		   ; Enable OSXSAVE instructions
	mov cr4, eax

	.end_set_sse2:
	popad
	pop ebp
	ret

enable_avx:
    push eax
    push ecx

    xor ecx, ecx

    xgetbv              ;Load XCR0 register

    or eax, 7           ;Set AVX, SSE, X87 bits
    xsetbv              ;Save back to XCR0

    pop ecx
    pop eax

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


section .bss
resb 8192                 ; 8KB for temporary stack
stack_space:
