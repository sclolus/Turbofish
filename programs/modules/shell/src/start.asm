[BITS 32]

extern main
extern exit

segment .text

global _start
_start:
	call main
	jmp .exit

.test_failure:
	mov eax, -1

.exit:
	push eax
	call exit
