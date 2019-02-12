/// This files contains all ISRs for the exceptions.
/// See https://wiki.osdev.org/Exceptions

extern "C" {
    pub(super) fn _isr_divide_by_zero() -> !;
    pub(super) fn _isr_debug() -> !;
    pub(super) fn _isr_non_maskable_interrupt() -> !;
    pub(super) fn _isr_breakpoint() -> !;
    pub(super) fn _isr_overflow() -> !;
    pub(super) fn _isr_bound_range_exceeded() -> !;
    pub(super) fn _isr_invalid_opcode() -> !;
    pub(super) fn _isr_no_device() -> !;
    pub(super) fn _isr_double_fault() -> !;
    pub(super) fn _isr_fpu_seg_overrun() -> !;
    pub(super) fn _isr_invalid_tss() -> !;
    pub(super) fn _isr_seg_no_present() -> !;
    pub(super) fn _isr_stack_seg_fault() -> !;
    pub(super) fn _isr_general_protect_fault() -> !;
    pub(super) fn _isr_page_fault() -> !;
    // no.15 reserved
    pub(super) fn _isr_fpu_floating_point_exep() -> !;
    pub(super) fn _isr_alignment_check() -> !;
    pub(super) fn _isr_machine_check() -> !;
    pub(super) fn _isr_simd_fpu_fp_exception() -> !;
    pub(super) fn _isr_virtualize_exception() -> !;
    // 21-29 reserved
    pub(super) fn _isr_security_exception() -> !;
// 31 reserved
}

pub(super) extern "C" fn reserved_exception() -> ! {
    panic!("This is a reserved exception");
}
