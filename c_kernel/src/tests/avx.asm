segment .text
GLOBAL _avx_test
_avx_test:
	push ebp
	mov ebp, esp
	vgatherdpd xmm1, qword [ebp+xmm7*2+0x0],xmm2
	vgatherqpd xmm1, qword [ebp+xmm7*2+0x0],xmm2
	vgatherdpd ymm1, qword [ebp+xmm7*2+0x0],ymm2
	vgatherqpd ymm1, qword [ebp+ymm7*2+0x0],ymm2
	vgatherdpd ymm6, qword [xmm4*1+0x8],ymm5
	vgatherdpd ymm6, qword [xmm4*1-0x8],ymm5
	vgatherdpd ymm6, qword [xmm4*1+0x0],ymm5
	vgatherdpd ymm6, qword [xmm4*1+0x298],ymm5
	vgatherdpd ymm6, qword [xmm4*8+0x8],ymm5
	vgatherdpd ymm6, qword [xmm4*8-0x8],ymm5
	vgatherdpd ymm6, qword [xmm4*8+0x0],ymm5
	vgatherdpd ymm6, qword [xmm4*8+0x298],ymm5
	vgatherdps xmm1, dword [ebp+xmm7*2+0x0],xmm2
	vgatherqps xmm1, dword [ebp+xmm7*2+0x0],xmm2
	vgatherdps ymm1, dword [ebp+ymm7*2+0x0],ymm2
	vgatherqps xmm1, dword [ebp+ymm7*2+0x0],xmm2
	vgatherdps xmm6, dword [xmm4*1+0x8],xmm5
	vgatherdps xmm6, dword [xmm4*1-0x8],xmm5
	vgatherdps xmm6, dword [xmm4*1+0x0],xmm5
	vgatherdps xmm6, dword [xmm4*1+0x298],xmm5
	vgatherdps xmm6, dword [xmm4*8+0x8],xmm5
	vgatherdps xmm6, dword [xmm4*8-0x8],xmm5
	vgatherdps xmm6, dword [xmm4*8+0x0],xmm5
	vgatherdps xmm6, dword [xmm4*8+0x298],xmm5
	vpgatherdd xmm1, dword [ebp+xmm7*2+0x0],xmm2
	vpgatherqd xmm1, dword [ebp+xmm7*2+0x0],xmm2
	vpgatherdd ymm1, dword [ebp+ymm7*2+0x0],ymm2
	vpgatherqd xmm1, dword [ebp+ymm7*2+0x0],xmm2
	vpgatherdd xmm6, dword [xmm4*1+0x8],xmm5
	vpgatherdd xmm6, dword [xmm4*1-0x8],xmm5
	vpgatherdd xmm6, dword [xmm4*1+0x0],xmm5
	vpgatherdd xmm6, dword [xmm4*1+0x298],xmm5
	vpgatherdd xmm6, dword [xmm4*8+0x8],xmm5
	vpgatherdd xmm6, dword [xmm4*8-0x8],xmm5
	vpgatherdd xmm6, dword [xmm4*8+0x0],xmm5
	vpgatherdd xmm6, dword [xmm4*8+0x298],xmm5
	vpgatherdq xmm1, qword [ebp+xmm7*2+0x0],xmm2
	vpgatherqq xmm1, qword [ebp+xmm7*2+0x0],xmm2
	vpgatherdq ymm1, qword [ebp+xmm7*2+0x0],ymm2
	vpgatherqq ymm1, qword [ebp+ymm7*2+0x0],ymm2
	vpgatherdq ymm6, qword [xmm4*1+0x8],ymm5
	vpgatherdq ymm6, qword [xmm4*1-0x8],ymm5
	vpgatherdq ymm6, qword [xmm4*1+0x0],ymm5
	vpgatherdq ymm6, qword [xmm4*1+0x298],ymm5
	vpgatherdq ymm6, qword [xmm4*8+0x8],ymm5
	vpgatherdq ymm6, qword [xmm4*8-0x8],ymm5
	vpgatherdq ymm6, qword [xmm4*8+0x0],ymm5
	vpgatherdq ymm6, qword [xmm4*8+0x298],ymm5
	pop ebp
	xor eax, eax
	ret
