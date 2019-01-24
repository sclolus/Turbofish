	[BITS 32]

	segment .text

	global asm_inb
	global asm_inw
	global asm_inl
	
	global asm_outb
	global asm_outw
	global asm_outl
	
	global asm_io_wait

asm_inb:
	xor eax, eax
	xor edx, edx

	mov	dx, [dword esp + 4]
	in	al, dx
	ret

asm_inw:
	xor eax, eax
	xor edx, edx

	mov	dx, [dword esp + 4]
	in	ax, dx
	ret

asm_inl:
	xor eax, eax
	xor edx, edx

	mov	dx, [dword esp + 4]
	in	eax, dx
	ret

asm_outb:
	xor eax, eax
	xor edx, edx

	mov dx, [dword esp + 8]
	mov al, [byte esp + 4]
	out dx, al
	ret
	
asm_outw:
	xor eax, eax
	xor edx, edx

	mov dx, [dword esp + 8]
	mov ax, [dword esp + 4]
	out dx, ax
	ret

asm_outl:
	xor eax, eax
	xor edx, edx

	mov dx, [dword esp + 8]
	mov eax, [byte esp + 4]
	out dx, eax
	ret

	;; Wait one io cycle by outb'ing at unused port (Needs a way to ensure it is unused)
asm_io_wait:
	out 0x80, al
	ret
