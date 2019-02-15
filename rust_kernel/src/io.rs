use core::cmp::PartialEq;

pub mod pio;
pub use pio::{io_wait, Pio};
#[macro_use]
pub mod uart_16550;
pub use uart_16550::UART_16550;

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
    pub fn new(inner: I) -> Self {
        ReadOnly { inner }
    }

    pub fn read(&self) -> I::Value {
        self.inner.read()
    }
}

/// This is a struct that encapsulate an object that is Io, in a write only mode
pub struct WriteOnly<I: Io> {
    inner: I,
}

impl<I: Io> WriteOnly<I> {
    pub fn new(inner: I) -> Self {
        WriteOnly { inner }
    }

    pub fn write(&mut self, value: I::Value) {
        self.inner.write(value)
    }
}
