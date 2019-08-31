.intel_syntax noprefix

.section .init

	push [ebp + 16] # envp
	push [ebp + 12] # argv
	push [ebp + 8]  # argc

# Initialize the 8087 FPU
	finit

	call basic_constructor
	add esp, 12

.section .fini

	call basic_destructor
