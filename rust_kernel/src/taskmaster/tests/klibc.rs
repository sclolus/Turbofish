//! Fake libC on kernel side

pub mod syscall;
pub use syscall::{_user_exit, _user_fork, _user_write};
