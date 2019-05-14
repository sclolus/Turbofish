mod common;
use common::*;
use rand::prelude::*;
use std::fs::File;
use std::io::Read;

fn write_of_size(size: usize) {
    //create a disk of size of the file + a little space for metadata
    create_disk(size + 1024 * 1024 * 10);
    let filename = "simple_write";

    /* CREATE with the std */
    mount_disk();
    {
        let filename_mounted = DISK_MOUNTED_NAME.to_owned() + "/" + filename;
        File::create(&filename_mounted).expect(&format!(
            "open on mouted filesystem failed {}",
            &filename_mounted
        ));
    }
    umount_disk();

    /* WRITE with the ext2 */
    let v: Vec<u8> = (0..(size)).map(|_| random::<u8>()).collect();
    let count = write_ext2(filename, &v);
    assert_eq!(count, size);

    /* READ with the std */
    let buf = {
        mount_disk();
        let filename_mounted = DISK_MOUNTED_NAME.to_owned() + filename;
        let mut f = File::open(&filename_mounted).expect(&format!(
            "open on mouted filesystem failed {}",
            &filename_mounted
        ));
        let mut buf = vec![42; size];
        f.read_exact(&mut buf).expect("read with std failed");
        buf
    };
    umount_disk();
    assert_eq!(buf[..], v[..]);
}

#[test]
fn write() {
    let sizes = &[
        42,
        DIRECT_MAX_SIZE + 42,
        SINGLY_MAX_SIZE + 42,
        DOUBLY_MAX_SIZE + 42,
    ];
    for size in sizes {
        write_of_size(*size);
    }
}
