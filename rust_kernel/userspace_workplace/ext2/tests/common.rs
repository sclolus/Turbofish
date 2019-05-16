#![allow(dead_code)]
//! to run tests, to prevent multithreading bugs, you have to run:
//! sudo RUST_TEST_TASKS=1 RUST_TEST_THREADS=1 cargo  test

use ext2::{Ext2Filesystem, IoResult, OpenFlags};
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

pub const DIRECT_MAX_SIZE: usize = 12 * 1024;
pub const SINGLY_MAX_SIZE: usize = DIRECT_MAX_SIZE + (1024 / 4) * 1024;
pub const DOUBLY_MAX_SIZE: usize = SINGLY_MAX_SIZE + (1024 / 4) * (1024 / 4) * 1024;

pub fn read_ext2(filename: &str, buf: &mut [u8]) -> usize {
    let f = File::open(DISK_NAME).expect("open filesystem failed");
    let mut ext2 = Ext2Filesystem::new(f);
    let mut file = ext2
        .open(filename, OpenFlags::O_RDWR, 0o644)
        .expect("open on filesystem failed");

    ext2.read(&mut file, buf)
        .expect("read on filesystem failed") as usize
}

pub fn write_ext2(filename: &str, buf: &[u8]) -> usize {
    let f = OpenOptions::new()
        .read(true)
        .write(true)
        .open(DISK_NAME)
        .expect("open filesystem failed");
    let mut ext2 = Ext2Filesystem::new(f);
    let mut file = ext2
        .open(filename, OpenFlags::O_RDWR, 0o644)
        .expect("open on filesystem failed");
    ext2.write(&mut file, buf)
        .expect("write on filesystem failed") as usize
}

pub fn open_ext2(path: &str, open_flags: OpenFlags) -> IoResult<ext2::File> {
    let f = OpenOptions::new()
        .read(true)
        .write(true)
        .open(DISK_NAME)
        .expect("open filesystem failed");
    let mut ext2 = Ext2Filesystem::new(f);
    ext2.open(path, open_flags, 0o644)
}

pub fn debug_fs() {
    let f = File::open(DISK_NAME).expect("open filesystem failed");
    let mut ext2 = Ext2Filesystem::new(f);
    let mut ext2_clone = ext2.try_clone().unwrap();
    for entry in ext2.iter_entries(2).expect("iter entries failed") {
        dbg!(entry);
        dbg!(ext2_clone.get_inode(entry.0.get_inode()).unwrap());
    }
}
