[BITS 32]

segment .text
align 16

global _idle_process_code
global _idle_process_len

_idle_process_code:
	hlt
	jmp _idle_process_code

_idle_process_len:    dd $-_idle_process_code
