//! This module manage println! and print! rust macros

use core::fmt::{Result, Write};

/// Main Writer structure
pub struct Writer {
    f: Option<fn(&str)>,
}

/// Main implementation
impl Writer {
    /// Void new declaration
    const fn new() -> Self {
        Self { f: None }
    }

    /// Set the given write callback
    pub fn set_write_callback(&mut self, f: fn(&str)) {
        self.f = Some(f);
    }
}

/// Standard implementation trait
impl Write for Writer {
    fn write_str(&mut self, s: &str) -> Result {
        (self.f.as_ref().expect("Cannot write"))(s);
        Ok(())
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
