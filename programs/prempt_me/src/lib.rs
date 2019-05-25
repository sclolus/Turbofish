#![cfg_attr(not(test), no_std)]

#[no_mangle]
extern "C" fn _start() -> ! {
    let ret = main();
    unsafe { user_exit(ret) }
}

fn main() -> i32 {
    println!("initialise Rainbow");
    loop {
        if unsafe { rainbow() } != 0 {
            break;
        }
        if unsafe { user_rainbow() } != 0 {
            break;
        }
    }
    println!("rainbow error");
    -1
}

extern "C" {
    fn rainbow() -> i32;
    fn user_rainbow() -> i32;
}

pub struct Writer {}

impl Writer {
    pub const fn new() -> Self {
        Self {}
    }
}

use core::fmt::Write;

impl Write for Writer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        unsafe {
            user_write(1, s.as_ptr(), s.len());
        }
        Ok(())
    }
}

pub static mut WRITER: Writer = Writer::new();

/// common print method
#[macro_export]
#[cfg(not(test))]
macro_rules! print {
    ($($arg:tt)*) => ({
        match format_args!($($arg)*) {
            a => {
                use crate::WRITER;
                unsafe {
                    core::fmt::write(&mut WRITER, a).unwrap();
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

#[panic_handler]
#[no_mangle]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    println!("Panic sa mere !");
    loop {}
}

extern "C" {
    fn user_write(fd: i32, s: *const u8, len: usize) -> i32;
    fn user_exit(return_value: i32) -> !;
    // fn user_fork() -> i32;
}
