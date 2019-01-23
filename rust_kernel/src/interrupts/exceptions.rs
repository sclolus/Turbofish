// This files contains all ISRs for the exceptions.

extern "C" {
    pub fn _isr_divide_by_zero() -> !;
    pub fn _isr_debug() -> !;
    pub fn _isr_non_maskable_interrupt() -> !;
    pub fn _isr_breakpoint() -> !;
    pub fn _isr_overflow() -> !;
    pub fn _isr_bound_range_exceeded() -> !;
    pub fn _isr_invalid_opcode() -> !;
    pub fn _isr_no_device() -> !;
    pub fn _isr_double_fault() -> !;
    pub fn _isr_fpu_seg_overrun() -> !;
    pub fn _isr_invalid_tss() -> !;
    pub fn _isr_seg_no_present() -> !;
    pub fn _isr_stack_seg_fault() -> !;
    pub fn _isr_general_protect_fault() -> !;
    pub fn _isr_page_fault() -> !;
    // no.15 reserved
    pub fn _isr_fpu_floating_point_exep() -> !;
    pub fn _isr_alignment_check() -> !;
    pub fn _isr_machine_check() -> !;
    pub fn _isr_simd_fpu_fp_exception() -> !;
    pub fn _isr_virtualize_exception() -> !;
    // 21-29 reserved
    pub fn _isr_security_exception() -> !;
// 31 reserved
}

pub extern "C" fn reserved_exception() -> ! {
    loop {
        println!("This is a reserved exception");
    }
}
