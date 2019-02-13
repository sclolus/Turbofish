[BITS 32]

segment .rodata
align 16

global _wanggle_bmp_start
global _wanggle_bmp_end
global _wanggle_bmp_size

_wanggle_bmp_start:   incbin "medias/wanggle.bmp"
_wanggle_bmp_end:
_wanggle_bmp_size:    dd $-_wanggle_bmp_start
