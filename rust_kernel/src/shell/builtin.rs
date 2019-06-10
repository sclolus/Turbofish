use crate::drivers::{storage::ext2::EXT2, ACPI, PCI};
use crate::system::i8086_payload_apm_shutdown;
use crate::interrupts;
use alloc::format;
use alloc::string::String;
use core::time::Duration;
use ext2::syscall::OpenFlags;
use keyboard::{KeyMap, KEYBOARD_DRIVER, PS2_CONTROLER};

/// Halt the PC
pub fn halt(_args: &[&str]) -> u8 {
    unsafe {
        interrupts::disable();
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

use crate::Spinlock;
use lazy_static::lazy_static;

lazy_static! {
    /// current working directory
    pub static ref CWD: Spinlock<String> = Spinlock::new(String::from("/"));
}

/// list all files
pub fn ls(_args: &[&str]) -> u8 {
    match unsafe { EXT2.as_mut() } {
        None => {
            println!("ext2 not init");
            1
        }
        Some(ext2) => {
            let (_, dir_entry) = ext2.find_path(&*CWD.lock()).unwrap();
            for (e, _) in ext2.iter_entries(dir_entry.0.get_inode()).unwrap() {
                print!("{} ", unsafe { e.get_filename() });
            }
            print!("\n");
            0
        }
    }
}

pub fn cd(args: &[&str]) -> u8 {
    if args[0] == "" {
        return 1;
    }
    let new_cwd = format!("{}/{}", *CWD.lock(), args[0]);
    match unsafe { EXT2.as_mut() } {
        None => {
            println!("ext2 not init");
            1
        }
        Some(ext2) => match ext2.access(&new_cwd, 0o644) {
            Err(e) => {
                println!("{:?}", e);
                1
            }
            Ok(_) => {
                *CWD.lock() = new_cwd;
                0
            }
        },
    }
}

pub fn pwd(_args: &[&str]) -> u8 {
    let s: &str = &*CWD.lock();
    println!("{}", s);
    0
}

pub fn cat(args: &[&str]) -> u8 {
    pub fn cat_result(args: &[&str]) -> Result<(), &'static str> {
        let filename = format!("{}/{}", *CWD.lock(), args[0]);
        let ext2 = unsafe { EXT2.as_mut().ok_or("ext2 not init")? };
        let mut file = dbg!(ext2.open(dbg!(&filename), OpenFlags::O_RDONLY, 0)).map_err(|_| "open failed")?;
        let buf: &mut [u8; 1024] = &mut [0; 1024];

        loop {
            let size_read = ext2.read(&mut file, &mut buf[..]).map_err(|_| "read failed")?;
            if size_read == 0 {
                return Ok(());
            }
            print!("{}", core::str::from_utf8(&buf[0..size_read as usize]).map_err(|_| "not valid utf8")?);
        }
    }
    match cat_result(args) {
        Ok(()) => 0,
        Err(e) => {
            println!("{}", e);
            1
        }
    }
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

pub fn stack_overflow(_args: &[&str]) -> u8 {
    fn dummy_factorial(n: usize) -> usize {
        if n == 0 {
            return 1;
        }
        n.wrapping_mul(dummy_factorial(n - 1))
    }
    dummy_factorial(100000000) as u8
}
