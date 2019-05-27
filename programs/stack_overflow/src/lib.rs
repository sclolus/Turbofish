#![cfg_attr(not(test), no_std)]

#[no_mangle]
fn main() -> i32 {
    println!("Stack overflow program, do recursive");
    stack_overflow(42, 42, 42, 42, 42, 42)
}

#[allow(unconditional_recursion)]
fn stack_overflow(a: u32, b: u32, c: u32, d: u32, e: u32, f: u32) -> i32 {
    println!("Sum: {:?}", a + b + c + d + e + f);
    stack_overflow(a + 1, b + 1, c + 1, d + 1, e + 1, f + 1)
}

pub struct Writer {}

impl Writer {
    pub const fn new() -> Self {
        Self {}
    }
}

use core::fmt::Write;

impl Write for Writer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        unsafe {
            user_write(1, s.as_ptr(), s.len());
        }
        Ok(())
    }
}

pub static mut WRITER: Writer = Writer::new();

/// common print method
#[macro_export]
#[cfg(not(test))]
macro_rules! print {
    ($($arg:tt)*) => ({
        match format_args!($($arg)*) {
            a => {
                use crate::WRITER;
                unsafe {
                    core::fmt::write(&mut WRITER, a).unwrap();
                }
            }
        }
    })
}

/// common println method
#[macro_export]
#[cfg(not(test))]
macro_rules! println {
    () => (print!("\n"));
    ($fmt:expr, $($arg:tt)*) => ($crate::print!(concat!($fmt, "\n"), $($arg)*));
    ($fmt:expr) => ($crate::print!(concat!($fmt, "\n")));
}

#[panic_handler]
#[no_mangle]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    println!("Panic sa mere !");
    loop {}
}

extern "C" {
    fn user_write(fd: i32, s: *const u8, len: usize) -> i32;
    // fn user_fork() -> i32;
}
