/// This macro gets the current interrupts state before executing arbitrary code,
/// The interrupts are disabled inside this macro.
/// it then restores the interrupt state. It helps limiting the boilerplate required to preserve the interrupts.
#[macro_export]
macro_rules! uninterruptible_context {
    ($code: block) => {{
        use crate::taskmaster::scheduler::{interruptible, uninterruptible};

        uninterruptible();

        let _ret = { $code };

        interruptible();
        _ret
    }};
}
