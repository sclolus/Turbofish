[BITS 32]

; Implementation of align stack without arguments
; the ESP address before calling function witch requested stack alignment must be terminate by 0x0.
; If ESP is not aligned. check the offset by mod 16 operation and sub the result to esp.
; Keep in mind that ESP come from high to low !
; esp % 16 = 0  -> nothing to do (here sub esp by 0)
; esp % 16 = 12 -> sub esp by 12
; esp % 16 = 8  -> sub esp by 8
; esp % 16 = 4  -> sub esp by 4
; after setting esp, call the function pointer (first function argument)

; extern u32 _align_stack_simple(u32(*f)());

global _align_stack_simple
segment .text
_align_stack_simple:
	push ebp
	mov ebp, esp

	push ebx

	mov ebx, esp
	and ebx, 0xf

	sub esp, ebx

	call [ebp + 8]

	add esp, ebx

	pop ebx

	pop ebp
	ret

; Implementation of align stack with a list of arguments
; args_len must be expressed as byte size surrounded by 4 (u8. u16, u32 count for 4) OR Sizeof(struct ...)
; the difference between that function and _align_stack_simple is that we tell the arguments size

; extern u32 _align_stack(u32(*f)(), u32 args_len, ...);
global _align_stack
segment .text
_align_stack:
	push ebp
	mov ebp, esp

	push ebx
	push esi
	push edi

	; get the arguments size
	mov ecx, [ebp + 12]

	; check ESP alignment, as _align_stack_simple, but also consider output arguments length
	mov ebx, esp
	add ebx, ecx
	and ebx, 0xf

	; decrement the ESP pointer by offset and output args length
	add ebx, ecx
	sub esp, ebx

	; set the source pointer to input arguments
	mov esi, ebp
	add esi, 16

	; set the destination pointer to ouput arguments
	mov edi, esp

	; copy IN stack to OUT stack
	shr ecx, 2
	cld
	rep movsd

	; call the function pointed by the first argument
	call [ebp + 8]

	; recovery of start stack offset
	add esp, ebx

	pop edi
	pop esi
	pop ebx

	pop ebp
	ret
