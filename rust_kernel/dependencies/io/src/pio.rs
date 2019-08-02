//! See [Port_IO](https://wiki.osdev.org/Port_IO)
use super::Io;
use core::marker::PhantomData;

/// This waits one IO cycle.
/// Most likely useless on most modern hardware.
/// Wait one io cycle by outb'ing at unused port (Needs a way to ensure it is unused)
#[no_mangle]
#[inline(always)]
pub extern "C" fn io_wait() {
    unsafe {
        asm!("out %al, %dx" :: "{al}"(0x42), "{dx}"(0x80));
    }
}

/// This is a generic structure to represent IO ports
/// It implements the IO Trait for u8, u16 and u32
#[derive(Debug)]
pub struct Pio<T> {
    port: u16,
    value: PhantomData<T>,
}

impl<T> Pio<T> {
    /// Returns a new Pio assigned to the port `port`
    pub const fn new(port: u16) -> Self {
        Pio {
            port,
            value: PhantomData,
        }
    }
}

impl Io for Pio<u8> {
    type Value = u8;

    fn read(&self) -> Self::Value {
        let result: Self::Value;
        unsafe {
            asm!("in %dx, %al" : "={al}"(result) : "{dx}"(self.port));
        }
        result
    }

    fn write(&mut self, value: Self::Value) {
        unsafe {
            asm!("out %al, %dx" :: "{al}"(value), "{dx}"(self.port));
        }
    }
}

impl Io for Pio<u16> {
    type Value = u16;

    fn read(&self) -> Self::Value {
        let result: Self::Value;
        unsafe {
            asm!("in %dx, %ax" : "={ax}"(result) : "{dx}"(self.port));
        }
        result
    }

    fn write(&mut self, value: Self::Value) {
        unsafe {
            asm!("out %ax, %dx" :: "{ax}"(value), "{dx}"(self.port));
        }
    }
}

impl Io for Pio<u32> {
    type Value = u32;

    fn read(&self) -> Self::Value {
        let result: Self::Value;
        unsafe {
            asm!("in %dx, %eax" : "={eax}"(result) : "{dx}"(self.port));
        }
        result
    }

    fn write(&mut self, value: Self::Value) {
        unsafe {
            asm!("out %eax, %dx" :: "{eax}"(value), "{dx}"(self.port));
        }
    }
}
