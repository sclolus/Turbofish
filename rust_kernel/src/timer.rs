use crate::interrupts::pic_8259;
use core::sync::atomic::{AtomicU32, Ordering};

/// Time in unit of pit period
#[no_mangle]
pub static mut TIME: AtomicU32 = AtomicU32::new(0);

#[no_mangle]
extern "C" fn timer_interrupt_handler(_interrupt_name: *const u8) {
    unsafe {
        TIME.fetch_add(1, Ordering::SeqCst);
    }
    pic_8259::send_eoi(1);
}
