mod common;
use common::*;

use ext2::OpenFlags;
use std::fs::File;

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

    open_ext2(filename, OpenFlags::O_CREAT | OpenFlags::O_RDWR).expect("create file failed");
    open_ext2(filename, OpenFlags::O_RDWR).expect("open just created file failed");

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
    let paths: Vec<String> = (0..NB_TESTS)
        .map(|i| format!("simple_file, {}", i))
        .collect();
    for path in paths.iter() {
        open_ext2(&path, OpenFlags::O_RDWR | OpenFlags::O_CREAT)
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
        open_ext2(&path, OpenFlags::O_RDWR).expect("open with ext2 failed");
    }
    debug_fs();
}
