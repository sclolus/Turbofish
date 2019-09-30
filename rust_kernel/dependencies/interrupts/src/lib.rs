//! This crate provide a small brief about IRQ
#![cfg_attr(not(test), no_std)]
#![feature(asm)]
#![feature(slice_index_methods)]

#[macro_use]
pub mod macros;
pub mod idt;

use i386::Eflags;

pub use self::idt::{GateType, IdtGateEntry, Idtr, InterruptTable};

/// Enables interrupts system-wide
#[inline(always)]
pub unsafe fn enable() {
    asm!("sti" :::: "volatile");
}

/// Disable interrupts system-wide
#[inline(always)]
pub unsafe fn disable() {
    asm!("cli" :::: "volatile");
}

/// Get the current interrupts state
pub fn get_interrupts_state() -> bool {
    Eflags::get_eflags().interrupt_flag()
}

/// Restore the interrupts state
pub unsafe fn restore_interrupts_state(state: bool) {
    match state {
        true => enable(),
        false => disable(),
    }
}

#[cfg(not(test))]
/// For local use of root::macro
use crate as interrupts;

#[cfg(not(test))]
/// This function initialize the Interrupt module: The default Idtr and InterruptTable are loaded,
/// then the PIC is configured.
/// This function returns the created InterruptTable.
pub unsafe fn init<'a>(
    exceptions: [(unsafe extern "C" fn() -> !, GateType); 32],
    default_isr: unsafe extern "C" fn(),
) -> InterruptTable<'a> {
    let ret = without_interrupts!({ Idtr::default().init_idt(exceptions, default_isr) });
    ret
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
