//! Dome builtins

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

pub fn broken_fish(_args: &[&str]) -> u8 {
    fish::broken_fish();
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
            let env: &[&str] = &["VAR_A=A", "VAR_B=B"];
            let env_array: CStringArray = env.into();

            let f = fork();
            if f < 0 {
                println!("Fork Failed");
                1
            } else if f == 0 {
                execve(filename.as_ptr(), argv.as_ptr(), env_array.as_ptr()) as u8;
                perror("execve failed\0".as_ptr());
                1
            } else {
                setpgid(f, f);
                tcsetpgrp(0, f);
                let mut status: i32 = 0;
                let w = wait(&mut status as *mut i32);
                tcsetpgrp(0, getpgrp());
                println!("wait finished: ret = {} status = {}", w, status);
                0
            }
        } else {
            1
        }
    }
}

extern "C" {
    fn execve(
        filename: *const c_char,
        argv: *const *const c_char,
        envp: *const *const c_char,
    ) -> i32;
    fn fork() -> i32;
    fn wait(status: *mut i32) -> i32;
    fn perror(msg: *const u8);
    fn tcsetpgrp(fildes: i32, pgid: i32);
    fn getpgrp() -> i32;
    fn setpgid(pid: i32, pgid: i32);
}
