#[macro_use]
pub mod macros;
pub mod idt;
pub mod pic_8259;
pub mod pit;

pub use self::idt::Idtr;
pub use self::pic_8259::PIC_8259;

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

    unsafe { Eflags::get_eflags().interrupt_flag() }
}

/// Restore the interrupts state
pub unsafe fn restore_interrupts_state(state: bool) {
    match state {
        true => enable(),
        false => disable(),
    }
}
