[BITS 32]

segment .text

; trampoline code, just do a sygreturn syscall
global _trampoline
global _trampoline_len
_trampoline:
	mov eax, 200
	int 0x80
_trampoline_len:    dd $-_trampoline
