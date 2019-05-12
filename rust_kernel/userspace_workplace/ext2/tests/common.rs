use ext2::ext2_filesystem::{Ext2Filesystem, IoResult, OpenFlags};
use std::fs::{File, OpenOptions};
use std::process::Command;

pub const DISK_NAME: &str = "disk";
pub const DISK_MOUNTED_NAME: &str = "disk_mounted/";

pub fn exec_shell(cmd: &str) {
    let exit_code = Command::new("bash")
        .args(&["-c"])
        .args(&[cmd])
        .status()
        .unwrap();
    if !exit_code.success() {
        eprintln!("command failed while crating disk: {}", cmd);
    }
}

pub fn create_disk(size: usize) {
    exec_shell(&format!("sync"));
    exec_shell(&format!("losetup -D"));
    exec_shell(&format!("mkdir -p {}", DISK_MOUNTED_NAME));
    exec_shell(&format!("umount {}", DISK_MOUNTED_NAME));
    exec_shell(&format!(
        "dd if=/dev/zero of={} bs=1024 count={}",
        DISK_NAME,
        size / 1024
    ));
    exec_shell(&format!("mkfs.ext2 {}", DISK_NAME));
    exec_shell("sync");
}

pub fn mount_disk() {
    exec_shell(&format!("sync"));
    exec_shell(&format!("loop=$(losetup -f)"));
    exec_shell(&format!("losetup -fP {}", DISK_NAME));
    exec_shell(&format!("mount {} {}", DISK_NAME, DISK_MOUNTED_NAME));
}

pub fn umount_disk() {
    exec_shell(&format!("sync"));
    exec_shell(&format!("losetup -D $loop"));
    exec_shell(&format!("sync"));
    exec_shell(&format!("umount {}", DISK_MOUNTED_NAME));
}

#[allow(dead_code)]
pub const DIRECT_MAX_SIZE: usize = 12 * 1024;
#[allow(dead_code)]
pub const SINGLY_MAX_SIZE: usize = DIRECT_MAX_SIZE + (1024 / 4) * 1024;
#[allow(dead_code)]
pub const DOUBLY_MAX_SIZE: usize = SINGLY_MAX_SIZE + (1024 / 4) * (1024 / 4) * 1024;

#[allow(dead_code)]
pub fn read_ext2(filename: &str, buf: &mut [u8]) -> usize {
    let f = File::open(DISK_NAME).expect("open filesystem failed");
    let mut ext2 = Ext2Filesystem::new(f);
    let mut file = ext2
        .open(filename, OpenFlags::ReadWrite)
        .expect("open on filesystem failed");

    ext2.read(&mut file, buf)
        .expect("read on filesystem failed")
}

#[allow(dead_code)]
pub fn write_ext2(filename: &str, buf: &[u8]) -> usize {
    let f = OpenOptions::new()
        .read(true)
        .write(true)
        .open(DISK_NAME)
        .expect("open filesystem failed");
    let mut ext2 = Ext2Filesystem::new(f);
    let mut file = ext2
        .open(filename, OpenFlags::ReadWrite)
        .expect("open on filesystem failed");
    ext2.write(&mut file, buf)
        .expect("write on filesystem failed")
}

#[allow(dead_code)]
pub fn open_ext2(path: &str, open_flags: OpenFlags) -> IoResult<ext2::ext2_filesystem::File> {
    let f = OpenOptions::new()
        .read(true)
        .write(true)
        .open(DISK_NAME)
        .expect("open filesystem failed");
    let mut ext2 = Ext2Filesystem::new(f);
    ext2.open(path, open_flags)
}
