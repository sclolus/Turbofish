#![cfg_attr(not(test), no_std)]
#![feature(asm)]
pub mod spinlock;
pub use spinlock::*;
pub mod dead_mutex;
pub use dead_mutex::*;
pub mod uninterruptible_mutex;
pub use uninterruptible_mutex::*;
