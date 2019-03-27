pub fn echo(args: &[&str]) {
    for s in args {
        print!("{} ", s);
    }
    print!("\n");
}

pub fn yes(_args: &[&str]) {
    loop {
        println!("y");
    }
}

pub fn fucking_big_string(args: &[&str]) {
    let nb = args[0].parse();
    match nb {
        Err(e) => println!("{}", e),
        Ok(n) => crate::test_helpers::fucking_big_string(n),
    }
}
