/// common print method
#[macro_export]
#[cfg(not(test))]
#[cfg(not(feature = "std-print"))]
macro_rules! print {
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        match format_args!($($arg)*) {
            a => {
                #[allow(unused_unsafe)]
                unsafe {
                    match $crate::TERMINAL.as_mut() {
                        None => {
                            use $crate::EARLY_TERMINAL;
                            core::fmt::write(&mut EARLY_TERMINAL, a).unwrap()
                        },
                        Some(term) => term.get_foreground_tty().tty.write_fmt(a).unwrap(),
                    }
                }
            }
        }
    })
}

#[macro_export]
macro_rules! print_tty {
    ($tty_number:expr, $($arg:tt)*) => ({
        use core::fmt::Write;
        match format_args!($($arg)*) {
            a => {
                #[allow(unused_unsafe)]
                unsafe {
                    match $crate::TERMINAL.as_mut() {
                        None => {
                            use $crate::EARLY_TERMINAL;
                            core::fmt::write(&mut EARLY_TERMINAL, a).unwrap()
                        },
                        Some(term) => {
                            term.get_tty($tty_number).write_fmt(a).unwrap();
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
macro_rules! print_bypass_mutex {
    ($($arg:tt)*) => ({
        #[allow(unused_unsafe)]
        match format_args!($($arg)*) {
            a => {
                unsafe {
                    // For national security, force unlock this mutex
                    screen::SCREEN_MONAD.force_unlock();

                    match $crate::TERMINAL.as_mut() {
                        None => {
                            use $crate::EARLY_TERMINAL;
                            core::fmt::write(&mut EARLY_TERMINAL, a).unwrap()
                        },
                        // I consider it's works !
                        Some(term) => {
                            use core::fmt::Write;
                            term.get_foreground_tty().tty.tty.write_fmt(a).unwrap();
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
#[cfg(not(feature = "std-print"))]
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
                unsafe {
                    match {$crate::TERMINAL.as_mut()} {
                        None => {},
                        Some(term) => {
                            term.get_tty($crate::SYSTEM_LOG_TTY_IDX).write_fmt(a).unwrap();
                        }
                    }
                }
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
                    match {$crate::TERMINAL.as_mut()} {
                        None => {},
                        Some(term) => {
                            use $crate::WriteMode;
                            use ansi_escape_code::Pos;
                            use core::fmt::Write;

                            let btty = &mut term.get_foreground_tty().tty;
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

/// eprint! without UART, dumping data in some active TTY
#[cfg(not(feature = "serial-eprintln"))]
#[cfg(not(test))]
#[cfg(not(feature = "std-print"))]
#[macro_export]
macro_rules! eprint {
    ($($arg:tt)*) => ($crate::print_bypass_mutex!($($arg)*))
}

/// eprint! with UART
#[cfg(feature = "serial-eprintln")]
#[cfg(not(test))]
#[macro_export]
macro_rules! eprint {
    ($($arg:tt)*) => ($crate::serial_print!($($arg)*));
}

/// eprintln! without UART, dumping data in some active TTY
#[cfg(not(feature = "serial-eprintln"))]
#[cfg(not(test))]
#[cfg(not(feature = "std-print"))]
#[macro_export]
macro_rules! eprintln {
    () => ($crate::eprint!("\n"));
    // ($fmt:expr, $($arg:tt)*) => ($crate::eprint!($fmt, $($arg)*));
    // ($fmt:expr) => ($crate::eprint!($fmt));
    ($fmt:expr, $($arg:tt)*) => ($crate::eprint!(concat!($fmt, "\n"), $($arg)*));
    ($fmt:expr) => ($crate::eprint!(concat!($fmt, "\n")));
}

/// eprintln! with UART
#[cfg(feature = "serial-eprintln")]
#[cfg(not(test))]
#[macro_export]
macro_rules! eprintln {
    ($($arg:tt)*) => ($crate::serial_println!($($arg)*));
}

/// copy of std dbg! macro
#[allow(unused_macros)]
#[macro_export]
#[cfg(not(feature = "std-print"))]
macro_rules! dbg {
    ($val: expr) => {
        match $val {
            tmp => {
                $crate::eprintln!(
                    "[{}:{}] {} = {:#?}",
                    file!(),
                    line!(),
                    stringify!($val),
                    &tmp
                );
                tmp
            }
        }
    };
}

/// copy of std dbg! macro
#[allow(unused_macros)]
#[macro_export]
macro_rules! dbg_hex {
    ($val: expr) => {
        match $val {
            tmp => {
                $crate::println!(
                    "[{}:{}] {} = {:#X?}",
                    file!(),
                    line!(),
                    stringify!($val),
                    &tmp
                );
                tmp
            }
        }
    };
}
