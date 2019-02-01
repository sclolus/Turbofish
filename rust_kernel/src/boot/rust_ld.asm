[BITS 32]

segment .text

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
