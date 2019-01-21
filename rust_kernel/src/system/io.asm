	[BITS 32]

	segment .text

	global asm_inb
	global asm_outb

asm_inb:
	xor eax, eax
	xor edx, edx

	mov	dx, [dword esp + 4]
	in	al, dx
	ret

asm_outb:
	xor eax, eax
	xor edx, edx

	mov dx, [dword esp + 8]
	mov al, [byte esp + 4]
	out dx, al
	ret
