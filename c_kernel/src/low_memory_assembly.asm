[BITS 32]

extern _jump_c

segment .low_memory
GLOBAL _jump_asm
_jump_asm:
	call _jump_c
