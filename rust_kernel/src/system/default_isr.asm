[BITS 32]

;; This file contains the default Interrupt Service routine.

;; default ISR for all IDT entries
global _default_isr
_default_isr:
	iret
