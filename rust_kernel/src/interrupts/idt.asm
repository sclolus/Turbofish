	%define IDT_LENGTH 0x100  ;; 256
	%define IDT_BASE 0x2000 ;; no Idea what I'm doing
	%define IDTR(len, base) ((base << 16) | len)
	%define TEST_IDTR IDTR(IDT_LENGTH, IDT_BASE)


	segment .data
idt:
	dw 1028
	dd 0x0

	global asm_load_idtr

asm_load_idtr:
	;; mov edi, TEST_IDTR
	push ebp
	mov ebp, esp
	mov eax, [dword ebp + 8]
	lidt [eax]
	sidt [eax] 					;eventually remove this.
	mov eax, [dword idt + 4]
	mov edx, [dword idt + 4]
	pop ebp
	ret


asm_get_idtr:
	push ebp
	mov ebp, esp
	mov eax, [dword ebp + 8]
	sidt [eax]
	mov eax, [dword idt + 4]
	mov edx, [dword idt + 4]
	pop ebp
	ret

	global asm_int
asm_int:
	push ebp
	mov ebp, esp
	int 0x0
	pop ebp
	ret
