global _user_fork
_user_fork:
	push ebp
	mov ebp, esp
	mov eax, 0x2
	int 0x80
	; TODO: save return value wich is in eax
	pop ebp
	ret

global _user_write
_user_write:
	push ebp
	mov ebp, esp
	mov eax, 0x4
	mov ebx, [ebp + 8]
	mov ecx, [ebp + 12]
	mov edx, [ebp + 16]
	int 0x80
	; TODO: save return value wich is in eax
	pop ebp
	ret

global _user_exit
_user_exit:
	push ebp
	mov ebp, esp
	mov eax, 0x1
	mov ebx, [ebp + 8]
	int 0x80
	; TODO: save return value wich is in eax
	pop ebp
	ret
