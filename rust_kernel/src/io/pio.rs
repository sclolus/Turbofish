use core::marker::PhantomData;
use super::Io;
use core::cmp::PartialEq;

extern "C" {
    fn asm_inb(port: u16) -> u8;
    fn asm_inw(port: u16) -> u16;
    fn asm_inl(port: u16) -> u32;
    
    fn asm_outb(byte: u8, port: u16);
    fn asm_outw(byte: u16, port: u16);
    fn asm_outl(byte: u32, port: u16);
    
    fn asm_io_wait();
}

#[no_mangle]
#[inline(always)]
pub extern "C" fn _inb(port: u16) -> u8 {
    unsafe { asm_inb(port) }
}

#[no_mangle]
#[inline(always)]
pub extern "C" fn _inw(port: u16) -> u16 {
    unsafe { asm_inw(port) }
}

#[no_mangle]
#[inline(always)]
pub extern "C" fn _inl(port: u16) -> u32 {
    unsafe { asm_inl(port) }
}


#[no_mangle]
#[inline(always)]
pub extern "C" fn _outb(byte: u8, port: u16) {
    unsafe { asm_outb(byte, port) }
}

#[no_mangle]
#[inline(always)]
pub extern "C" fn _outw(byte: u16, port: u16) {
    unsafe { asm_outw(byte, port) }
}

#[no_mangle]
#[inline(always)]
pub extern "C" fn _outl(byte: u32, port: u16) {
    unsafe { asm_outl(byte, port) }
}

#[no_mangle]
#[inline(always)]
pub extern "C" fn io_wait() {
    unsafe { asm_io_wait() }
}

pub struct Pio<T> {
    port: u16,
    value: PhantomData<T>,
}

impl<T> Pio<T> {
    pub const fn new(port: u16) -> Self {
        Pio {
            port,
            value: PhantomData
        }
    }
}

impl Io for Pio<u8> {
    type Value = u8;

    fn read(&self) -> Self::Value {
        _inb(self.port)
    }
    
    fn write(&mut self, value: Self::Value) {
        _outb(value, self.port)
    }
}

impl Io for Pio<u16> {
    type Value = u16;

    fn read(&self) -> Self::Value {
        _inw(self.port)
    }
    
    fn write(&mut self, value: Self::Value) {
        _outw(value, self.port)
    }
}

impl Io for Pio<u32> {
    type Value = u32;

    fn read(&self) -> Self::Value {
        _inl(self.port)
    }
    
    fn write(&mut self, value: Self::Value) {
        _outl(value, self.port)
    }
}
