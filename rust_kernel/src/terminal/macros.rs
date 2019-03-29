/// common print method
#[macro_export]
#[cfg(not(test))]
macro_rules! print {
    ($($arg:tt)*) => ({
        match format_args!($($arg)*) {
            a => {
                #[allow(unused_unsafe)]
                unsafe {
                    match {$crate::terminal::TERMINAL.as_mut()} {
                        None => {
                            use crate::early_terminal::EARLY_TERMINAL;
                            core::fmt::write(&mut EARLY_TERMINAL, a).unwrap()
                        },
                        Some(term) => core::fmt::write({term.get_tty(1)}, a).unwrap(),
                    }
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

/// common set_text_color method
#[macro_export]
#[cfg(not(test))]
macro_rules! set_text_color {
    ($color:expr) => {{
        unsafe {
            match { $crate::terminal::TERMINAL.as_mut() } {
                None => {
                    use crate::early_terminal::EARLY_TERMINAL;
                    EARLY_TERMINAL.set_text_color($color);
                }
                Some(term) => {
                    term.get_tty(1).set_text_color($color);
                }
            }
        }
    }};
}
