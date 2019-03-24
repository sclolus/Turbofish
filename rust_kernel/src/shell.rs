use crate::drivers::keyboard::keysymb::KeySymb;
use crate::terminal::TERMINAL;
mod builtin;
use alloc::prelude::*;
use builtin::echo;

const PROMPT: &str = "$>";

fn block_read(buf: &mut [KeySymb]) {
    unsafe {
        while TERMINAL.as_mut().unwrap().read(buf) == 0 {
            asm!("hlt" :::: "volatile");
        }
    }
}

const BUILTINS: [(&str, fn(&[&str])); 1] = [("echo", echo)];

fn exec_builtin(line: &str) {
    // println!("\nexec builtin {}", line);

    let mut split = line.split_whitespace();
    let command = split.next().unwrap_or("");
    let args: Vec<&str> = split.collect();
    match BUILTINS.iter().find(|(c, _)| c == &command) {
        None => println!("{}: command not found", command),
        Some((_c, f)) => f(args.as_slice()),
    };
}

fn read_line() -> Vec<KeySymb> {
    let mut line = Vec::new();
    // let cursor_pos = 0;
    let mut buf: [KeySymb; 1] = [KeySymb::nul; 1];

    loop {
        block_read(&mut buf);
        let keysymb = buf[0];
        if keysymb == KeySymb::Return {
            return line;
        }
        if (keysymb >= KeySymb::space) && (keysymb <= KeySymb::asciitilde) {
            line.push(keysymb);
            print!("{}", keysymb as u32 as u8 as char);
        }
    }
}

pub fn shell() {
    loop {
        print!("{}", PROMPT);
        let line = read_line();
        let line_str: String = line.into_iter().map(|x| x as u8 as char).collect();
        print!("\n");
        exec_builtin(&line_str);
    }
}
