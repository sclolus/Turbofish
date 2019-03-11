#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
        match format_args!($($arg)*) {
            a => {
                core::fmt::write(unsafe {&mut $crate::monitor::SCREEN_MONAD}, a).unwrap();
            }
        }
    })
}

#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($fmt:expr, $($arg:tt)*) => ($crate::print!(concat!($fmt, "\n"), $($arg)*));
    ($fmt:expr) => ($crate::print!(concat!($fmt, "\n")));
}

#[cfg(not(feature = "serial-eprintln"))]
#[macro_export]
macro_rules! eprintln {
    ($($arg:tt)*) => ($crate::println!($($arg)*));
}

#[cfg(feature = "serial-eprintln")]
#[macro_export]
macro_rules! eprintln {
    ($($arg:tt)*) => ($crate::serial_println!($($arg)*));
}
