pub type BuiltinResult = core::result::Result<usize, ()>;

pub fn echo(args: &[&str]) -> BuiltinResult {
    for s in args {
        print!("{} ", s);
    }
    print!("\n");
    Ok(0)
}

pub fn yes(args: &[&str]) -> BuiltinResult {
    loop {
        if args.len() == 0 {
            println!("y");
        } else {
            println!("{}", args[0]);
        }
    }
}

pub fn ls(_args: &[&str]) -> BuiltinResult {
    println!("fuck you !");
    Ok(0)
}

pub fn fucking_big_string(args: &[&str]) -> BuiltinResult {
    let nb = args[0].parse();
    match nb {
        Err(e) => println!("{}", e),
        Ok(n) => crate::test_helpers::fucking_big_string(n),
    }
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
