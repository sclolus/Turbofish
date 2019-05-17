use ext2::OpenFlags;
mod common;
use common::*;
use std::fs::read_dir;

#[test]
fn mkdir_simple() {
    create_disk(1024 * 1024);
    let path = "simple_dir";
    let mut ext2 = new_ext2_readable_writable();
    ext2.mkdir(path, 0o644).expect("mkdir failed");
    mount_disk();
    {
        let path_mounted = DISK_MOUNTED_NAME.to_owned() + path;

        read_dir(&path_mounted).expect("read dir failed");
    }
    umount_disk();
}

const NB_TESTS: usize = 10;

#[test]
fn mkdir_multiple() {
    create_disk(1024 * 1024);
    let mut ext2 = new_ext2_readable_writable();
    let paths: Vec<String> = (0..NB_TESTS)
        .map(|i| format!("simple_file, {}", i))
        .collect();
    for path in paths.iter() {
        ext2.mkdir(path, 0o644).expect("mkdir failed");
        open_ext2(&path, OpenFlags::O_RDWR).unwrap();
    }
    mount_disk();
    {
        for path in paths.iter() {
            let path_mounted = DISK_MOUNTED_NAME.to_owned() + path;

            read_dir(&path_mounted).expect("read dir failed");
        }
    }
    umount_disk();
}
