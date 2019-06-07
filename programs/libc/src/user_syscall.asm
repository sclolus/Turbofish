[BITS 32]

; Calling convention: Platform System V i386
; ------------------------------------------
; Return Value | Parameter Registers | Additional Parameters | Stack Alignment
; eax, edx       none                  stack (right to left)   none
;
; Scratch Registers | Preserved Registers       | Call List
; eax, ecx, edx       ebx, esi, edi, ebp, esp     ebp
;
; Syscall convention INT 80H. INTEL => Parameters are passed by registers. SysNum: EAX
; Parameters registers order: EBX, ECX, EDX, ESI, EDI
; Return on EAX (no carry flag feature)

; int _user_syscall(u32 syscall_number, u32 args_len, ...);

; When call int 80h, there is the Graphic of Stack relative to argument number
; +-----+ | ----+-----+ | -----+-----+ | ----------+-----+ +
; | EBP | | 5   | EBP | |  4   | EBP | | 3, 2, 1   | EBP | | 0
; +-----+ |     |-----+ |      +-----| |           +-----+ v
; | EDI | |     | ESI | |      | EBX | |
; +-----+ |     |-----+ |      +-----+ v
; | ESI | |     | EBX | |
; +-----+ |     +-----+ v
; | EBX | |
; +-----+ v

global _user_syscall
_user_syscall:
	push ebp
	mov ebp, esp

	; get the number of arguments in eax
	mov eax, [ebp + 12]

	; test if arg number is lower than 5
	cmp eax, 5
	jb .next4
	; store preserved edi
	push edi
	mov edi, [ebp + 32]

.next4:
	; test if arg number is lower than 4
	cmp eax, 4
	jb .next3
	; store preserved esi
	push esi
	mov esi, [ebp + 28]

.next3:
	; test if arg number is lower than 3
	cmp eax, 3
	jb .next2
	mov edx, [ebp + 24]

.next2:
	; test if arg number is lower than 2
	cmp eax, 2
	jb .next1
	mov ecx, [ebp + 20]

.next1:
	; test if arg number is 0 len
	cmp eax, 0
	je .sys_exec
	; store preserved ebx
	push ebx
	mov ebx, [ebp + 16]

.sys_exec:
	; get the syscall number
	mov eax, dword [ebp + 8]
	; Sys Call: the return value will be in eax
	int 80h

	; get again the number of arguments but in edx now
	mov edx, [ebp + 12]

	cmp edx, 0
	je .end

	; restore ebx
	pop ebx
	cmp edx, 3
	jbe .end

	; restore esi
	pop esi
	cmp edx, 4
	je .end

	; restore edi
	pop edi

.end:
	; function return
	pop ebp
	ret
