mod common;
use common::*;

use ext2::{Ext2Filesystem, OpenFlags};
use std::fs::File;
use std::fs::OpenOptions;

#[test]
fn create_file() {
    create_disk(1024 * 1024 * 10);
    let filename = "banane";
    let filename_mounted = DISK_MOUNTED_NAME.to_owned() + "/" + filename;
    // mount_disk();
    // {
    //     File::create(&filename_mounted).expect(&format!(
    //         "open on mouted filesystem failed {}",
    //         &filename_mounted
    //     ));
    // }
    // umount_disk();

    open_ext2(filename, OpenFlags::CREAT | OpenFlags::READWRITE).expect("create file failed");
    open_ext2(filename, OpenFlags::READWRITE).expect("open just created file failed");

    debug_fs();
    mount_disk();
    {
        File::open(&filename_mounted).expect("open std failed");
    }
    umount_disk();
}

const NB_TESTS: usize = 10;

#[test]
fn create_mutltiple_file() {
    create_disk(1024 * 1024 * 10);
    let f = OpenOptions::new()
        .read(true)
        .write(true)
        .open(DISK_NAME)
        .expect("open filesystem failed");
    let ext2 = Ext2Filesystem::new(f);

    let paths: Vec<String> = (0..NB_TESTS)
        .map(|i| format!("simple_file, {}", i))
        .collect();
    for path in paths.iter() {
        open_ext2(&path, OpenFlags::READWRITE | OpenFlags::CREAT)
            .expect("open OCreate with ext2 failed");
    }
    mount_disk();
    {
        for path in paths.iter() {
            File::open(DISK_MOUNTED_NAME.to_owned() + "/" + &path).expect("open std failed");
        }
    }
    umount_disk();
    for path in paths.iter() {
        open_ext2(&path, OpenFlags::READWRITE).expect("open with ext2 failed");
    }
    debug_fs();
}
