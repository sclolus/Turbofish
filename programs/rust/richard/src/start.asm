[BITS 32]

; Calling convention: Platform System V i386
; ------------------------------------------
; Return Value | Parameter Registers | Additional Parameters | Stack Alignment
; eax, edx       none                  stack (right to left)   none
;
; Scratch Registers | Preserved Registers       | Call List
; eax, ecx, edx       ebx, esi, edi, ebp, esp     ebp

extern rustmain

segment .text

global _start ; must be declared for linker (ld)
_start:       ; tell linker entry point
	push ebp
	mov ebp, esp

	call rustmain

	push 0
	call user_exit

global user_write
user_write:
	push ebp
	mov ebp, esp

	push ebx

	mov edx, [ebp + 16]
	mov ecx, [ebp + 12]
	mov ebx, [ebp + 8]

	mov eax, 4 ; system call number (sys_write)
	int 80h

	pop ebx

	pop ebp
	ret

user_exit:
	push ebp
	mov ebp, esp

	mov ebx, [ebp + 8]

	mov eax, 1 ; system call number (sys_exit)
	int 80h


; --- STRACE DUMP ---
; execve("./richard", ["./richard"], [/* 39 vars */]) = 0
; strace: [ Process PID=16439 runs in 32 bit mode. ]
; write(1, "I never used GNU/LINUX distribut"..., 36I never used GNU/LINUX distribution.) = 36
; exit(0)                                 = ?
; +++ exited with 0 +++


; Richard Matthew Stallman (/ˈstɔːlmən/ ; born March 16, 1953), often known by his initials, rms,[1]
; (also his email ID), and occasionally upper-case RMS, is an American free software movement activist
; and programmer. He campaigns for software to be distributed in a manner such that its users receive
; the freedoms to use, study, distribute, and modify that software. Software that ensures these
; freedoms is termed free software. Stallman launched the GNU Project, founded the Free Software
; Foundation, developed the GNU Compiler Collection and GNU Emacs, and wrote the GNU General Public License.
