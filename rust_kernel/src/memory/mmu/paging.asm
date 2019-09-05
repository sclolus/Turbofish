;; This file contains primitives related to the paging. Such as enabling paging or loading the page_directory
[BITS 32]

section .text

global _enable_paging

global _read_cr2
global _read_cr3

global _invlpg
global _invlpg_range

global _enable_page_global

%define PAGE_SIZE 4096

;; set the Page Global Enabled bit in cr4
;; prevents the TLB from updating the address in its cache if CR3 is reset
_enable_page_global:
	mov eax, cr4
	or eax, 1 << 7
	mov cr4, eax
	ret

;; It loads the argument as the page directory pointer in cr3,
;; then actives paging.
;; Takes a pointer to the page directory as argument
_enable_paging:
	push ebp
	mov ebp, esp

	mov eax, [ebp + 8]
	mov cr3, eax

	mov eax, cr0
	or eax, 0x80000001
	mov cr0, eax
	leave
	ret

_read_cr3:
	push ebp
	mov ebp, esp

	mov eax, cr3

	pop ebp
	ret

_invlpg:
	push ebp
	mov ebp, esp
	mov eax, [ebp + 8]
	invlpg [eax]
	pop ebp
	ret

_invlpg_range:
	push ebp
	mov ebp, esp

	mov eax, [ebp + 8]
	mov ecx, [ebp + 12]
.repeat:
	invlpg [eax]
	add eax, PAGE_SIZE
	loop .repeat

	pop ebp
	ret

_read_cr2:
	push ebp
	mov ebp, esp

	mov eax, cr2

	pop ebp
	ret

;; ;; Enables the Page Size Extension (PSE)
;; _enable_pse:
;; 	push ebp
;; 	mov ebp, esp

;; 	;; Sets the bit 8 of cr4, which enables the Page Size Extension (PSE)
;; 	mov eax, cr4
;; 	or eax, 0x00000010
;; 	mov cr4, eax

;; 	leave
;; 	ret
