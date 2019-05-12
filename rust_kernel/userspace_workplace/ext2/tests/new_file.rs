mod common;
use common::*;

use ext2::ext2_filesystem::{Ext2Filesystem, OpenFlags};
use std::fs::{File, OpenOptions};

#[test]
fn create_file() {
    create_disk(1024 * 1024 * 10);
    let filename = "banane";
    let filename_mounted = DISK_MOUNTED_NAME.to_owned() + "/" + filename;

    {
        let f = OpenOptions::new()
            .read(true)
            .write(true)
            .open(DISK_NAME)
            .expect("open filesystem failed");
        let mut ext2 = Ext2Filesystem::new(f);
        ext2.open(filename, OpenFlags::Creat | OpenFlags::ReadWrite)
            .expect("open on filesystem failed");
        let f = ext2
            .open(filename, OpenFlags::ReadWrite)
            .expect("open with ext2 failed");
        dbg!(f);
    }

    mount_disk();
    {
        File::open(&filename_mounted).unwrap();
    }
    umount_disk();
}
