#[macro_export]
#[cfg(not(test))]
macro_rules! print_screen {
    ($($arg:tt)*) => ({
        #[allow(unused_unsafe)]
        match format_args!($($arg)*) {
            a => {
                #[allow(unused_unsafe)]
                core::fmt::write(unsafe {&mut $crate::monitor::SCREEN_MONAD}, a).unwrap();
                // core::fmt::write(unsafe {$crate::terminal::TERMINAL.as_mut().unwrap().get_tty(1)}, a).unwrap();
            }
        }
    })
}

#[macro_export]
#[cfg(not(test))]
macro_rules! print {
    ($($arg:tt)*) => ({
        match format_args!($($arg)*) {
            a => {
                #[allow(unused_unsafe)]
                match unsafe {$crate::terminal::TERMINAL.as_mut()} {
                    Some(term) => core::fmt::write(unsafe {term.get_tty(1)}, a).unwrap(),
                    None => core::fmt::write(unsafe {&mut $crate::monitor::SCREEN_MONAD}, a).unwrap(),
                }
                // core::fmt::write(unsafe {&mut $crate::monitor::SCREEN_MONAD}, a).unwrap();
                // core::fmt::write(unsafe {$crate::terminal::TERMINAL.as_mut().unwrap().get_tty(1)}, a).unwrap();
            }
        }
    })
}

#[macro_export]
macro_rules! print_tty {
    ($($arg:tt)*) => ({
        match format_args!($($arg)*) {
            a => {
                core::fmt::write(unsafe {$crate::terminal::TERMINAL.as_mut().unwrap().get_tty(0)}, a).unwrap();
            }
        }
    })
}

#[macro_export]
macro_rules! printfixed {
    ($x:expr, $y:expr, $($arg:tt)*) => ({
        match format_args!($($arg)*) {
            a => {
                #[allow(unused_unsafe)]
                unsafe {
                    use crate::monitor::SCREEN_MONAD;

                    let cursor = SCREEN_MONAD.get_cursor_position();
                    SCREEN_MONAD.set_cursor_position($x, $y).unwrap();
                    SCREEN_MONAD.set_write_mode(WriteMode::Fixed).unwrap();

                    core::fmt::write(&mut $crate::monitor::SCREEN_MONAD, a).unwrap();

                    SCREEN_MONAD.set_cursor_position(cursor.0, cursor.1).unwrap();
                    SCREEN_MONAD.set_write_mode(WriteMode::Dynamic).unwrap();
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

#[cfg(not(feature = "serial-eprintln"))]
#[cfg(not(test))]
#[macro_export]
macro_rules! eprintln {
    ($($arg:tt)*) => ($crate::println!($($arg)*));
}

#[cfg(feature = "serial-eprintln")]
#[cfg(not(test))]
#[macro_export]
macro_rules! eprintln {
    ($($arg:tt)*) => ($crate::serial_println!($($arg)*));
}
