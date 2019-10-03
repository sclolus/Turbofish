.intel_syntax noprefix

.section .init
.global _init
.type _init, @function
_init:
	push ebp
	mov ebp, esp

	push [ebp + 16] # envp
	push [ebp + 12] # argv
	push [ebp + 8]  # argc

# Initialize the 8087 FPU
	finit

	call basic_constructor
	add esp, 12
	call call_init_array_ctors

	# gcc will nicely put the contents of crtbegin.o\'s .init section here.

.section .fini
.global _fini
_fini:
	push ebp
	mov ebp, esp
	# gcc will nicely put the contents of crtbegin.o\'s .fini section here.
