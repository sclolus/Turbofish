[BITS 32]

extern virtual_offset

segment .bootstrap.text

extern alt_bootstrap_main

GLOBAL bootstrap_begin
bootstrap_begin:
    jmp alt_bootstrap_main

extern _init

GLOBAL alt_bootstrap_end
alt_bootstrap_end:
    lea eax, [_init]
    sub eax, dword virtual_offset

    jmp eax

segment .bootstrap.data
