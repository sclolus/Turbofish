[BITS 32]

segment .bootstrap.text

extern alt_bootstrap_main

GLOBAL bootstrap_begin
bootstrap_begin:
	jmp alt_bootstrap_main

extern _init

GLOBAL alt_bootstrap_end
alt_bootstrap_end:
	lea eax, [_init]
	sub eax, 0xc0000000

	jmp eax

segment .bootstrap.data
