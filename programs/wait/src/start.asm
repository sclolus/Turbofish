[BITS 32]

segment .text

extern main
extern user_exit

global _start ; must be declared for linker (ld)
_start:       ; tell linker entry point
	push ebp
	mov ebp, esp

	call main

	push 0
	call user_exit
