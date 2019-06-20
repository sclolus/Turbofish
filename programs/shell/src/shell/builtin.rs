//! Dome builtins

use super::ansi_escape_code;

mod fish;

mod really_lazy_hello_world;
use really_lazy_hello_world::really_lazy_hello_world;

// simple, basic
pub fn echo(args: &[&str]) -> u8 {
    for s in args {
        print!("{} ", s);
    }
    print!("\n");
    0
}

/// list all files
pub fn ls(_args: &[&str]) -> u8 {
    print!("Fuck you !\n");
    0
}

/// Display a fish
pub fn fish(_args: &[&str]) -> u8 {
    fish::fish();
    0
}

/// display more fish
pub fn more_fish(_args: &[&str]) -> u8 {
    fish::fish2();
    0
}

/// display a very lazy hello world
pub fn hello_world(_args: &[&str]) -> u8 {
    really_lazy_hello_world();
    0
}

/// display a very lazy hello world
pub fn reboot_computer(_args: &[&str]) -> u8 {
    unsafe {
        reboot();
    }
    1
}

/// display a very lazy hello world
pub fn shutdown_computer(_args: &[&str]) -> u8 {
    unsafe {
        shutdown();
    }
    1
}

extern "C" {
    fn reboot() -> i32;
    fn shutdown() -> i32;
}

use crate::ffi::{c_char, CString, CStringArray};

/// Execute a program
pub fn exec(args: &[&str]) -> u8 {
    unsafe {
        if args.len() > 0 {
            let filename: CString = args[0].into();
            let argv: CStringArray = args.into();

            execve(filename.as_ptr(), argv.as_ptr(), 0 as *const *const c_char) as u8
        } else {
            1
        }
    }
}

extern "C" {
    fn execve(filename: *const c_char, argv: *const *const c_char, envp: *const *const c_char) -> i32;
}
