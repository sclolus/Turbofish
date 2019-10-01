//! This module manage emergency_print! rust macros

use core::fmt::Write;

/// Main Writer structure
pub struct EmergencyWriter {
    f: Option<fn(&str)>,
}

/// Main implementation
impl EmergencyWriter {
    /// Void new declaration
    pub const fn new() -> Self {
        Self { f: None }
    }

    /// Set the given write callback
    pub fn set_write_callback(&mut self, f: fn(&str)) {
        self.f = Some(f);
    }
}

/// Standard implementation trait
impl Write for EmergencyWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        (self.f.as_ref().expect("Cannot write"))(s);
        Ok(())
    }
}

/// Main Writer Globale
pub static mut EMERGENCY_WRITER: EmergencyWriter = EmergencyWriter::new();

/// common print method
#[macro_export]
#[cfg(not(test))]
macro_rules! emergency_print {
    ($($arg:tt)*) => ({
        match format_args!($($arg)*) {
            a => {
                use crate::EMERGENCY_WRITER;
                 #[allow(unused_unsafe)]
                unsafe {
                    core::fmt::write(&mut EMERGENCY_WRITER, a).unwrap();
                }
            }
        }
    })
}
