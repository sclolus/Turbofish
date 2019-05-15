[BITS 32]

segment .text
align 16

global _dummy_process_code
global _dummy_process_len

_dummy_process_code:
	times 16 db 0x42
	times 8 db 0x16

_dummy_process_len:    dd $-_dummy_process_code
