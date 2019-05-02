extern "C" {
    pub fn _user_write(fd: i32, buf: *const u8, count: usize);
    pub fn _user_exit(status: i32) -> i32;
    pub fn _user_fork() -> i32;
}

pub struct UserWriter;

pub static mut USER_WRITER: UserWriter = UserWriter;

use core::fmt::Write;
impl Write for UserWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        unsafe {
            _user_write(1, s.as_ptr(), s.len());
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! user_print {
    ($($arg:tt)*) => ({
        use crate::syscall::test_syscall::USER_WRITER;
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
