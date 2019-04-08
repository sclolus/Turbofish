/// This macro gets the current interrupts state before executing arbitrary code,
/// The interrupts are disabled inside this macro.
/// it then restores the interrupt state. It helps limiting the boilerplate required to preserve the interrupts.
#[macro_export]
macro_rules! without_interrupts {
    ($code: block) => {{
        use crate::interrupts::{disable, enable, get_interrupts_state};

        struct Finalizer {
            state: bool,
        }

        impl Drop for Finalizer {
            fn drop(&mut self) {
                if self.state == true {
                    unsafe { enable() }
                }
            }
        }

        let interrupts_state = get_interrupts_state();
        let _ensure = Finalizer { state: interrupts_state };

        if interrupts_state == true {
            disable();
        }

        let ret = { $code };

        ret
    }};
}

/// This macro gets the current interrupts state before executing arbitrary code,
/// it then restores the interrupt state. It helps limiting the boilerplate required to preserve the interrupts.
#[macro_export]
macro_rules! preserve_interrupts {
    ($code: block) => {{
        use crate::interrupts::{get_interrupts_state, restore_interrupts_state};

        struct Finalizer {
            state: bool,
        }

        impl Drop for Finalizer {
            fn drop(&mut self) {
                unsafe {
                    restore_interrupts_state(self.state);
                }
            }
        }

        let interrupts_state = get_interrupts_state();
        let _ensure = Finalizer { state: interrupts_state };
        let ret = { $code };

        ret
    }};
}
