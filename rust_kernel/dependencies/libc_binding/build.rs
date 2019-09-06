use std::process::Command;

fn main() {
    println!(r#"cargo:rerun-if-changed={}"#, "../../../libc/include");
    println!(r#"cargo:rerun-if-changed={}"#, "../../../libc/include/sys");
    let out = Command::new("./bindgen.sh").output().unwrap();

    if !out.status.success() {
        panic!("{:?}", unsafe {
            core::str::from_utf8_unchecked(&out.stdout)
        });
    }
}
