[BITS 32]

global _rdrand
_rdrand:
; loop until the carry flag is not set
	rdrand eax
	jnc _rdrand
	ret
