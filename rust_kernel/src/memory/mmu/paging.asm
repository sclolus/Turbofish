;; This file contains primitives related to the paging. Such as enabling paging or loading the page_directory
[BITS 32]

section .text
global _enable_paging_with_cr
;; global _enable_paging
;; global _disable_paging

;; It loads the argument as the page directory pointer in cr3,
;; then actives paging.
;; Takes a pointer to the page directory as argument
_enable_paging_with_cr:
	push ebp
	mov ebp, esp

	mov eax, [ebp + 8]
	mov cr3, eax

	mov eax, cr0
	or eax, 0x80000001
	mov cr0, eax
	leave
	ret

_enable_paging:
	push ebp
	mov ebp, esp

	mov eax, cr0
	or eax, 0x80000001
	mov cr0, eax
	leave
	ret

_disable_paging:
	push ebp
	mov ebp, esp

	mov eax, cr0
	and eax, 0x7fffffff
	mov cr0, eax

	leave
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
