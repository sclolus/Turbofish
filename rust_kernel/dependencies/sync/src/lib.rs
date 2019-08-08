#![cfg_attr(not(test), no_std)]
pub mod spinlock;
pub use spinlock::*;
pub mod dead_mutex;
pub use dead_mutex::*;
