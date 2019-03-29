/// This structure is made for bypassing the SCREEN_MONAD mutex, it guaranted the chain to be displayed
pub struct WriterBypassMutex;

impl core::fmt::Write for WriterBypassMutex {
    fn write_str(&mut self, _s: &str) -> core::fmt::Result {
        unsafe {
            crate::monitor::SCREEN_MONAD.force_unlock();
            Ok(())
        }
        //        crate::monitor::SCREEN_MONAD.lock().write_str(s)
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

/// Common structure of printer
pub struct Writer;

impl core::fmt::Write for Writer {
    fn write_str(&mut self, _s: &str) -> core::fmt::Result {
        // crate::monitor::SCREEN_MONAD.lock().write_str(s)
        Ok(())
    }
}

/// dump lines in syslog
#[macro_export]
macro_rules! print_syslog {
    ($($arg:tt)*) => ({
        match format_args!($($arg)*) {
            _a => {
//                core::fmt::write(unsafe {$crate::terminal::TERMINAL.as_mut().unwrap().get_tty(0)}, a).unwrap();
            }
        }
    })
}

/*
/// common print fixed method
#[macro_export]
macro_rules! printfixed {
    ($x:expr, $y:expr, $($arg:tt)*) => ({
        match format_args!($($arg)*) {
            a => {
                use crate::monitor::SCREEN_MONAD;

                let cursor = SCREEN_MONAD.lock().get_cursor_position();
                SCREEN_MONAD.lock().set_cursor_position($x, $y).unwrap();
                SCREEN_MONAD.lock().set_write_mode(WriteMode::Fixed).unwrap();

                core::fmt::write(&mut $crate::monitor::macros::Writer, a).unwrap();

                SCREEN_MONAD.lock().set_cursor_position(cursor.0, cursor.1).unwrap();
                SCREEN_MONAD.lock().set_write_mode(WriteMode::Dynamic).unwrap();
            }
        }
    })
}
*/

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

/*
/// common print method
#[macro_export]
#[cfg(not(test))]
macro_rules! print {
    ($($arg:tt)*) => ({
        match format_args!($($arg)*) {
            a => {
                #[allow(unused_unsafe)]
                match unsafe {$crate::terminal::TERMINAL.as_mut()} {
                    Some(term) => core::fmt::write(unsafe {term.get_tty(1)}, a).unwrap(),
                    None => core::fmt::write(&mut $crate::monitor::macros::Writer, a).unwrap(),
                }
            }
        }
    })
}

#[macro_export]
#[cfg(not(test))]
macro_rules! println {
    () => (print!("\n"));
    ($fmt:expr, $($arg:tt)*) => ($crate::print!(concat!($fmt, "\n"), $($arg)*));
    ($fmt:expr) => ($crate::print!(concat!($fmt, "\n")));
}
*/
