[BITS 32]

; https://wiki.osdev.org/Exceptions
; These CPU ISR gates are on vector 0 -> 31

%macro PUSH_ALL_REGISTERS_WITH_ERRCODE_OFFSET 0
	pushad
	push dword [ebp + 16] ; eflags
	push dword [ebp + 12] ; cs
	push dword [ebp + 8]  ; eip
	push ss
	push gs
	push fs
	push es
	push ds
%endmacro

%macro PUSH_ALL_REGISTERS_WITHOUT_ERRCODE_OFFSET 0
	pushad
	push dword [ebp + 12] ; eflags
	push dword [ebp + 8]  ; cs
	push dword [ebp + 4]  ; eip
	push ss
	push gs
	push fs
	push es
	push ds
%endmacro

extern cpu_panic_handler
extern _align_stack

%macro CREATE_ISR 3
segment .data
	isr_%1_str: db %2, " error", 0
segment .text
GLOBAL _isr_%1
_isr_%1:
	push ebp
	mov ebp, esp
	%3
	push isr_%1_str
	push 72
	push cpu_panic_handler
	call _align_stack
%endmacro

; After expansion of macro (for cpu_default_interrupt)
; segment .data
; isr_cpu_default_interrupt_str:
;     db "cpu default interrupt", 0
; segment .text
; GLOBAL _isr_cpu_default_interrupt
; _isr_cpu_default_interrupt:
;     push ebp
;     mov ebp, esp
; # MACRO BLOC: PUSH_ALL_REGISTERS_WITHOUT_ERRCODE_OFFSET
;     pushad                ; EAX, ECX, EDX, EBX, and ESP, EBP, ESI, EDI
;     push dword [ebp + 12] ; eflags
;     push dword [ebp + 8]  ; cs
;     push dword [ebp + 4]  ; eip
;     push ss
;     push gs
;     push fs
;     push es
;     push ds
; # END MACRO BLOC
;     push isr_cpu_default_interrupt_str
;     call panic

; CPU default interrupt without err_code
CREATE_ISR cpu_default_interrupt, "cpu default interrupt", PUSH_ALL_REGISTERS_WITHOUT_ERRCODE_OFFSET

; CPU ISR without err_code
CREATE_ISR debug, "debug", PUSH_ALL_REGISTERS_WITHOUT_ERRCODE_OFFSET
CREATE_ISR divide_by_zero, "division by zero", PUSH_ALL_REGISTERS_WITHOUT_ERRCODE_OFFSET
CREATE_ISR non_maskable_interrupt, "non maskable interrupt", PUSH_ALL_REGISTERS_WITHOUT_ERRCODE_OFFSET
CREATE_ISR breakpoint, "breakpoint", PUSH_ALL_REGISTERS_WITHOUT_ERRCODE_OFFSET
CREATE_ISR overflow, "overflow", PUSH_ALL_REGISTERS_WITHOUT_ERRCODE_OFFSET
CREATE_ISR bound_range_exceeded, "bound range exceeded", PUSH_ALL_REGISTERS_WITHOUT_ERRCODE_OFFSET
CREATE_ISR invalid_opcode, "invalid_opcode", PUSH_ALL_REGISTERS_WITHOUT_ERRCODE_OFFSET
CREATE_ISR no_device, "no device", PUSH_ALL_REGISTERS_WITHOUT_ERRCODE_OFFSET
CREATE_ISR fpu_seg_overrun, "fpu seg overrun", PUSH_ALL_REGISTERS_WITHOUT_ERRCODE_OFFSET
CREATE_ISR fpu_floating_point_exep, "fpu floating point exeption", PUSH_ALL_REGISTERS_WITHOUT_ERRCODE_OFFSET
CREATE_ISR machine_check, "machine check", PUSH_ALL_REGISTERS_WITHOUT_ERRCODE_OFFSET
CREATE_ISR simd_fpu_fp_exception, "simd fpu fp exception", PUSH_ALL_REGISTERS_WITHOUT_ERRCODE_OFFSET
CREATE_ISR virtualize_exception, "virtualize exception", PUSH_ALL_REGISTERS_WITHOUT_ERRCODE_OFFSET

; CPU ISR with err_code
CREATE_ISR page_fault, "page fault", PUSH_ALL_REGISTERS_WITH_ERRCODE_OFFSET
CREATE_ISR double_fault, "double fault", PUSH_ALL_REGISTERS_WITH_ERRCODE_OFFSET
CREATE_ISR invalid_tss, "invalid tss", PUSH_ALL_REGISTERS_WITH_ERRCODE_OFFSET
CREATE_ISR seg_no_present, "segment no present", PUSH_ALL_REGISTERS_WITH_ERRCODE_OFFSET
CREATE_ISR stack_seg_fault, "stack segment fault", PUSH_ALL_REGISTERS_WITH_ERRCODE_OFFSET
CREATE_ISR general_protect_fault, "general protection fault", PUSH_ALL_REGISTERS_WITH_ERRCODE_OFFSET
CREATE_ISR alignment_check, "alignment check", PUSH_ALL_REGISTERS_WITH_ERRCODE_OFFSET
CREATE_ISR security_exception, "security exception", PUSH_ALL_REGISTERS_WITH_ERRCODE_OFFSET
