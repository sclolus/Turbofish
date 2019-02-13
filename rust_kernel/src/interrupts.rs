#[macro_use]
pub mod macros;
pub mod idt;
pub mod pic_8259;
pub mod pit;

pub use self::idt::{Idtr, InterruptTable};
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

/// This function initialize the Interrupt module: The default Idtr and InterruptTable are loaded,
/// then the PIC is configured.
/// This function returns the created InterruptTable.
pub unsafe fn init<'a>() -> InterruptTable<'a> {
    let idt = without_interrupts!({
        let idt = Idtr::default().init_idt();

        PIC_8259.init();
        PIC_8259.disable_all_irqs();
        PIC_8259.enable_irq(pic_8259::Irq::KeyboardController); // enable only the keyboard.

        idt
    });
    enable();
    idt
}
