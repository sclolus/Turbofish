//! This file contains lot of stuff to experiment syscalls

extern "C" {
    pub fn _user_write(fd: i32, buf: *const u8, count: usize);
    pub fn _user_exit(status: i32) -> i32;
    pub fn _user_fork() -> i32;
}

pub struct UserWriter;

pub static mut USER_WRITER: UserWriter = UserWriter;

/// Klibc write macros  are in /utils directory
use core::fmt::Write;
impl Write for UserWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        unsafe {
            _user_write(1, s.as_ptr(), s.len());
        }
        Ok(())
    }
}
