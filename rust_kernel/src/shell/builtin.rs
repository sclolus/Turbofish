pub fn echo(args: &[&str]) {
    for s in args {
        print!("{} ", s);
    }
    print!("\n");
}

pub fn yes(args: &[&str]) {
    loop {
        if args[0] == "" {
            println!("y");
        } else {
            println!("{}", args[0]);
        }
    }
}

pub fn ls(_args: &[&str]) {
    println!("fuck you !");
}

pub fn fucking_big_string(args: &[&str]) {
    let nb = args[0].parse();
    match nb {
        Err(e) => println!("{}", e),
        Ok(n) => crate::test_helpers::fucking_big_string(n),
    }
}
