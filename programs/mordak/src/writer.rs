//! This module manage println! and print! rust macros

use core::fmt::Write;

/// Main Writer structure
pub struct Writer {}

impl Writer {
    /// Void new declaration
    pub const fn new() -> Self {
        Self {}
    }
}

impl Write for Writer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        rust_write(1, s.as_ptr(), s.len())
    }
}

/// Main Writer Globale
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

/// Returns a &str containing the full namespace specified name of the function
/// This works by declaring a dummy function f() nested in the current function.
/// Then by the type_name instrinsics, get the slice of the full specified name of the function f()
/// we then truncate the slice by the range notation to the name of the current function.
/// That is the slice with 5 characters removed.
#[allow(unused_macros)]
#[macro_export]
macro_rules! function {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            extern crate core;
            unsafe { core::intrinsics::type_name::<T>() }
        }
        let name = type_name_of(f);
        &name[6..name.len() - 4]
    }};
}
