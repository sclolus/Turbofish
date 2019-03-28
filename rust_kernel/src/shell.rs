mod builtin;
use crate::drivers::keyboard::keysymb::KeySymb;
use crate::terminal::TERMINAL;
use alloc::string::String;
use alloc::vec::Vec;
use builtin::*;

// ASCII mouse
const PROMPT: &str = "----{,_,\"> $ ";

fn block_read(buf: &mut [KeySymb]) {
    unsafe {
        while TERMINAL.as_mut().unwrap().read(buf) == 0 {
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

fn exec_builtin(line: &str) {
    // println!("\nexec builtin {}", line);

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

fn read_line() -> String {
    let mut line = String::new();
    let mut cursor_pos = 0;
    let mut buf: [KeySymb; 1] = [KeySymb::nul; 1];

    loop {
        block_read(&mut buf);
        let keysymb = buf[0];
        match keysymb {
            KeySymb::Return => {
                return line;
            }
            key if (key >= KeySymb::space) && (key <= KeySymb::asciitilde) => {
                line.insert(cursor_pos, key as u8 as char);
                print!("{}", &line[cursor_pos..]);
                cursor_pos += 1;

                //unsafe {
                //    TERMINAL.as_mut().unwrap().move_cursor(CursorDirection::Left, line.len() - cursor_pos).unwrap()
                //};
            }
            KeySymb::Left => {
                if cursor_pos > 0 {
                    cursor_pos -= 1;
                    //unsafe { TERMINAL.as_mut().unwrap().move_cursor(CursorDirection::Left, 1).unwrap() };
                }
            }
            KeySymb::Right => {
                if cursor_pos < line.len() {
                    cursor_pos += 1;
                    //unsafe { TERMINAL.as_mut().unwrap().move_cursor(CursorDirection::Right, 1).unwrap() };
                }
            }
            KeySymb::Delete => {
                if cursor_pos > 0 {
                    line.remove(cursor_pos - 1);
                    cursor_pos -= 1;
                    //unsafe { TERMINAL.as_mut().unwrap().move_cursor(CursorDirection::Left, 1).unwrap() };
                    if cursor_pos == line.len() {
                        print!("{}", " ");
                    } else {
                        print!("{}", &line[cursor_pos..]);
                        print!("{}", " ");
                    }
                    //unsafe {
                    //    TERMINAL
                    //        .as_mut()
                    //        .unwrap()
                    //        .move_cursor(CursorDirection::Left, line.len() - cursor_pos + 1)
                    //        .unwrap()
                    //
                    //};
                }
            }
            _ => {}
        };
    }
}

pub fn shell() {
    loop {
        print!("{}", PROMPT);
        //unsafe {
        //TERMINAL.as_mut().unwrap().move_cursor(CursorDirection::Left, 0).unwrap();
        //}
        let line = read_line();
        print!("\n");
        exec_builtin(&line);
    }
}
