use core::cmp::PartialEq;

pub mod pio;
pub use pio::{Pio, _inb, _inw, _inl, _outb, _outl, _outw, io_wait};

// The general Io trait, for In/out objects
pub trait Io {
    type Value: PartialEq;

    fn read(&self) -> Self::Value;
    fn write(&mut self, value: Self::Value);
}

// This is a struct that encapsulate an object that is Io, in a read only mode
pub struct ReadOnly<I: Io> {
    inner: I,
}

impl<I: Io> ReadOnly<I> {
    pub fn new(inner: I) -> Self {
        ReadOnly {
            inner
        }
    }

    pub fn read(&self) -> I::Value {
        self.inner.read()
    }
}


// This is a struct that encapsulate an object that is Io, in a write only mode
pub struct WriteOnly<I: Io> {
    inner: I,
}

impl <I: Io> WriteOnly<I> {
    pub fn new(inner: I) -> Self {
        WriteOnly {
            inner
        }
    }

    pub fn write(&mut self, value: I::Value) {
        self.inner.write(value)
    }

}
