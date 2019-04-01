mod builtin;
use crate::drivers::keyboard::keysymb::KeySymb;
use crate::terminal::ansi_escape_code::CursorMove;
use crate::terminal::TERMINAL;
use alloc::string::String;
use alloc::vec::Vec;
use builtin::*;

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
const BUILTINS: [(&str, fn(&[&str]) -> BuiltinResult); 8] = [
    ("echo", echo),
    ("ls", ls),
    ("yes", yes),
    ("fucking_big_string", fucking_big_string),
    ("page_fault", page_fault),
    ("division_by_zero", division_by_zero),
    ("lspci", lspci),
    ("hello_world", hello_world),
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

    loop {
        block_read(&mut buf);
        let keysymb = buf[0];
        match keysymb {
            KeySymb::Return => {
                print!("{}", &line[cursor_pos..]);
                return line;
            }
            key if (key >= KeySymb::space) && (key <= KeySymb::asciitilde) => {
                line.insert(cursor_pos, key as u8 as char);
                print!("{}", &line[cursor_pos..]);
                cursor_pos += 1;

                print!("{}", CursorMove::Backward(line.len() - cursor_pos));
            }
            KeySymb::Left => {
                if cursor_pos > 0 {
                    cursor_pos -= 1;
                    print!("{}", CursorMove::Backward(1));
                }
            }
            KeySymb::Right => {
                if cursor_pos < line.len() {
                    cursor_pos += 1;
                    print!("{}", CursorMove::Forward(1));
                }
            }
            KeySymb::Delete => {
                if cursor_pos > 0 {
                    line.remove(cursor_pos - 1);
                    cursor_pos -= 1;
                    print!("{}", CursorMove::Backward(1));
                    if cursor_pos == line.len() {
                        print!("{}", " ");
                    } else {
                        print!("{}", &line[cursor_pos..]);
                        print!("{}", " ");
                    }
                    print!("{}", CursorMove::Backward(line.len() - cursor_pos + 1));
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
        print!("{}", CursorMove::Forward(0));
        // Call to blocked read_line function
        let line = read_line();
        // Make a line jump
        print!("\n");
        // Execute command
        exec_builtin(&line);
    }
}
