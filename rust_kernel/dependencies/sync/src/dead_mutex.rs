use core::sync::atomic::{AtomicBool, Ordering};
use lock_api::{GuardSend, RawMutex};

// 1. Define our raw lock type
#[derive(Debug)]
pub struct RawDeadMutex(AtomicBool);

// 2. Implement RawMutex for this type
unsafe impl RawMutex for RawDeadMutex {
    const INIT: RawDeadMutex = RawDeadMutex(AtomicBool::new(false));

    // A spinlock guard can be sent to another thread and unlocked there
    type GuardMarker = GuardSend;

    fn lock(&self) {
        if !self.try_lock() {
            panic!("dead lock from {:?}", self);
        }
    }

    fn try_lock(&self) -> bool {
        if self.0.load(Ordering::SeqCst) {
            return false;
        }
        self.0.swap(true, Ordering::SeqCst);
        return true;
    }

    fn unlock(&self) {
        self.0.store(false, Ordering::SeqCst);
    }
}

// 3. Export the wrappers. This are the types that your users will actually use.
pub type DeadMutex<T> = lock_api::Mutex<RawDeadMutex, T>;
pub type DeadMutexGuard<'a, T> = lock_api::MutexGuard<'a, RawDeadMutex, T>;
