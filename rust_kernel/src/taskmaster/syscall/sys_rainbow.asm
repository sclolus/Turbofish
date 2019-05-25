[BITS 32]

segment .data

res: dd 0

segment .text

global _sys_rainbow
_sys_rainbow:
	push ebp
	mov ebp, esp

	pushad
	mov eax, 1
	mov ebx, 2
	mov ecx, 3
	mov edx, 4
	mov esi, 5
	mov edi, 6
	mov ebp, 7

	add eax, ebx
	add eax, ecx
	add eax, edx
	add eax, edi
	add eax, esi
	add eax, ebp
	mov dword [res], eax

	popad

	mov eax, dword [res]
	cmp eax, 28
	je .success

	mov eax, -1
	pop ebp
	ret

.success:
	xor eax, eax
	pop ebp
	ret
