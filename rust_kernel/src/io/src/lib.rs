//! This module provide methods to read and write on I/O ports
#![cfg_attr(not(test), no_std)]
#![feature(asm)]
#![deny(missing_docs)]

use core::cmp::PartialEq;

pub mod pio;
pub use pio::{io_wait, Pio};

/// The general Io trait, for In/out objects
pub trait Io {
    /// Type of the values returned by read and write.
    type Value: PartialEq;

    /// Reads from object returning a `Self::value`
    fn read(&self) -> Self::Value;

    /// Writes `value` to the object
    fn write(&mut self, value: Self::Value);
}

/// This is a struct that encapsulate an object that is Io, in a read only mode
pub struct ReadOnly<I: Io> {
    inner: I,
}

impl<I: Io> ReadOnly<I> {
    /// Global constructor
    pub fn new(inner: I) -> Self {
        ReadOnly { inner }
    }

    /// Reads from object returning a `I::value`
    pub fn read(&self) -> I::Value {
        self.inner.read()
    }
}

/// This is a struct that encapsulate an object that is Io, in a write only mode
pub struct WriteOnly<I: Io> {
    inner: I,
}

impl<I: Io> WriteOnly<I> {
    /// Global constructor
    pub fn new(inner: I) -> Self {
        WriteOnly { inner }
    }

    /// Writes `I::value` to the object
    pub fn write(&mut self, value: I::Value) {
        self.inner.write(value)
    }
}
