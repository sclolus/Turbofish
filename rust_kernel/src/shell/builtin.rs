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
