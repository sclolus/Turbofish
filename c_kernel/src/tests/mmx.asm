
[BITS 32]
segment .data

a: dd 0x2a, 0x15
b: dd 0, 0

segment .text

GLOBAL _mmx_test
_mmx_test:
	push ebp
	mov ebp, esp

	push b
	push a
	call .add2_and_copy_memory8
	add esp, 8

	mov eax, dword [b]
	cmp eax, 0x54
	jne .check
	mov eax, dword [b + 4]
	cmp eax, 0x2a
	jne .check

	xor eax, eax
	pop ebp
	ret

.check:
	mov eax, -1
	pop ebp
	ret

; add twice the first operante and copy 8 bytes from src to dest by one mm register
; mnemotechnic of movq: mov quad word
; MMX is also an earler method to perform 64 bits integer operations on 32 bits machines
.add2_and_copy_memory8:
	push ebp
	mov ebp, esp

	mov eax, [ebp + 8]
	movq mm0, [eax]
	movq mm1, [eax]
	paddd mm0, mm1 				; 64 bits addition 0x0000002a00000015 * 2 => 0x000000540000002a
	mov eax, [ebp + 12]
	movq [eax], mm0
	emms

	pop ebp
	ret
