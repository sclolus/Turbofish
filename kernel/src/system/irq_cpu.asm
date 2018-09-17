[BITS 32]

; Some documentation are in
; https://wiki.osdev.org/Exceptions#General_Protection_Fault
segment .data

panic_buf: times 512 db 0

cpu_default_interrupt_msg: db "Not defined", 0
divide_by_zero_msg: db "Divide by zero", 0
debug_msg: db "Debug", 0
non_maskable_interrupt_msg: db "Non maskable interrupt", 0
breakpoint_msg: db "Breakpoint", 0
overflow_msg: db "Overflow", 0
bound_range_exceeded_msg: db "bound range exceeded", 0
invalid_opcode_msg: db "Invalid opcode", 0
no_device_msg: db "No device founded", 0
double_fault_msg: db "Double Fault !", 0
fpu_seg_overrun_msg: db "FPU segment Overrun", 0
invalid_tss_msg: db "Invalid TSS", 0
seg_no_present_msg: db "Segment no present", 0
stack_seg_fault_msg: db "Stack segment fault", 0
general_protect_fault_msg: db "GENERAL PROTECTION FAULT", 0
page_fault_msg: db "Page fault at address %p err_reg: 0x%.8x", 0
fpu_floating_point_exep_msg: db "FPU floating point exception", 0
alignment_check_msg: db "Alignment check", 0
machine_check_msg: db "Machine check", 0
simd_fpu_fp_exception_msg: db "SIMD FPU floating point exception", 0
virtualize_exception_msg: db "Virtualize exception", 0
security_exception_msg: db "Security exception", 0

segment .text
extern panic
extern sprintk

GLOBAL asm_cpu_default_interrupt

GLOBAL asm_divide_by_zero
GLOBAL asm_debug
GLOBAL asm_non_maskable_interrupt
GLOBAL asm_breakpoint
GLOBAL asm_overflow
GLOBAL asm_bound_range_exceeded
GLOBAL asm_invalid_opcode
GLOBAL asm_no_device
GLOBAL asm_double_fault
GLOBAL asm_fpu_seg_overrun
GLOBAL asm_invalid_tss
GLOBAL asm_seg_no_present
GLOBAL asm_stack_seg_fault
GLOBAL asm_general_protect_fault
GLOBAL asm_page_fault
GLOBAL asm_fpu_floating_point_exep
GLOBAL asm_alignment_check
GLOBAL asm_machine_check
GLOBAL asm_simd_fpu_fp_exception
GLOBAL asm_virtualize_exception
GLOBAL asm_security_exception

%macro PUSH_ALL_REGISTERS_WITH_ERRCODE_OFFSET 0
    pushad                ; EAX, ECX, EDX, EBX, and ESP, EBP, ESI, EDI
    push dword [ebp + 16] ; eflags
    push dword [ebp + 12] ; cs
    push dword [ebp + 8]  ; eip
    push ss
    push es
    push ds
%endmacro

%macro PUSH_ALL_REGISTERS_WITHOUT_ERRCODE_OFFSET 0
    pushad                ; EAX, ECX, EDX, EBX, and ESP, EBP, ESI, EDI
    push dword [ebp + 12] ; eflags
    push dword [ebp + 8]  ; cs
    push dword [ebp + 4]  ; eip
    push ss
    push es
    push ds
%endmacro

%macro POP_ALL_REGISTERS 0
    pop ds
    pop es
    add esp, 16
    popad
%endmacro

; CPU interrupt without err_code
asm_cpu_default_interrupt:
    push ebp
    mov ebp, esp
    PUSH_ALL_REGISTERS_WITHOUT_ERRCODE_OFFSET
    push cpu_default_interrupt_msg
    call panic

asm_divide_by_zero:
    push ebp
    mov ebp, esp
    PUSH_ALL_REGISTERS_WITHOUT_ERRCODE_OFFSET
    push divide_by_zero_msg
    call panic
asm_debug:
    push ebp
    mov ebp, esp
    PUSH_ALL_REGISTERS_WITHOUT_ERRCODE_OFFSET
    push debug_msg
    call panic
asm_non_maskable_interrupt:
    push ebp
    mov ebp, esp
    PUSH_ALL_REGISTERS_WITHOUT_ERRCODE_OFFSET
    push non_maskable_interrupt_msg
    call panic
asm_breakpoint:
    push ebp
    mov ebp, esp
    PUSH_ALL_REGISTERS_WITHOUT_ERRCODE_OFFSET
    push breakpoint_msg
    call panic
asm_overflow:
    push ebp
    mov ebp, esp
    PUSH_ALL_REGISTERS_WITHOUT_ERRCODE_OFFSET
    push overflow_msg
    call panic
asm_bound_range_exceeded:
    push ebp
    mov ebp, esp
    PUSH_ALL_REGISTERS_WITHOUT_ERRCODE_OFFSET
    push bound_range_exceeded_msg
    call panic
asm_invalid_opcode:
    push ebp
    mov ebp, esp
    PUSH_ALL_REGISTERS_WITHOUT_ERRCODE_OFFSET
    push invalid_opcode_msg
    call panic
asm_no_device:
    push ebp
    mov ebp, esp
    PUSH_ALL_REGISTERS_WITHOUT_ERRCODE_OFFSET
    push no_device_msg
    call panic
asm_fpu_seg_overrun:
    push ebp
    mov ebp, esp
    PUSH_ALL_REGISTERS_WITHOUT_ERRCODE_OFFSET
    push fpu_seg_overrun_msg
    call panic
asm_fpu_floating_point_exep:
    push ebp
    mov ebp, esp
    PUSH_ALL_REGISTERS_WITHOUT_ERRCODE_OFFSET
    push fpu_floating_point_exep_msg
    call panic
asm_machine_check:
    push ebp
    mov ebp, esp
    PUSH_ALL_REGISTERS_WITHOUT_ERRCODE_OFFSET
    push machine_check_msg
    call panic
asm_simd_fpu_fp_exception:
    push ebp
    mov ebp, esp
    PUSH_ALL_REGISTERS_WITHOUT_ERRCODE_OFFSET
    push simd_fpu_fp_exception_msg
    call panic
asm_virtualize_exception:
    push ebp
    mov ebp, esp
    PUSH_ALL_REGISTERS_WITHOUT_ERRCODE_OFFSET
    push virtualize_exception_msg
    call panic

; CPU interrupt with err_code
asm_double_fault:
    push ebp
    mov ebp, esp
    PUSH_ALL_REGISTERS_WITH_ERRCODE_OFFSET
    push double_fault_msg
    call panic
asm_invalid_tss:
    push ebp
    mov ebp, esp
    PUSH_ALL_REGISTERS_WITH_ERRCODE_OFFSET
    push invalid_tss_msg
    call panic
asm_seg_no_present:
    push ebp
    mov ebp, esp
    PUSH_ALL_REGISTERS_WITH_ERRCODE_OFFSET
    push seg_no_present_msg
    call panic
asm_stack_seg_fault:
    push ebp
    mov ebp, esp
    PUSH_ALL_REGISTERS_WITH_ERRCODE_OFFSET
    push stack_seg_fault_msg
    call panic
asm_general_protect_fault:
    push ebp
    mov ebp, esp
    PUSH_ALL_REGISTERS_WITH_ERRCODE_OFFSET
    push general_protect_fault_msg
    call panic
asm_alignment_check:
    push ebp
    mov ebp, esp
    PUSH_ALL_REGISTERS_WITH_ERRCODE_OFFSET
    push alignment_check_msg
    call panic
asm_security_exception:
    push ebp
    mov ebp, esp
    PUSH_ALL_REGISTERS_WITH_ERRCODE_OFFSET
    push security_exception_msg
    call panic

; when a normal CPU interruption is launched, EFLAGS, CS and EIP are pushed.
; in the case of page_fault, an other value (err_code) is pushed after.
; see 'rec03-2.pdf' at page 11 for more explanation.
;
; to execute IRET corectly we must add esp by 4 or pop something to skip
; err_code
extern page_fault_handler
asm_page_fault:
    push ebp
    mov ebp, esp

    PUSH_ALL_REGISTERS_WITH_ERRCODE_OFFSET

; C manager execution, test if this page fault is not fatal
    mov eax, cr2
    push eax
    mov eax, [ebp + 4]
    push eax
    call page_fault_handler
    add esp, 8
    cmp eax, 0
    je .end ; if OKAY, jump to the end

; panic execution block, fill the error string and launch the BSOD
    push dword [ebp + 4]
    mov eax, cr2
    push eax
    push page_fault_msg
    push panic_buf

    call sprintk
    add esp, 16

    push panic_buf
    call panic
; the execution cannot continue here

; end segment, return to programm
.end:
    POP_ALL_REGISTERS

    pop ebp
    ; bypass the error code
    add esp, 4
    iret
