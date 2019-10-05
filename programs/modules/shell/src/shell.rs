//! Shell is a blocked function, it display prompt and wait for a command

use ansi_escape_code::CursorMove;

mod builtin;
use builtin::{
    broken_fish, echo, exec, fish, hello_world, ls, more_fish, reboot_computer, shutdown_computer,
};

use alloc::string::String;
use alloc::vec::Vec;

/// ASCII mouse
const PROMPT: &str = "----{,_,\"> $ ";

/// Main function
pub fn shell() -> ! {
    unsafe { set_raw_mode() };
    loop {
        // Display prompt
        print!("{}", PROMPT);
        print!("{}", CursorMove::Forward(0));
        // Call to blocked read_line function
        let line = read_line();
        // Make a line jump
        print!("\n");
        // Execute command
        if line.starts_with("/") {
            unsafe { set_cooked_mode() };
            exec(&line.split_whitespace().collect::<Vec<&str>>());
            unsafe { set_raw_mode() };
        } else {
            exec_builtin(&line);
        }
    }
}

/// List of some builtins
const BUILTINS: [(&str, fn(&[&str]) -> u8); 9] = [
    ("ls", ls),
    ("echo", echo),
    ("hello_world", hello_world),
    ("fish", fish),
    ("more_fish", more_fish),
    ("broken_fish", broken_fish),
    ("reboot", reboot_computer),
    ("shutdown", shutdown_computer),
    ("exec", exec),
];

/// Exectution of builtin commands
fn exec_builtin(line: &str) {
    let mut split = line.split_whitespace();
    let command = split.next().unwrap_or("");
    if command == "" {
        return;
    }
    let args: Vec<&str> = split.collect();
    match BUILTINS.iter().find(|(c, _)| c == &command) {
        None => {
            println!("{}: command not found", command);
        }
        Some((_c, f)) => {
            f(args.as_slice());
        }
    };
}

/// Blocked read line
fn read_line() -> String {
    let mut line = String::new();
    let mut cursor_pos = 0;
    let mut buf: [u8; 8] = [0; 8];

    let mut graphical_cursor_offset = 0;
    let mut graphical_len = 0;

    loop {
        let ret = unsafe { read(0, buf.as_mut_ptr() as *mut u8, 1) };
        if ret == -1 {
            panic!("read failed");
        }
        let first = buf[0];
        match first {
            key if key == ('\n' as u8) => {
                print!("{}", &line[cursor_pos..]);
                return line;
            }
            key if ((key >= ' ' as u8) && (key <= '~' as u8) && (key != 127)) => {
                line.insert(cursor_pos, key as u8 as char);

                print!("{}", &line[cursor_pos..]);

                cursor_pos += (key as u8 as char).len_utf8();

                graphical_cursor_offset += 1;
                graphical_len += 1;

                print!(
                    "{}",
                    CursorMove::Backward(graphical_len - graphical_cursor_offset)
                );
            }
            27 => {
                let ret = unsafe { read(0, buf[1..].as_mut_ptr() as *mut u8, 2) };
                if ret == -1 {
                    panic!("read failed");
                }
                if ret != 2 {
                    continue;
                }
                match buf[1..3] {
                    // left
                    [79, 68] => {
                        if cursor_pos > 0 {
                            while !line.is_char_boundary(cursor_pos - 1) {
                                cursor_pos -= 1;
                            }
                            cursor_pos -= 1;
                            graphical_cursor_offset -= 1;

                            print!("{}", CursorMove::Backward(1))
                        }
                    }

                    //right
                    [79, 67] => {
                        if cursor_pos < line.len() {
                            while !line.is_char_boundary(cursor_pos + 1) {
                                cursor_pos += 1;
                            }
                            cursor_pos += 1;

                            graphical_cursor_offset += 1;

                            print!("{}", CursorMove::Forward(1));
                        }
                    }
                    _ => (),
                }
            }
            /* delete */
            127 => {
                if cursor_pos > 0 {
                    while !line.is_char_boundary(cursor_pos - 1) {
                        cursor_pos -= 1;
                    }
                    line.remove(cursor_pos - 1);
                    cursor_pos -= 1;

                    graphical_cursor_offset -= 1;
                    graphical_len -= 1;

                    print!("{}", CursorMove::Backward(1));
                    if cursor_pos == line.len() {
                        print!("{}", " ");
                    } else {
                        print!("{}", &line[cursor_pos..]);
                        print!("{}", " ");
                    }
                    print!(
                        "{}",
                        CursorMove::Backward(graphical_len - graphical_cursor_offset + 1)
                    );
                }
            }
            _ => {}
        };
    }
}

extern "C" {
    fn read(fd: i32, buf: *mut u8, len: usize) -> isize;
    pub fn set_raw_mode();
    pub fn set_cooked_mode();
}
