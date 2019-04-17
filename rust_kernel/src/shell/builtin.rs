use crate::drivers::{ACPI, PCI};
use crate::system::i8086_payload_apm_shutdown;
use core::time::Duration;
use keyboard::{KeyMap, KEYBOARD_DRIVER, PS2_CONTROLER};

/// Halt the PC
pub fn halt(_args: &[&str]) -> u8 {
    unsafe {
        asm!("cli" :::: "volatile");
        println!("System is now halted.");
        asm!("hlt" :::: "volatile");
    }
    unreachable!();
}

/// shutdown the PC
pub fn reboot(_args: &[&str]) -> u8 {
    match *ACPI.lock() {
        Some(mut acpi) => match acpi.reboot_computer() {
            Ok(_) => {}
            Err(e) => {
                log::error!("ACPI reboot failure: {:?}. Trying with PS/2 controler ...", e);
                unsafe {
                    PS2_CONTROLER.reboot_computer();
                }
            }
        },
        None => unsafe { PS2_CONTROLER.reboot_computer() },
    }
    1
}

/// shutdown the PC
pub fn shutdown(_args: &[&str]) -> u8 {
    match *ACPI.lock() {
        Some(mut acpi) => match unsafe { acpi.shutdown() } {
            Ok(_) => {}
            Err(e) => {
                log::error!("ACPI shudown failure: {:?}. Trying with APM ...", e);
                match i8086_payload_apm_shutdown() {
                    Ok(_) => {}
                    Err(e) => log::error!("APM shutdown error: {:?}", e),
                }
            }
        },
        None => match i8086_payload_apm_shutdown() {
            Ok(_) => {}
            Err(e) => log::error!("APM shutdown error: {:?}", e),
        },
    }
    log::error!("shutdown failure ... it is very disapointing ...");
    1
}

/// show backtrace
pub fn backtrace(_args: &[&str]) -> u8 {
    #[cfg(not(test))]
    unsafe {
        let ebp: *const u32;
        asm!("mov eax, ebp" : "={eax}"(ebp) : : : "intel");
        crate::panic::trace_back((*ebp.add(1), *ebp as *const u32));
    }
    0
}

/// put a butterfly on memory
pub fn butterfly(args: &[&str]) -> u8 {
    if args.len() != 2 {
        println!("usage: butterfly [mem_location in hex] [value in hex]");
    } else {
        match usize::from_str_radix(args[0], 16) {
            Ok(v) => {
                let p: *mut u8 = v as *mut u8;
                match usize::from_str_radix(args[1], 16) {
                    Ok(v) => {
                        unsafe {
                            *p = v as u8;
                        }
                        return 0;
                    }
                    Err(e) => println!("cannot parse args[1]: {:?}", e),
                }
            }
            Err(e) => println!("cannot parse args[0]: {:?}", e),
        }
    }
    1
}

/// simple, basic
pub fn echo(args: &[&str]) -> u8 {
    for s in args {
        print!("{} ", s);
    }
    print!("\n");
    0
}

/// a posix system without the yes command isn't a posix system (vcombey)
pub fn yes(args: &[&str]) -> u8 {
    loop {
        if args.len() == 0 {
            println!("y");
        } else {
            println!("{}", args[0]);
        }
    }
}

/// list all files
pub fn ls(_args: &[&str]) -> u8 {
    println!("fuck you !");
    0
}

/// enumerate all pci devices
pub fn lspci(_args: &[&str]) -> u8 {
    PCI.lock().list_pci_devices();
    0
}

/// display a big fucking string n times
pub fn fucking_big_string(args: &[&str]) -> u8 {
    let nb = match args.len() {
        0 => Ok(1),
        _ => args[0].parse(),
    };
    match nb {
        Err(e) => println!("{}", e),
        Ok(n) => crate::test_helpers::fucking_big_string(n),
    };
    0
}

/// select a keyboard layout
pub fn layout(args: &[&str]) -> u8 {
    if args.len() != 1 {
        println!("usage: layout [en/us || fr]");
    } else {
        match args[0] {
            "fr" => unsafe { KEYBOARD_DRIVER.as_mut().unwrap().keymap = KeyMap::Fr },
            "en" | "us" => unsafe { KEYBOARD_DRIVER.as_mut().unwrap().keymap = KeyMap::En },
            _ => println!("unknown keymap !"),
        }
    }
    0
}

/// display a very lazy hello world
pub fn hello_world(_args: &[&str]) -> u8 {
    crate::test_helpers::really_lazy_hello_world(Duration::from_millis(100));
    0
}

/// page_fault fail test
pub fn page_fault(_args: &[&str]) -> u8 {
    let toto: *mut u8 = 0x42424242 as *mut u8;
    unsafe {
        *toto = 0x42;
    }
    0
}

/// division by zero fail test
pub fn division_by_zero(_args: &[&str]) -> u8 {
    let toto: usize = 1;
    let w = 42 / (toto - 1) as usize;
    w as u8
}

pub fn fish(_args: &[&str]) -> u8 {
    crate::test_helpers::fish();
    0
}

pub fn more_fish(_args: &[&str]) -> u8 {
    crate::test_helpers::fish2();
    0
}