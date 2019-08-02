[BITS 32]

segment .text

extern main
extern exit

global _start ; must be declared for linker (ld)
_start:       ; tell linker entry point
	push ebp
	mov ebp, esp

	push ecx ; push envp
	push ebx ; push argv
	push eax ; push argc
	call main
	add esp, 12

	push 0
	call exit
