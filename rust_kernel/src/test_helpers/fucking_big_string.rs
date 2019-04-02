pub fn fucking_big_string(count: u32) -> () {
    for _i in 0..count {
        println!("{}", include_str!("../terminal/monitor.rs"));
    }
}
