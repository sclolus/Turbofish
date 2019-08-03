.intel_syntax noprefix

.set CLONE, 120

.global sys_clone
sys_clone:
	push ebp
	mov ebp, esp

	push ebx
	push ecx
	mov eax, CLONE
	mov ebx, [ebp + 8]
	mov ecx, [ebp + 12]
	int 0x80
	cmp eax, 0
	jne .continue
	cmp ebx, 0
	jne clone_child

.continue:
	pop ecx
	pop ebx

	pop ebp
	ret

clone_child:
	call continue_clone_child
