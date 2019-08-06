.intel_syntax noprefix

.section .text

.global _start
_start:
# Set up end of the stack frame linked list.
	push ebp
	mov ebp, esp

	push ecx # envp
	push ebx # argv
	push eax # argc

	# Prepare signals, memory allocation, stdio and such.
	# call initialize_standard_library

	# Run the global constructors.
	call _init

	call main
	add esp, 12

	# Save the return value of the main function
	push eax

	# Run the global destructors.
	call _fini

	# Terminate the process with the exit code.
	call exit

.size _start, . - _start
