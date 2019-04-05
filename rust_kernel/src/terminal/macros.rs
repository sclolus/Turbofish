/// common print method
#[macro_export]
#[cfg(not(test))]
macro_rules! print {
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        match format_args!($($arg)*) {
            a => {
                #[allow(unused_unsafe)]
                unsafe {
                    match $crate::terminal::TERMINAL.as_mut() {
                        None => {
                            use crate::terminal::EARLY_TERMINAL;
                            core::fmt::write(&mut EARLY_TERMINAL, a).unwrap()
                        },
                        Some(term) => term.get_tty(1).write_fmt(a).unwrap(),
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
macro_rules! print_bypass_mutex {
    ($($arg:tt)*) => ({
        #[allow(unused_unsafe)]
        match format_args!($($arg)*) {
            a => {
                unsafe {
                    // For national security, force unlock this mutex
                    crate::terminal::monitor::SCREEN_MONAD.force_unlock();

                    match $crate::terminal::TERMINAL.as_mut() {
                        None => {
                            use crate::terminal::EARLY_TERMINAL;
                            core::fmt::write(&mut EARLY_TERMINAL, a).unwrap()
                        },
                        // I consider it's works !
                        Some(term) => {
                            use core::fmt::Write;
                            term.get_foreground_tty().unwrap().write_fmt(a).unwrap();
                        }
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
                use core::fmt::Write;
                unsafe {$crate::terminal::TERMINAL.as_mut().unwrap().get_tty(0).write_fmt(a).unwrap();}
            }
        }
    })
}

/// Common print fixed method
#[macro_export]
macro_rules! printfixed {
    ($cursor_pos:expr, $($arg:tt)*) => ({
        match format_args!($($arg)*) {
            a => {
                unsafe {
                    match {$crate::terminal::TERMINAL.as_mut()} {
                        None => {},
                        Some(term) => {
                            use crate::terminal::WriteMode;
                            use crate::terminal::Pos;
                            use core::fmt::Write;

                            let btty = term.get_tty(1);
                            let (save_write_mode, save_cursor) = (btty.tty.write_mode, btty.tty.cursor.pos);
                            btty.tty.write_mode = WriteMode::Fixed;
                            btty.tty.cursor.pos = $cursor_pos;

                            btty.write_fmt(a).unwrap();

                            btty.tty.write_mode = save_write_mode;
                            btty.tty.cursor.pos = save_cursor;
                        }
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
    () => ($crate::print_bypass_mutex!("\n"));
    ($fmt:expr, $($arg:tt)*) => ($crate::print_bypass_mutex!(concat!($fmt, "\n"), $($arg)*));
    ($fmt:expr) => ($crate::print_bypass_mutex!(concat!($fmt, "\n")));
}

/// eprintln! with UART
#[cfg(feature = "serial-eprintln")]
#[cfg(not(test))]
#[macro_export]
macro_rules! eprintln {
    ($($arg:tt)*) => ($crate::serial_println!($($arg)*));
}
