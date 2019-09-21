/// This macro gets the current interrupts state before executing arbitrary code,
/// The interrupts are disabled inside this macro.
/// it then restores the interrupt state. It helps limiting the boilerplate required to preserve the interrupts.
#[macro_export]
macro_rules! without_interrupts {
    ($code: block) => {{
        use interrupts::{disable, enable, get_interrupts_state};

        let interrupts_state = get_interrupts_state();
        if interrupts_state == true {
            disable();
        }

        let ret = { $code };

        if interrupts_state == true {
            enable();
        }
        ret
    }};
}

/// This macro gets the current interrupts state before executing arbitrary code,
/// it then restores the interrupt state. It helps limiting the boilerplate required to preserve the interrupts.
#[macro_export]
macro_rules! preserve_interrupts {
    ($code: block) => {{
        use interrupts::{get_interrupts_state, restore_interrupts_state};

        let interrupts_state = get_interrupts_state();
        let ret = { $code };

        restore_interrupts_state(interrupts_state);
        ret
    }};
}
