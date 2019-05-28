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

; execve("./mordak", ["./mordak"], [/* 39 vars */]) = 0
; strace: [ Process PID=15940 runs in 32 bit mode. ]
; write(1, "initialise Mordak's Sodo-test\n", 30initialise Mordak's Sodo-test
; ) = 30
; mmap(NULL, 2097152, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0xf757f000
; mmap(NULL, 65536, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0xf756f000
; mmap(NULL, 65536, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0xf755f000
; mmap(NULL, 360267776, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0xe1dcb000
; munmap(0xe1dcb000, 360267776)           = 0
; write(1, "test 1 passed\n", 14test 1 passed
; )         = 14
; mmap(NULL, 3173556224, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0x3a2d5000 ;
; munmap(0x3a2d5000, 3173556224)          = 0
; write(1, "test 2 passed\n", 14test 2 passed
; )         = 14
; mmap(NULL, 2052141056, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0x7d04c000
; munmap(0x7d04c000, 2052141056)          = 0
; mmap(NULL, 2052141056, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0x7d04c000
; munmap(0x7d04c000, 2052141056)          = 0
; write(1, "test 3 passed\n", 14test 3 passed
; )         = 14
; mmap(NULL, 2703646720, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0x562f9000
; munmap(0x562f9000, 2703646720)          = 0
; write(1, "test 4 passed\n", 14test 4 passed
; )         = 14
; mmap(NULL, 4011384832, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0x83d1000
; munmap(0x83d1000, 4011384832)           = 0
; write(1, "test 5 passed\n", 14test 5 passed
; )         = 14
; exit(0)                                 = ?
; +++ exited with 0 +++
