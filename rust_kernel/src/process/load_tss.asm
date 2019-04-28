global load_tss

load_tss:	
	push ebp
	mov ebp, esp
	mov ax, [ebp + 8]
	ltr ax
	pop ebp
	ret
