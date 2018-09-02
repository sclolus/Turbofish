; Copyright (C) 2014  Arjun Sreedharan
; License: GPL version 2 or higher http://www.gnu.org/licenses/gpl.html

bits 32
section .text
        ;multiboot spec
        align 4
        dd 0x1BADB002              	;magic
        dd 0x02                    	;flags
        dd - (0x1BADB002 + 0x02)   	;checksum. m+f+c should be zero

global _start
extern kmain 				;this is defined in the c file
extern init_GDT

_start:
	cli 				;block interrupts
	mov esp, stack_space		;set stack pointer

	call init_GDT

	mov ax, 0x20
	mov ss, ax
	mov esp, 0xF0000

	call kmain
	hlt 				;halt the CPU

section .bss
resb 8192				;8KB for stack
stack_space:
