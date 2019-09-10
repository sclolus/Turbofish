[BITS 32]

extern syscall_interrupt_handler

segment .data

segment .text

;; Preemptive schedule beacon
;; Scheduler MUST be not preemptible !
;;
;; 0x0000 +---------+               ^ (to high memory)
;;        | SS      | TSS ONLY      |
;; 0x0004 +---------+                    * Illustration of the kernel stack just before IRET
;;        | ESP     | TSS ONLY
;; 0x0008 +---------+
;;        | EFLAGS  |
;; 0x000C +---------+
;;        | CS      |
;; 0x0010 +---------+
;;        | EIP     | <---- ESP on the first instruction -----------> IRET
;; 0x0014 +---------+
;;        | ErrCode | In case if CPU EXCEPTION, the TSS STACK segment contains this value
;; 0x0018 +---------+
;;        | CpuIsr  | In case if CPU EXCEPTION, set here the exception vector number
;; 0x001C ----------+
;;        | DS      |
;; 0x0020 +---------+
;;        | ES      |
;; 0x0024 +---------+
;;        | FS      |
;; 0x0028 +---------+
;;        | GS      |
;; 0x002C +---------+
;;        | REGS    |
;;        |    ...  |
;;        |    ...  |
;; 0x004C +---------+
;;        |(padding)|
;; 0x0050 +---------+ IMPORTANT: MUST BE ALIGNED ON 16 BYTES
;;        |  * FPU  |
;;        |  * MMX  | 512 bytes for FPU/MMX/SSE/AVX Support (80x86 only)
;;        |  * SSE  | ... (must be used only when switching from ring3)
;;        |  regs 	|
;; 0x0250 +---------+
;;        | 0x0     |
;; 0x0254 +---------+ ---> pointer to CpuState Structure (kernel_esp)

global _isr_syscall
_isr_syscall:

%macro STORE_CONTEXT 0
	; Generate the struct CpuState on the stack :)
	push ds
	push es
	push fs
	push gs
	pushad

	; Dec stack to store FxRegion
	sub esp, 0x4
	sub esp, 0x200

	; Get CS stored value to check if we went from RING3
	mov eax, dword [esp + 0x240]
	and eax, 0b11
	cmp eax, 0b11
	jne %%.skip_storing_fx_region
	; Store FPU/MMX/SSE/AVX of the current process (only from a ring3 context)
	fxsave [esp]
%%.skip_storing_fx_region:

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

	; Get CS stored value to check if we went from RING3
	mov eax, dword [esp + 0x240]
	and eax, 0b11
	cmp eax, 0b11
	jne %%.skip_restoring_fx_region
	; Restore FPU/MMX/SSE/AVX of the current process (only to a ring3 context)
	fxrstor [esp]
	%%.skip_restoring_fx_region:

	; Inc stack
	add esp, 0x200
	add esp, 0x4

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
	; Set the new stack pointer
	mov esp, eax

	LOAD_CONTEXT
	add esp, 8 ; skip err code & cpu isr fields
	; Return contains now new registers, new eflags, new esp and new eip
	iret

extern scheduler_interrupt_handler

; It is time to schedule
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

; This function if launched for the first process
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
	mov ecx, dword [ebp + 16]   ; get return status of process to free

	; Go to the stack of the new current process
	mov esp, dword [ebp + 8]

	push ecx
	push ebx
	; Free the ressources of the existed process
	call scheduler_exit_resume
	add esp, 8

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
	; Set the new stack pointer
	mov esp, eax

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
	; Set the new stack pointer
	mov esp, eax

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
