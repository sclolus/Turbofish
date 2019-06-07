[BITS 32]

extern main
extern exit

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
	call exit

; execve("./signal", ["./signal"], [ /* 39 vars */]) = 0
; strace:	 [ Process PID=12467 runs in 32 bit mode. ]
; write(1, "initialise Signals test\n", 24initialise Signals test
; ) = 24
; sigaction(SIGINT, {sa_handler=0x400460, sa_mask=[], sa_flags=0}, NULL) = 0
; write(1, "signal function return: ", 24signal function return: ) = 24
; write(1, "0x", 20x)                       = 2
; write(1, "400460", 6400460)                   = 6
; write(1, "\n", 1
; )                       = 1
; ^C--- SIGINT {si_signo=SIGINT, si_code=SI_KERNEL} ---
; Signal Received 5/5: strace: Process 12467 detached
; 2
; Signal Received 5/5: 2
