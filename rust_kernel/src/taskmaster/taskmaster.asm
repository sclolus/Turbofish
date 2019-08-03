[BITS 32]

extern syscall_interrupt_handler

segment .text

;; Preemptive schedule beacon
;; Scheduler MUST be not preemptible !
;;
;; +---------+               ^ (to high memory)
;; | SS      | TSS ONLY      |
;; +---------+                    * Illustration of the kernel stack just before IRET
;; | ESP     | TSS ONLY
;; +---------+
;; | EFLAGS  |
;; +---------+
;; | CS      |
;; +---------+
;; | EIP     | <---- ESP on the first instruction -----------> IRET
;; +---------+
;; | ErrCode | In case if CPU EXCEPTION, the TSS STACK segment contains this value
;; +---------+
;; | CpuIsr  | In case if CPU EXCEPTION, set here the exception vector number
;; ----------+
;; | DS      |
;; +---------+
;; | ES      |
;; +---------+
;; | FS      |
;; +---------+
;; | GS      |
;; +---------+
;; | REGS    |
;; |    ...  |
;; |    ...  |
;; +---------+
;; | 0x0     |
;; +---------+ ---> pointer to CpuState Structure (kernel_esp)

global _isr_syscall
_isr_syscall:

%macro STORE_CONTEXT 0
	; Generate the struct CpuState on the stack :)
	push ds
	push es
	push fs
	push gs
	pushad

	; Push 0x0 for backtrace endpoint
	push dword 0

	; Assign kernel data segments
	mov ax, 0x10
	mov ds, ax
	mov es, ax
	mov fs, ax
	mov gs, ax

	; --- MUST PASS POINTER TO THAT STRUCTURE ---
	push esp
	mov ebp, esp                ; set the backtrace endpoint
%endmacro

%macro LOAD_CONTEXT 0
	add esp, 4                  ; skip stack reserved field

	; Recover all purpose registers
	popad
	pop gs
	pop fs
	pop es
	pop ds
%endmacro

	sub esp, 8 ; skip err code & cpu isr fields
	STORE_CONTEXT

	call syscall_interrupt_handler
	add esp, 4

	LOAD_CONTEXT
	add esp, 8 ; skip err code & cpu isr fields
	; Return contains now new registers, new eflags, new esp and new eip
	iret

global _start_process
_start_process:
	push ebp
	mov ebp, esp

	; Temporary disable interrupts
	cli

	; Follow the kernel_esp pointer (pass to breaking stack context)
	mov esp, [ebp + 8]

	LOAD_CONTEXT
	add esp, 8 ; skip err code & cpu isr fields
	; Return contains now new registers, new eflags, new esp and new eip
	iret

extern scheduler_interrupt_handler

global _schedule_next
_schedule_next:
	sub esp, 8 ; skip err code & cpu isr fields
	STORE_CONTEXT

	call scheduler_interrupt_handler
	; Set the new stack pointer
	mov esp, eax
schedule_return:

	LOAD_CONTEXT
	add esp, 8 ; skip err code & cpu isr fields
	; Return contains now new registers, new eflags, new esp and new eip
	iret

extern _preemptible

; It is identical to the above its mark system as scheduler-preemptible
; This function MUST be used only in a INTGATE context
global _schedule_force_preempt
_schedule_force_preempt:
	sub esp, 8 ; skip err code & cpu isr fields
	STORE_CONTEXT
	call _preemptible
	call scheduler_interrupt_handler
	; Set the new stack pointer
	mov esp, eax

	LOAD_CONTEXT
	add esp, 8 ; skip err code & cpu isr fields
	; Return contains now new registers, new eflags, new esp and new eip
	iret

; unsafe extern "C" fn scheduler_exit_resume(process_to_free: Pid, status: i32)
extern scheduler_exit_resume

; fn _exit_resume(new_kernel_esp: u32, process_to_free: Pid, status: i32) -> !;
global _exit_resume
_exit_resume:
	push ebp
	mov ebp, esp

	mov ebx, dword [ebp + 12]   ; get PID of process to free
	mov ecx, dword [ebp + 16]   ; get TID of process to free
	mov edx, dword [ebp + 20]   ; get return status of process to free

	; Go to the stack of the new current process
	mov esp, dword [ebp + 8]

	push edx
	push ecx
	push ebx
	; Free the ressources of the existed process
	call scheduler_exit_resume
	add esp, 12

	jmp schedule_return

global _continue_schedule
_continue_schedule:
	push ebp
	mov ebp, esp

	; Go to the stack of the new current process
	mov esp, dword [ebp + 8]

	jmp schedule_return


; https://wiki.osdev.org/Exceptions
; These CPU ISR gates are on vector 0 -> 31

extern cpu_isr_interrupt_handler

%macro CPU_ISR_WITHOUT_ERRCODE_OFFSET 2
global _cpu_isr_%2
_cpu_isr_%2:
	sub esp, 4 ; skip err_code
	push %1 ; push the isr number

	STORE_CONTEXT
	call cpu_isr_interrupt_handler
	add esp, 4

	LOAD_CONTEXT
	add esp, 8 ; skip err code & cpu isr fields
	iret
%endmacro

%macro CPU_ISR_WITH_ERRCODE_OFFSET 2
global _cpu_isr_%2:
_cpu_isr_%2:
	push %1 ; push the isr number

	STORE_CONTEXT
	call cpu_isr_interrupt_handler
	add esp, 4

	LOAD_CONTEXT
	add esp, 8 ; skip err code & cpu isr fields
	iret
%endmacro

; CPU ISR without err_code
CPU_ISR_WITHOUT_ERRCODE_OFFSET 0, divide_by_zero
CPU_ISR_WITHOUT_ERRCODE_OFFSET 1, debug
CPU_ISR_WITHOUT_ERRCODE_OFFSET 2, non_maskable_interrupt
CPU_ISR_WITHOUT_ERRCODE_OFFSET 3, breakpoint
CPU_ISR_WITHOUT_ERRCODE_OFFSET 4, overflow
CPU_ISR_WITHOUT_ERRCODE_OFFSET 5, bound_range_exceeded
CPU_ISR_WITHOUT_ERRCODE_OFFSET 6, invalid_opcode
CPU_ISR_WITHOUT_ERRCODE_OFFSET 7, no_device
CPU_ISR_WITHOUT_ERRCODE_OFFSET 9, fpu_seg_overrun
CPU_ISR_WITHOUT_ERRCODE_OFFSET 16, fpu_floating_point_exep
CPU_ISR_WITHOUT_ERRCODE_OFFSET 18, machine_check
CPU_ISR_WITHOUT_ERRCODE_OFFSET 19, simd_fpu_fp_exception
CPU_ISR_WITHOUT_ERRCODE_OFFSET 20, virtualize_exception

; CPU ISR with err_code
CPU_ISR_WITH_ERRCODE_OFFSET 8, double_fault
CPU_ISR_WITH_ERRCODE_OFFSET 10, invalid_tss
CPU_ISR_WITH_ERRCODE_OFFSET 11, seg_no_present
CPU_ISR_WITH_ERRCODE_OFFSET 12, stack_seg_fault
CPU_ISR_WITH_ERRCODE_OFFSET 13, general_protect_fault
CPU_ISR_WITH_ERRCODE_OFFSET 14, page_fault
CPU_ISR_WITH_ERRCODE_OFFSET 17, alignment_check
CPU_ISR_WITH_ERRCODE_OFFSET 30, security_exception
