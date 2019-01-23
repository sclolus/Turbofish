	[BITS 32]

	;; This file contains all asm code regarding the interrupt service routines
	;; For now. just a generic ISR wrapper


	global generic_asm_isr_wrapper
	global div_by_zero_test

generic_asm_isr_wrapper:
	pushad

	;; ret 						; crash test
	popad
	iret
