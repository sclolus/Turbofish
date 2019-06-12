//! impl Fallible collections on allocation errors, quite as describe
//! in [RFC 2116](https://github.com/rust-lang/rfcs/blob/master/text/2116-alloc-me-maybe.md)
#![cfg_attr(not(test), no_std)]
#![feature(try_reserve)]
#![feature(specialization)]
#![feature(allocator_api)]
#![deny(missing_docs)]

extern crate alloc;
pub mod boxed;
pub use boxed::*;
#[macro_use]
pub mod vec;
pub use vec::*;
pub mod rc;
pub use rc::*;

use alloc::collections::CollectionAllocErr;

/// trait for trying to clone an elem, return an error instead of
/// panic if allocation failed
pub trait TryClone {
    /// try clone method, (Self must be size because of Result
    /// constraint)
    fn try_clone(&self) -> Result<Self, CollectionAllocErr>
    where
        Self: core::marker::Sized;
}
