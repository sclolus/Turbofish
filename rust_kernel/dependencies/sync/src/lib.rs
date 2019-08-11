#![cfg_attr(not(test), no_std)]
extern crate alloc;
pub mod spinlock;
pub use spinlock::{Spinlock, SpinlockGuard};
pub mod dead_mutex;
pub use dead_mutex::{DeadMutex, DeadMutexGuard};
pub mod lock_forest;
pub use lock_forest::LockForest;
