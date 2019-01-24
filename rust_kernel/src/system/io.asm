[BITS 32]
;; This files contains the primitives for I/O port operations
	
segment .text

global _inb
global _inw
global _inl
	
global _outb
global _outw
global _outl
	
global _io_wait

_inb:
	push	ebp
	mov	ebp, esp
	
	xor	eax, eax
	xor	edx, edx

	mov	dx, [dword ebp + 8]
	in	al, dx
	pop	ebp
	ret

_inw:
	push	ebp
	mov	ebp, esp

	xor	eax, eax
	xor	edx, edx

	mov	dx, [dword ebp + 8]
	in	ax, dx
	pop	ebp
	ret

_inl:
	push	ebp
	mov	ebp, esp

	xor	eax, eax
	xor	edx, edx

	mov	dx, [dword ebp + 8]
	in	eax, dx
	pop	ebp
	ret

_outb:
	push	ebp
	mov	ebp, esp

	xor	eax, eax
	xor	edx, edx

	mov	dx, [dword ebp + 12]
	mov	al, [byte ebp + 8]
	out	dx, al
	
	pop	ebp
	ret
	
_outw:
	push	ebp
	mov	ebp, esp

	xor	eax, eax
	xor	edx, edx

	mov	dx, [dword ebp + 12]
	mov	ax, [dword ebp + 8]
	out	dx, ax
	
	pop	ebp
	ret

_outl:
	push	ebp
	mov	ebp, esp

	xor	eax, eax
	xor	edx, edx

	mov	dx, [dword ebp + 12]
	mov	eax, [byte ebp + 8]
	out	dx, eax
	
	pop	ebp
	ret

;; Wait one io cycle by outb'ing at unused port (Needs a way to ensure it is unused)
_io_wait:
	push	ebp
	mov	ebp, esp
	
	out	0x80, al

	pop	ebp
	ret
