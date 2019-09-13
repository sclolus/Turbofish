//! Here is the DustMan worker. It trashes processes
use super::{_preemptible, SCHEDULER};

use core::sync::atomic::{AtomicBool, Ordering};

pub static DUSTMAN_TRIGGER: AtomicBool = AtomicBool::new(false);

extern "C" {
    fn _get_preemptible_state() -> u32;
}

fn trash_process() {
    let mut scheduler = SCHEDULER.lock();
    if let Some((pid, status)) = scheduler.on_exit_routine {
        scheduler.exit_resume(pid, status);
    } else {
        log::info!("Dustman, ready to serve !");
    }
}

pub unsafe extern "C" fn dustman_handler() {
    loop {
        trash_process();
        _preemptible();
        while DUSTMAN_TRIGGER.compare_and_swap(true, false, Ordering::Relaxed) == false {
            asm!("hlt");
        }
        // Check if we are really on a unpreemptible state
        assert_eq!(_get_preemptible_state(), 1);
        DUSTMAN_TRIGGER.store(false, Ordering::Relaxed);
    }
}
