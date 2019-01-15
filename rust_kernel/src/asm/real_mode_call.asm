[BITS 32]
segment .data
	bios_interrupt dw 0			; XXX That variable must be rebased in the first no in order of 16b mode

segment .text

	; Rust calling convention i386: The first parameter is close than EIP, obviously, access is EBP + 8
	; for example, fn (x:u32. y:u32) -> u32, after pushing EBP, x is on (EBP + 8) and y is on (EBP + 12)
	; the classical return is the EAX register. Caution: Never return structure between two compiler
	; ---------
	; | ARG 2 |
	; +-------+
	; | ARG 1 |
	; +-------+
	; |  EIP  | (EIP of caller)
	; +-------+
	; |  EBP  | (After pushing EBP by first instruction of ASM function)
	; +-------+

%define ALL_REGISTERS_OFFSET 32

	; extern "C" {
	;    fn asm_real_mode_op
	;        (eax:u32, ebx:u32, ecx:u32, edx:u32, esi:u32, edi:u32, bios_int:u16) -> u32;
	; }

GLOBAL asm_real_mode_op
asm_real_mode_op:
	push ebp
	mov ebp, esp

	pushad						; TIPS: pushad modify ESP but not EBP

	mov ax, word [ebp + 32]
	mov [bios_interrupt], ax

	mov eax, [ebp + 8]
	mov ebx, [ebp + 12]
	mov ecx, [ebp + 16]
	mov edx, [ebp + 20]
	mov esi, [ebp + 24]
	mov edi, [ebp + 28]

	popad

	mov eax, 42

	pop ebp
ret

GLOBAL _get_ebp
_get_ebp:
	mov eax, ebp
	ret

GLOBAL _get_esp
_get_esp:
	mov eax, esp
	ret
