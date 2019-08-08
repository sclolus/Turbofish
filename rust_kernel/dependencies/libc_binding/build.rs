use std::process::Command;

fn main() {
    // panic!("my pwd is{}", env!("PWD"));
    // panic!("cargo manifest dir is{}",);
    // let res = Command::new(format!("{}/{}", env!("CARGO_MANIFEST_DIR"), "build.sh"))
    //     .output()
    //     .unwrap();
    // if !res.status.success() {
    //     panic!("{:?}", res);
    // }
    let out = Command::new("./bindgen.sh")
        .arg("all_includes.h")
        .output()
        .unwrap();
    if !out.status.success() {
        panic!("{:?}", String::from_utf8(out.stderr));
    }
    std::fs::write("src/libc.rs", out.stdout).unwrap();
}
