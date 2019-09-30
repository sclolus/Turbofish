//! Here is the Second Callback worker. It call process registered to each seconds events
use super::{_preemptible, SCHEDULER};

use core::sync::atomic::{AtomicBool, Ordering};

pub static SECOND_CALLBACK_TRIGGER: AtomicBool = AtomicBool::new(false);

extern "C" {
    fn _get_preemptible_state() -> u32;
}

/// Apply the second cycle routine on modules
/// The scheduler must call this function outside an INTGATE to ensure that disk IRQ or something else can happen
fn second_callback() {
    let scheduler = SCHEDULER.lock();
    for f in scheduler.kernel_modules.second_cycle.iter() {
        (f)()
    }
}

/// This function must be called in a unpremptible_context with the SECOND_CALLBACK_TRIGGER set as true
pub unsafe extern "C" fn second_callback_handler() {
    loop {
        while SECOND_CALLBACK_TRIGGER.compare_and_swap(true, false, Ordering::Relaxed) == false {
            asm!("hlt");
        }
        // Check if we are really on a unpreemptible state
        assert_eq!(_get_preemptible_state(), 1);
        SECOND_CALLBACK_TRIGGER.store(false, Ordering::Relaxed);
        second_callback();
        _preemptible();
    }
}
