use crate::drivers::pci::PCI;

pub type BuiltinResult = core::result::Result<usize, ()>;

/// simple, basic
pub fn echo(args: &[&str]) -> BuiltinResult {
    for s in args {
        print!("{} ", s);
    }
    print!("\n");
    Ok(0)
}

/// a posix system without the yes command isn't a posix system (vcombey)
pub fn yes(args: &[&str]) -> BuiltinResult {
    loop {
        if args.len() == 0 {
            println!("y");
        } else {
            println!("{}", args[0]);
        }
    }
}

/// list all files
pub fn ls(_args: &[&str]) -> BuiltinResult {
    println!("fuck you !");
    Ok(0)
}

/// enumerate all pci devices
pub fn lspci(_args: &[&str]) -> BuiltinResult {
    PCI.lock().list_pci_devices();
    Ok(0)
}

/// display a big fucking string n times
pub fn fucking_big_string(args: &[&str]) -> BuiltinResult {
    let nb = match args.len() {
        0 => Ok(1),
        _ => args[0].parse(),
    };
    match nb {
        Err(e) => println!("{}", e),
        Ok(n) => crate::test_helpers::fucking_big_string(n),
    }
    Ok(0)
}

use crate::drivers::keyboard::{KeyMap, KEYBOARD_DRIVER};

/// select a keyboard layout
pub fn layout(args: &[&str]) -> BuiltinResult {
    if args.len() != 1 {
        println!("usage: layout [en/us || fr]");
    } else {
        match args[0] {
            "fr" => unsafe { KEYBOARD_DRIVER.as_mut().unwrap().keymap = KeyMap::Fr },
            "en" | "us" => unsafe { KEYBOARD_DRIVER.as_mut().unwrap().keymap = KeyMap::En },
            _ => println!("unknown keymap !"),
        }
    }
    Ok(0)
}

/// display a very lazy hello world
pub fn hello_world(_args: &[&str]) -> BuiltinResult {
    crate::test_helpers::really_lazy_hello_world();
    Ok(0)
}

/// page_fault fail test
pub fn page_fault(_args: &[&str]) -> BuiltinResult {
    let toto: *mut u8 = 0x42424242 as *mut u8;
    unsafe {
        *toto = 0x42;
    }
    Ok(0)
}

/// division by zero fail test
pub fn division_by_zero(_args: &[&str]) -> BuiltinResult {
    let toto: usize = 1;
    let w = 42 / (toto - 1) as usize;
    Ok(w)
}
