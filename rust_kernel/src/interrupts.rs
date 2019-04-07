#[macro_use]
pub mod macros;
pub mod idt;

pub use self::idt::{Idtr, InterruptTable};

pub mod interrupt_manager;
// pub use interrupt_manager::{InterruptHandler, InterruptManager, Manager};

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
    use crate::registers::Eflags;

    Eflags::get_eflags().interrupt_flag()
}

/// Restore the interrupts state
pub unsafe fn restore_interrupts_state(state: bool) {
    match state {
        true => enable(),
        false => disable(),
    }
}

/// This function initialize the Interrupt module: The default Idtr and InterruptTable are loaded,
/// then the PIC is configured.
/// This function returns the created InterruptTable.
pub unsafe fn init<'a>() -> InterruptTable<'a> {
    let idt = without_interrupts!({ Idtr::default().init_idt() });
    idt
}
