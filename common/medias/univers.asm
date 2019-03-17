[BITS 32]

segment .rodata
align 16

global _univers_bmp_start
global _univers_bmp_end
global _univers_bmp_size

_univers_bmp_start:   incbin "medias/univers.bmp"
_univers_bmp_end:
_univers_bmp_size:    dd $-_univers_bmp_start
