[BITS 32]

extern _jump_c

segment .low_memory.text
GLOBAL _jump_asm
_jump_asm:
	call _jump_c

extern _init
GLOBAL _jump_init
_jump_init:
	lea eax, [_init]
	sub eax, 0xc0000000

	jmp eax
