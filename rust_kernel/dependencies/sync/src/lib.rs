#![cfg_attr(not(test), no_std)]
pub mod spinlock;
pub use spinlock::{Spinlock, SpinlockGuard};
pub mod dead_mutex;
pub use dead_mutex::{DeadMutex, DeadMutexGuard};
