use ext2::{Errno, Ext2Filesystem, OpenFlags};
use std::fs::OpenOptions;
mod common;
use common::*;
use std::fs::DirBuilder;

#[test]
fn rmdir() {
    create_disk(1024 * 1024);
    let path = "simple_dir";
    mount_disk();
    {
        let path_mounted = DISK_MOUNTED_NAME.to_owned() + path;

        DirBuilder::new().create(&path_mounted).unwrap();
    }
    umount_disk();
    let f = OpenOptions::new()
        .read(true)
        .write(true)
        .open(DISK_NAME)
        .expect("open filesystem failed");
    let mut ext2 = Ext2Filesystem::new(f);
    ext2.rmdir(path).expect("rmdir failed");
}

const NB_TESTS: usize = 10;

#[test]
fn rmdir_multiple() {
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
    mount_disk();
    {
        for path in paths.iter() {
            let path_mounted = DISK_MOUNTED_NAME.to_owned() + path;

            DirBuilder::new().create(&path_mounted).unwrap();
        }
    }
    umount_disk();
    for path in paths.iter() {
        eprintln!("free: {:?}", path);
        ext2.rmdir(path).expect("rmdir failed");
        assert_eq!(
            open_ext2(&path, OpenFlags::O_RDWR).unwrap_err(),
            Errno::Enoent
        );
    }
}
