use core::sync::atomic::{AtomicBool, Ordering};
use lock_api::{GuardSend, RawMutex};

// 1. Define our raw lock type
#[derive(Debug)]
pub struct RawSpinlock(AtomicBool);

// 2. Implement RawMutex for this type
unsafe impl RawMutex for RawSpinlock {
    const INIT: RawSpinlock = RawSpinlock(AtomicBool::new(false));

    // A spinlock guard can be sent to another thread and unlocked there
    type GuardMarker = GuardSend;

    fn lock(&self) {
        if !self.try_lock() {
            panic!("dead lock");
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
pub type Spinlock<T> = lock_api::Mutex<RawSpinlock, T>;
pub type SpinlockGuard<'a, T> = lock_api::MutexGuard<'a, RawSpinlock, T>;
