[BITS 32]

extern main
extern user_exit

segment .text

%macro TESTREGISTER 1
	test %1, %1
	jne .test_failure
%endmacro

global _start
_start:
	; Ensure that all registers are 0 initialized
	TESTREGISTER eax
	TESTREGISTER ebx
	TESTREGISTER ecx
	TESTREGISTER edx
	TESTREGISTER esi
	TESTREGISTER edi
	TESTREGISTER ebp

	call main
	jmp .exit

.test_failure:
	mov eax, -1

.exit:
	push eax
	call user_exit
