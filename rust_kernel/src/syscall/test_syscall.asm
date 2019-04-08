global _write
_write:
	push ebp
	mov ebp, esp
	pushad
	mov eax, 0x4
	mov ebx, [ebp + 8]
	mov ecx, [ebp + 12]
	mov edx, [ebp + 16]
	int 0x80
	popad
	pop ebp
	ret
