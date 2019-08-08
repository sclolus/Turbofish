use std::process::Command;

fn main() {
    // panic!("my pwd is{}", env!("PWD"));
    // panic!("cargo manifest dir is{}",);
    let res = Command::new(format!("{}/{}", env!("CARGO_MANIFEST_DIR"), "build.sh"))
        .output()
        .unwrap();
    if !res.status.success() {
        panic!("{:?}", res);
    }
    // panic!("bonjour");
    // std::fs::write("src/libc.rs", res.stdout).unwrap();
}
