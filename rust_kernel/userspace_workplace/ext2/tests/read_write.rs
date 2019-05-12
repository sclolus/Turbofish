mod common;
use common::*;
use ext2::ext2_filesystem::OpenFlags;
use rand::prelude::*;
use std::fs::File;

fn read_write_of_size(size: usize) {
    //create a disk of size of the file + a little space for metadata
    let filename = format!("simple_file, {}", random::<usize>());

    open_ext2(&filename, OpenFlags::ReadWrite | OpenFlags::Creat).expect("open with ext2 failed");
    // CREATE with the std
    // mount_disk();
    // {
    //     let filename_mounted = DISK_MOUNTED_NAME.to_owned() + "/" + &filename;
    //     File::create(&filename_mounted).expect(&format!(
    //         "open on mouted filesystem failed {}",
    //         &filename_mounted
    //     ));
    // }
    // umount_disk();

    /* WRITE with the ext2 */
    let v: Vec<u8> = (0..(size)).map(|_| random::<u8>()).collect();
    let count = write_ext2(&filename, &v);
    assert_eq!(count, size);

    /* READ with the ext2 */
    let mut buf = vec![42; size];
    let count = read_ext2(&filename, &mut buf);
    assert_eq!(count, size);

    assert_eq!(buf[..], v[..]);
}

const NB_TESTS: usize = 10;

#[test]
fn read_write() {
    let sizes: Vec<usize> = (0..NB_TESTS)
        .map(|_| random::<usize>() % 10) //DOUBLY_MAX_SIZE)
        .collect();
    let sum: usize = sizes.iter().sum();
    create_disk(sum + 1024 * 1024 * 10);
    for size in sizes {
        read_write_of_size(size);
    }
}
