/// common print method
#[macro_export]
#[cfg(not(test))]
macro_rules! print {
    ($($arg:tt)*) => ({
        match format_args!($($arg)*) {
            a => {
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

/// dump lines in syslog
#[macro_export]
macro_rules! print_syslog {
    ($($arg:tt)*) => ({
        match format_args!($($arg)*) {
            a => {
                core::fmt::write(unsafe {$crate::terminal::TERMINAL.as_mut().unwrap().get_tty(0)}, a).unwrap();
            }
        }
    })
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
                    term.set_text_color($color);
                }
            }
        }
    }};
}

/// Common print fixed method
#[macro_export]
macro_rules! printfixed {
    ($cursor_pos:expr, $color:expr, $($arg:tt)*) => ({
        match format_args!($($arg)*) {
            a => {
                unsafe {
                    match {$crate::terminal::TERMINAL.as_mut()} {
                        None => {},
                        Some(term) => {
                            use crate::terminal::WriteMode;
                            use crate::monitor::Pos;;

                            let tty = term.get_tty(1);
                            let env = tty.modify(WriteMode::Fixed, $cursor_pos, $color);

                            core::fmt::write({tty}, a).unwrap();

                            tty.modify(env.0, env.1, env.2);
                        }
                    }
                }
            }
        }
    })
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
                unsafe {
                    // For national security, force unlock this mutex
                    crate::monitor::SCREEN_MONAD.force_unlock();

                    match {$crate::terminal::TERMINAL.as_mut()} {
                        None => {
                            use crate::early_terminal::EARLY_TERMINAL;
                            core::fmt::write(&mut EARLY_TERMINAL, a).unwrap()
                        },
                        // I consider it's works !
                        Some(term) => core::fmt::write({term.get_foreground_tty().unwrap()}, a).unwrap(),
                    }
                }
            }
        }
    })
}

/// eprintln without UART, dumping data in some active TTY
#[cfg(not(feature = "serial-eprintln"))]
#[cfg(not(test))]
#[macro_export]
macro_rules! eprintln {
    () => (print_screen!("\n"));
    ($fmt:expr, $($arg:tt)*) => ($crate::print_screen!(concat!($fmt, "\n"), $($arg)*));
    ($fmt:expr) => ($crate::print_screen!(concat!($fmt, "\n")));
}

/// eprintln! with UART
#[cfg(feature = "serial-eprintln")]
#[cfg(not(test))]
#[macro_export]
macro_rules! eprintln {
    ($($arg:tt)*) => ($crate::serial_println!($($arg)*));
}
