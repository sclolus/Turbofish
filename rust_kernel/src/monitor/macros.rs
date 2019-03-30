/// This structure is made for bypassing the SCREEN_MONAD mutex, it guaranted the chain to be displayed
pub struct WriterBypassMutex;

impl core::fmt::Write for WriterBypassMutex {
    fn write_str(&mut self, _s: &str) -> core::fmt::Result {
        unsafe {
            crate::monitor::SCREEN_MONAD.force_unlock();
            Ok(())
        }
        // crate::monitor::SCREEN_MONAD.lock().write_str(s)
    }
}

/// Print_screen allow to write directly into the SCREEN_MONAD and bypass his mutex,
/// Use only when Panic or some fatal error occured !
#[macro_export]
#[cfg(not(test))]
macro_rules! print_screen {
    ($($arg:tt)*) => ({
        #[allow(unused_unsafe)]
        match format_args!($($arg)*) {
            a => {
                core::fmt::write(&mut $crate::monitor::macros::WriterBypassMutex, a).unwrap();
            }
        }
    })
}

#[cfg(not(feature = "serial-eprintln"))]
#[cfg(not(test))]
#[macro_export]
macro_rules! eprintln {
    () => (print_screen!("\n"));
    ($fmt:expr, $($arg:tt)*) => ($crate::print_screen!(concat!($fmt, "\n"), $($arg)*));
    ($fmt:expr) => ($crate::print_screen!(concat!($fmt, "\n")));
}

#[cfg(feature = "serial-eprintln")]
#[cfg(not(test))]
#[macro_export]
macro_rules! eprintln {
    ($($arg:tt)*) => ($crate::serial_println!($($arg)*));
}
