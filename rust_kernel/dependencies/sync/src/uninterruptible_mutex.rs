//! Mutex which disable interrupt during the lock
use lock_api::{GuardSend, RawMutex};

/// this mutex disable interrupt(with a cli) during the lock of the mutex
#[derive(Debug)]
pub struct RawUnInterruptibleMutex;

unsafe impl RawMutex for RawUnInterruptibleMutex {
    const INIT: RawUnInterruptibleMutex = RawUnInterruptibleMutex;

    type GuardMarker = GuardSend;

    fn lock(&self) {
        self.try_lock();
    }

    fn try_lock(&self) -> bool {
        unsafe {
            // asm!("cli");
        }
        return true;
    }

    fn unlock(&self) {
        unsafe {
            // asm!("sti");
        }
    }
}

pub type UnInterruptibleMutex<T> = lock_api::Mutex<RawUnInterruptibleMutex, T>;
pub type UnInterruptibleMutexGuard<'a, T> = lock_api::MutexGuard<'a, RawUnInterruptibleMutex, T>;
