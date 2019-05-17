use ext2::{Errno, OpenFlags};
use rand::prelude::*;
use std::fs::File;
use std::io::Write;
mod common;
use common::*;

#[test]
fn unlink() {
    create_disk(1024 * 1024);
    let path = "simple_file";
    let mut ext2 = new_ext2_readable_writable();
    open_ext2(&path, OpenFlags::O_RDWR | OpenFlags::O_CREAT).expect("open with ext2 failed");
    ext2.unlink(&path).expect("unlink failed");
    assert_eq!(
        open_ext2(&path, OpenFlags::O_RDWR).unwrap_err(),
        Errno::Enoent
    );
}

const NB_TESTS: usize = 10;

#[test]
fn unlink_multiple() {
    create_disk(1024 * 1024);
    let mut ext2 = new_ext2_readable_writable();
    let paths: Vec<String> = (0..NB_TESTS)
        .map(|i| format!("simple_file, {}", i))
        .collect();
    for path in paths.iter() {
        open_ext2(&path, OpenFlags::O_RDWR | OpenFlags::O_CREAT).expect("open with ext2 failed");
    }
    for path in paths.iter() {
        eprintln!("free: {:?}", path);
        ext2.unlink(&path).expect("unlink failed");
        assert_eq!(
            open_ext2(&path, OpenFlags::O_RDWR).unwrap_err(),
            Errno::Enoent
        );
    }
}

#[test]
fn unlink_big_files() {
    fn unlink_of_size(size: usize) {
        //create a disk of size of the file + a little space for metadata
        create_disk(size + 1024 * 1024 * 10);
        let filename = "simple_write";

        /* CREATE with the std */
        mount_disk();
        {
            let filename_mounted = DISK_MOUNTED_NAME.to_owned() + "/" + filename;
            let mut file = File::create(&filename_mounted).expect(&format!(
                "open on mouted filesystem failed {}",
                &filename_mounted
            ));
            let v: Vec<u8> = (0..(size)).map(|_| random::<u8>()).collect();
            file.write_all(&v).expect("write failed");
        }
        umount_disk();

        let mut ext2 = new_ext2_readable_writable();
        ext2.unlink(&filename).expect("unlink failed");
        assert_eq!(
            open_ext2(&filename, OpenFlags::O_RDWR).unwrap_err(),
            Errno::Enoent
        );
    }

    let sizes = &[
        42,
        DIRECT_MAX_SIZE + 42,
        SINGLY_MAX_SIZE + 42,
        DOUBLY_MAX_SIZE + 42,
    ];
    for size in sizes {
        unlink_of_size(*size);
    }
}
