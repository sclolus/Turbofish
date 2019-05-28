use core::fmt::Write;

pub struct Writer {}

impl Writer {
    pub const fn new() -> Self {
        Self {}
    }
}

impl Write for Writer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        rust_write(1, s.as_ptr(), s.len())
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
                 #[allow(unused_unsafe)]
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

fn rust_write(fd: i32, s: *const u8, len: usize) -> core::fmt::Result {
    unsafe {
        if write(fd, s, len) < 0 {
            Err(core::fmt::Error)
        } else {
            Ok(())
        }
    }
}

extern "C" {
    fn write(fd: i32, s: *const u8, len: usize) -> i32;
}
