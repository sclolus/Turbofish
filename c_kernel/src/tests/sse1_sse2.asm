
[BITS 32]
section .data
align 16

v1:	    dd 1.1, 2.2, 3.3, 4.4	; Four Single precision floats 32 bits each
v1dp:	dq 1.1, 2.2		        ; Two Double precision floats 64 bits each
v2:	    dd 5.5, 6.6, 7.7, 8.8
v2s1:	dd 5.5, 6.6, 7.7, -8.8
v2s2:	dd 5.5, 6.6, -7.7, -8.8
v2s3:	dd 5.5, -6.6, -7.7, -8.8
v2s4:	dd -5.5, -6.6, -7.7, -8.8
num1:	dd 1.2
v3:	    dd 1.2, 2.3, 4.5, 6.7	; No longer 16 byte aligned
v3dp:	dq 1.2, 2.3		        ; No longer 16 byte aligned

section .bss
	mask1:	resd 1
	mask2:	resd 1
	mask3:	resd 1
	mask4:	resd 1

section .text

global _sse1_sse2_test
_sse1_sse2_test:
	push ebp
	mov ebp, esp

	push eax
	push eax
	push eax
	push eax
;	op	dst,  src

; SSE TEST

; Using movaps since vectors are 16 byte aligned
	movaps	xmm0, [v1]	; Move four 32-bit(single precision) floats to xmm0
	movaps	xmm1, [v2]
	movups	xmm2, [v3]	; Need to use movups since v3 is not 16 byte aligned
	;movaps	xmm3, [v3]	; This would seg fault if uncommented

	movups [ebp - 39], xmm3

	movaps xmm3, [ebp + 8] 		; The STACK must be also aligned
	movaps xmm3, [ebp + 8]
	movaps xmm3, [ebp + 24]
	;	movaps xmm3, [ebp + 20] crah because the stack pointer in not aligned here!

	movss	xmm3, [num1]	; Move 32-bit float num1 to the least significant element of xmm3
	movss	xmm3, [v3]	; Move first 32-bit float of v3 to the least significant element of xmm3
	movlps	xmm4, [v3]	; Move 64-bits(two single precision floats) from memory to the lower 64-bit elements of xmm4
	movhps	xmm4, [v2]	; Move 64-bits(two single precision floats) from memory to the higher 64-bit elements of xmm4
	; Source and destination for movhlps and movlhps must be xmm registers
	movhlps	xmm5, xmm4	; Transfers the higher 64-bits of the source xmm4 to the lower 64-bits of the destination xmm5
	movlhps	xmm5, xmm4	; Transfers the lower 64-bits of the source xmm4 to the higher 64-bits of the destination xmm5
	movaps	xmm6, [v2s1]
	movmskps eax, xmm6	; Extract the sign bits from four 32-bits floats in xmm6 and create 4 bit mask in eax
	mov	[mask1], eax	; Should be 8
	movaps	xmm6, [v2s2]
	movmskps eax, xmm6	; Extract the sign bits from four 32-bits floats in xmm6 and create 4 bit mask in eax
	mov	[mask2], eax	; Should be 12
	movaps	xmm6, [v2s3]
	movmskps eax, xmm6	; Extract the sign bits from four 32-bits floats in xmm6 and create 4 bit mask in eax
	mov	[mask3], eax	; Should be 14
	movaps	xmm6, [v2s4]
	movmskps eax, xmm6	; Extract the sign bits from four 32-bits floats in xmm6 and create 4 bit mask in eax
	mov	[mask4], eax	; Should be 15

; SSE2 TEST

	movapd	xmm6, [v1dp]	; Move two 64-bit(double precision) floats to xmm6, using movapd since vector is 16 byte aligned
	; Next two instruction should have equivalent results to movapd xmm6, [vldp]
	movhpd	xmm6, [v1dp+8]	; Move a 64-bit(double precision) float into the higher 64-bit elements of xmm6
	movlpd	xmm6, [v1dp]	; Move a 64-bit(double precision) float into the lower 64-bit elements of xmm6
	movupd	xmm6, [v3dp]	; Move two 64-bit floats to xmm6, using movupd since vector is not 16 byte aligned

	add esp, 16

	xor eax, eax

	mov eax, [ebp + 8]
	mov ecx, [ebp + 12]
	sub eax, ecx

	pop ebp
	ret
