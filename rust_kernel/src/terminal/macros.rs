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
                    term.get_tty(1).set_text_color($color);
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
