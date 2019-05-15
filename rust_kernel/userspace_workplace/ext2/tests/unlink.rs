use ext2::{Errno, Ext2Filesystem, IoResult, OpenFlags};
use std::fs::OpenOptions;
mod common;
use common::*;
use std::fs::DirBuilder;

#[test]
fn unlink() {
    create_disk(1024 * 1024);
    let path = "simple_file";
    let f = OpenOptions::new()
        .read(true)
        .write(true)
        .open(DISK_NAME)
        .expect("open filesystem failed");
    let mut ext2 = Ext2Filesystem::new(f);
    open_ext2(&path, OpenFlags::READWRITE | OpenFlags::CREAT).expect("open with ext2 failed");
    ext2.unlink(&path).expect("unlink failed");
    assert_eq!(
        open_ext2(&path, OpenFlags::READWRITE).unwrap_err(),
        Errno::Enoent
    );
}

const NB_TESTS: usize = 10;

#[test]
fn unlink_multiple() {
    create_disk(1024 * 1024);
    let f = OpenOptions::new()
        .read(true)
        .write(true)
        .open(DISK_NAME)
        .expect("open filesystem failed");
    let mut ext2 = Ext2Filesystem::new(f);

    let paths: Vec<String> = (0..NB_TESTS)
        .map(|i| format!("simple_file, {}", i))
        .collect();
    for path in paths.iter() {
        open_ext2(&path, OpenFlags::READWRITE | OpenFlags::CREAT).expect("open with ext2 failed");
    }
    let mut ext2_clone = ext2.try_clone().unwrap();
    for entry in ext2.iter_entries(2).expect("iter entries failed") {
        dbg!(entry);
        dbg!(ext2_clone.get_inode(entry.0.get_inode()).unwrap());
    }
    for path in paths.iter() {
        eprintln!("free: {:?}", path);
        ext2.unlink(&path).expect("unlink failed");
        assert_eq!(
            open_ext2(&path, OpenFlags::READWRITE).unwrap_err(),
            Errno::Enoent
        );
    }
}
