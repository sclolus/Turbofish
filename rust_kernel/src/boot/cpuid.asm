[BITS 32]

segment .text

GLOBAL check_cpuid_feature
check_cpuid_feature:
	push ebp
	mov ebp, esp

	pushfd						;Store EFLAGS
	pushfd						;Store EFLAGS
	xor dword [esp],0x00200000	;Invert the ID bit in stored EFLAGS
	popfd						;Load stored EFLAGS (with ID bit inverted)
	pushfd						;Store EFLAGS again (ID bit may or may not be inverted)
	pop eax						;eax = modified EFLAGS (ID bit may or may not be inverted)
	xor eax,[esp]				;eax = whichever bits were changed
	popfd						;Restore original EFLAGS
	and eax,0x00200000			;eax = zero if ID bit can't be changed, else non-zero

	cmp eax, 0
	jne .success

.failure:
	mov eax, -1
	pop ebp
	ret
.success:
	mov eax, 0
	pop ebp
	ret

GLOBAL get_vendor_id
get_vendor_id:
	push ebp
	mov ebp, esp
	pushad

	mov eax, 0
	cpuid

	mov [vendor_id], ebx
	mov [vendor_id + 4], edx
	mov [vendor_id + 8], ecx

	popad
	mov eax, vendor_id
	pop ebp
	ret

GLOBAL get_ecx_cpufeatures
get_ecx_cpufeatures:
	push ebp
	mov ebp, esp
	pushad

	mov eax, 1
	cpuid
	mov [reg], ecx

	popad
	mov eax, [reg]
	pop ebp
	ret

GLOBAL get_edx_cpufeatures
get_edx_cpufeatures:
	push ebp
	mov ebp, esp
	pushad

	mov eax, 1
	cpuid
	mov [reg], edx

	popad
	mov eax, [reg]
	pop ebp
	ret

segment .data
vendor_id:
	times 13 db 0
reg:
	dd 0
