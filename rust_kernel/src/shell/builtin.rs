pub fn echo(args: &[&str]) {
    for s in args {
        print!("{} ", s);
    }
    print!("\n");
}
