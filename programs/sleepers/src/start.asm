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

; execve("./sleepers", ["./sleepers"], [/* 39 vars */]) = 0
; strace: [ Process PID=4472 runs in 32 bit mode. ]
; write(1, "initialise Sleeper's Sodo-test\n", 31initialise Sleeper's Sodo-test
; ) = 31
; write(1, "I will on sleeping for ", 23I will on sleeping for ) = 23
; write(1, "4", 14)                        = 1
; write(1, " seconds\n", 9 seconds
; )               = 9
; nanosleep({tv_sec=4, tv_nsec=0}, 0xfff401cc) = 0
; write(1, "Now, il attempt to sleept with n"..., 37Now, il attempt to sleept with nano.
; ) = 37
; write(1, "I will on nano sleeping , my tim"..., 44I will on nano sleeping , my time struct is ) = 44
; write(1, "Timespec", 8Timespec)                 = 8
; write(1, " {", 2 {)                       = 2
; write(1, "\n", 1
; )                       = 1
; write(1, "    ", 4    )                     = 4
; write(1, "seconds", 7seconds)                  = 7
; write(1, ": ", 2: )                       = 2
; write(1, "1", 11)                        = 1
; write(1, ",", 1,)                        = 1
; write(1, "\n", 1
; )                       = 1
; write(1, "    ", 4    )                     = 4
; write(1, "nanoseconds", 11nanoseconds)             = 11
; write(1, ": ", 2: )                       = 2
; write(1, "950000000", 9950000000)                = 9
; write(1, "\n}", 2
; })                      = 2
; write(1, "\n", 1
; )                       = 1
; nanosleep({tv_sec=1, tv_nsec=950000000}, 0xfff40248) = 0
; write(1, "I will on sleeping for ", 23I will on sleeping for ) = 23
; write(1, "1", 11)                        = 1
; write(1, " seconds\n", 9 seconds
; )               = 9
; nanosleep({tv_sec=1, tv_nsec=0}, q0xfff401cc) = 0
; write(1, "Now, il attempt to sleept with n"..., 37Now, il attempt to sleept with nano.
; ) = 37
; write(1, "I will on nano sleeping , my tim"..., 44I will on nano sleeping , my time struct is ) = 44
; write(1, "Timespec", 8Timespec)                 = 8
; write(1, " {", 2 {)                       = 2
; write(1, "\n", 1
; TO BE CONTINUED ...
