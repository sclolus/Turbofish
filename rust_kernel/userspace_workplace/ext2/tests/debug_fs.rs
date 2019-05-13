use ext2::ext2_filesystem::{Ext2Filesystem, IoResult, OpenFlags};
use std::fs::File;
mod common;
use common::*;

// #[test]
fn debug_fs() {
    let f = File::open(DISK_NAME).expect("open filesystem failed");
    let mut ext2 = Ext2Filesystem::new(f);
    let mut ext2_clone = ext2.try_clone().unwrap();
    for entry in ext2.iter_entries(2).expect("iter entries failed") {
        dbg!(entry);
        dbg!(ext2_clone.find_inode(entry.0.inode).unwrap());
    }
}
