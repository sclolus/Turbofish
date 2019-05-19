//! This file contains easy to use macro for fake Kernel LibC (usable for tests)

#[macro_export]
macro_rules! user_print {
    ($($arg:tt)*) => ({
        use crate::taskmaster::tests::klibc::syscall::USER_WRITER;
        match format_args!($($arg)*) {
            a => {
                core::fmt::write(&mut USER_WRITER, a).unwrap();
            }
        }
    })
}
#[macro_export]
macro_rules! user_eprintln {
    () => (print!("\n"));
    ($fmt:expr, $($arg:tt)*) => ($crate::user_print!(concat!($fmt, "\n"), $($arg)*));
    ($fmt:expr) => ($crate::user_print!(concat!($fmt, "\n")));
    () => {
    }
}
