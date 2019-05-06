fn main() {
    println!("&main: {:?}", main as fn() as *const u8);
     println!("hello main");
}
