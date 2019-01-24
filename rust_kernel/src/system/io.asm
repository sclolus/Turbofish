[BITS 32]
;; This files contains the primitives for I/O port operations
	
segment .text

global asm_inb
global asm_inw
global asm_inl
	
global asm_outb
global asm_outw
global asm_outl
	
global asm_io_wait

asm_inb:
	push	ebp
	mv	ebp, esp
	
	xor	eax, eax
	xor	edx, edx

	mov	dx, [dword ebp + 8]
	in	al, dx
	pop	ebp
	ret

asm_inw:
	push	ebp
	mv	ebp, esp

	xor	eax, eax
	xor	edx, edx

	mov	dx, [dword ebp + 8]
	in	ax, dx
	pop	ebp
	ret

asm_inl:
	push	ebp
	mv	ebp, esp

	xor	eax, eax
	xor	edx, edx

	mov	dx, [dword ebp + 8]
	in	eax, dx
	pop	ebp
	ret

asm_outb:
	push	ebp
	mv	ebp, esp

	xor	eax, eax
	xor	edx, edx

	mov	dx, [dword ebp + 12]
	mov	al, [byte ebp + 8]
	out	dx, al
	
	pop	ebp
	ret
	
asm_outw:
	push	ebp
	mv	ebp, esp

	xor	eax, eax
	xor	edx, edx

	mov	dx, [dword ebp + 12]
	mov	ax, [dword ebp + 8]
	out	dx, ax
	
	pop	ebp
	ret

asm_outl:
	push	ebp
	mv	ebp, esp

	xor	eax, eax
	xor	edx, edx

	mov	dx, [dword ebp + 12]
	mov	eax, [byte ebp + 8]
	out	dx, eax
	
	pop	ebp
	ret

;; Wait one io cycle by outb'ing at unused port (Needs a way to ensure it is unused)
asm_io_wait:
	push	ebp
	mv	ebp, rsp
	
	out	0x80, al

	pop	ebp
	ret
