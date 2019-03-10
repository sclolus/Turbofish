[BITS 32]

segment .text

global _set_sse
_set_sse:
	push ebp
	mov ebp, esp

	pushad

	mov eax, 0x1
	cpuid
	; test if SSE2 feature exist
	test edx, 1 << 26
	jz .end

	mov eax, cr0
	; clear coprocessor emulation CR0.EM
	and ax, 0xFFFB
	; set coprocessor monitoring  CR0.MP
	or ax, 0x2
	mov cr0, eax
	mov eax, cr4
	; set CR4.OSFXSR and CR4.OSXMMEXCPT at the same time
	or eax, 3 << 9
	; Enable OSXSAVE instructions
	or eax, 1 << 18
	mov cr4, eax

.end:
	popad

	pop ebp
	ret

global _set_avx
_set_avx:
	push ebp
	mov ebp, esp

	push eax
	push ecx
	push edx

	xor ecx, ecx

	; Load XCR0 register
	xgetbv

	; Set AVX, SSE, X87 bits
	or eax, 7
	; Save back to XCR0
	xsetbv

	pop edx
	pop ecx
	pop eax

	pop ebp
	ret

global _set_fpu
_set_fpu:
	push ebp
	mov ebp, esp

	finit

	pop ebp
	ret
