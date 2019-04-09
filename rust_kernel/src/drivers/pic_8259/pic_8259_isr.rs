//! See [ISR](https://wiki.osdev.org/ISR)
use crate::ffi::{c_char, strlen};

extern "C" {
    pub(super) fn _isr_timer();
    pub(super) fn _isr_keyboard();
    pub(super) fn _isr_cascade();
    pub(super) fn _isr_com2();
    pub(super) fn _isr_com1();
    pub(super) fn _isr_lpt2();
    pub(super) fn _isr_floppy_disk();
    pub(super) fn _isr_lpt1();
    pub(super) fn _isr_cmos();
    pub(super) fn _isr_acpi();
    pub(super) fn _isr_ps2_mouse();
    pub(super) fn _isr_fpu_coproc();
    pub(super) fn _isr_primary_hard_disk();
    pub(super) fn _isr_secondary_hard_disk();
}

/// For now, this is assigned as the handler for every interrupt that are not exceptions
/// Specifically handling the case for the keyboard, just for testing that it's working.
#[no_mangle]
extern "C" fn generic_interrupt_handler(interrupt_name: *const u8) {
    println!("in interrupt context");
    let slice: &[u8] = unsafe { core::slice::from_raw_parts(interrupt_name, strlen(interrupt_name as *const c_char)) };
    println!("From interrupt: {}", unsafe { core::str::from_utf8_unchecked(slice) });
}

/// This is the handler set to the reserved Gate Entries.
/// Panics when called.
#[no_mangle]
pub(super) extern "C" fn reserved_interruption() {
    panic!("Reserved interruption raised");
}
