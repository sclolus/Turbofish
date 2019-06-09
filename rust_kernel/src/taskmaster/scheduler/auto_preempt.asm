[BITS 32]

; fn _auto_preempt() -> i32
global _auto_preempt
_auto_preempt:
	int 81h
	; eax is returned as result
	ret
