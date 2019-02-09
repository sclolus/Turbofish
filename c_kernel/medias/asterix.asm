[BITS 32]

segment .rodata
align 16

global _asterix_bmp_start
global _asterix_bmp_end
global _asterix_bmp_size

_asterix_bmp_start:   incbin "medias/asterix.bmp"
_asterix_bmp_end:
_asterix_bmp_size:    dd $-_asterix_bmp_start
