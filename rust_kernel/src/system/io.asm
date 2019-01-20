
	segment .text

	global asm_inb

asm_inb:
	xor eax, eax
	xor edx, edx

	mov	dx, [dword esp + 4]
	in	al, dx
	ret

	global asm_outb
asm_outb:
	xor eax, eax
	xor edx, edx

	mov dx, [dword esp + 8]
	mov al, [byte esp + 4]
	out dx, al
	ret
