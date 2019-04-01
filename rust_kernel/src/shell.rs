mod builtin;
use crate::terminal::{CursorDirection, TERMINAL};
use alloc::string::String;
use alloc::vec::Vec;
use builtin::*;
use keyboard::keysymb::KeySymb;

// ASCII mouse
const PROMPT: &str = "----{,_,\"> $ ";

/// Blocked read
fn block_read(buf: &mut [KeySymb]) {
    unsafe {
        while TERMINAL.as_mut().unwrap().read(buf, 1) == 0 {
            asm!("hlt" :::: "volatile");
        }
    }
}

/// List of some builtins
const BUILTINS: [(&str, fn(&[&str]) -> BuiltinResult); 9] = [
    ("echo", echo),
    ("ls", ls),
    ("yes", yes),
    ("fucking_big_string", fucking_big_string),
    ("page_fault", page_fault),
    ("division_by_zero", division_by_zero),
    ("lspci", lspci),
    ("hello_world", hello_world),
    ("layout", layout),
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
            f(args.as_slice()).unwrap();
        }
    };
}

/// Blocked read line
fn read_line() -> String {
    let mut line = String::new();
    let mut cursor_pos = 0;
    let mut buf: [KeySymb; 1] = [KeySymb::nul; 1];

    let mut graphical_cursor_offset = 0;
    let mut graphical_len = 0;

    loop {
        block_read(&mut buf);
        let keysymb = buf[0];
        match keysymb {
            KeySymb::Return => {
                print!("{}", &line[cursor_pos..]);
                return line;
            }
            key if ((key >= KeySymb::space) && (key <= KeySymb::ydiaeresis) && (key != KeySymb::Delete)) => {
                line.insert(cursor_pos, key as u8 as char);

                print!("{}", &line[cursor_pos..]);

                cursor_pos += (key as u8 as char).len_utf8();

                graphical_cursor_offset += 1;
                graphical_len += 1;

                unsafe {
                    TERMINAL
                        .as_mut()
                        .unwrap()
                        .move_cursor(CursorDirection::Left, graphical_len - graphical_cursor_offset)
                };
            }
            KeySymb::Left => {
                if cursor_pos > 0 {
                    while !line.is_char_boundary(cursor_pos - 1) {
                        cursor_pos -= 1;
                    }
                    cursor_pos -= 1;

                    graphical_cursor_offset -= 1;

                    unsafe { TERMINAL.as_mut().unwrap().move_cursor(CursorDirection::Left, 1) };
                }
            }
            KeySymb::Right => {
                if cursor_pos < line.len() {
                    while !line.is_char_boundary(cursor_pos + 1) {
                        cursor_pos += 1;
                    }
                    cursor_pos += 1;

                    graphical_cursor_offset += 1;

                    unsafe { TERMINAL.as_mut().unwrap().move_cursor(CursorDirection::Right, 1) };
                }
            }
            KeySymb::Delete => {
                if cursor_pos > 0 {
                    while !line.is_char_boundary(cursor_pos - 1) {
                        cursor_pos -= 1;
                    }
                    line.remove(cursor_pos - 1);
                    cursor_pos -= 1;

                    graphical_cursor_offset -= 1;
                    graphical_len -= 1;

                    unsafe { TERMINAL.as_mut().unwrap().move_cursor(CursorDirection::Left, 1) };
                    if cursor_pos == line.len() {
                        print!("{}", " ");
                    } else {
                        print!("{}", &line[cursor_pos..]);
                        print!("{}", " ");
                    }
                    unsafe {
                        TERMINAL
                            .as_mut()
                            .unwrap()
                            .move_cursor(CursorDirection::Left, graphical_len - graphical_cursor_offset + 1)
                    };
                }
            }
            _ => {}
        };
    }
}

/// Shell is a blocked function, it display prompt and wait for a command
pub fn shell() {
    loop {
        // Display prompt
        print!("{}", PROMPT);
        unsafe {
            TERMINAL.as_mut().unwrap().move_cursor(CursorDirection::Left, 0);
        }
        // Call to blocked read_line function
        let line = read_line();
        // Make a line jump
        print!("\n");
        // Execute command
        exec_builtin(&line);
    }
}
