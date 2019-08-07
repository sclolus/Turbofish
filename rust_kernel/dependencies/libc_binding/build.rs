use std::process::Command;

fn main() {
    println!("bonjour");
    let res = Command::new("./bindgen.sh")
        .args(&["/toolchain_turbofish/sysroot/all_includes.h"])
        .output()
        .unwrap();
    // panic!("{:?}", res.stdout);
    std::fs::write("src/libc.rs", res.stdout).unwrap();
}
