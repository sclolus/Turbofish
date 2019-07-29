[BITS 32]

segment .text

extern exit
extern main
global environ

global _start ; must be declared for linker (ld)
_start:       ; tell linker entry point
	push ebp
	mov ebp, esp

	push ecx ; push envp
	push ebx ; push argv
	push eax ; push argc
	mov [environ], ecx
	call main
	;; call init_libc
	;; this should never return

	add esp, 12

	push eax
	call exit

segment .data
environ:	dd 0x0
