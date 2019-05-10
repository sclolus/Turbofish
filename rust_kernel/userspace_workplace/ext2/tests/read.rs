mod common;
use common::*;
use rand::prelude::*;
use std::fs::File;
use std::io::Write;

fn read_of_size(size: usize) {
    //create a disk of size of the file + a little space for metadata
    create_disk(size + 1024 * 1024 * 10);
    mount_disk();

    let filename = "simple_read";

    /* WRITE with the std */
    let v = {
        let filename_mounted = DISK_MOUNTED_NAME.to_owned() + "/" + filename;
        let mut f = File::create(&filename_mounted).expect(&format!(
            "open on mouted filesystem failed {}",
            &filename_mounted
        ));
        // random bytes to write
        let v: Vec<u8> = (0..(size)).map(|_| random::<u8>()).collect();
        f.write_all(&v).expect("write on mouted filesystem failed");
        v
    };
    umount_disk();

    /* READ with the ext2 */
    let mut buf = vec![42; size];
    let count = read_ext2(filename, &mut buf);
    assert_eq!(count, size);

    umount_disk();
    assert_eq!(buf[..], v[..]);
}

#[test]
fn read() {
    let sizes = &[
        42,
        DIRECT_MAX_SIZE + 42,
        SINGLY_MAX_SIZE + 42,
        DOUBLY_MAX_SIZE + 42,
    ];
    for size in sizes {
        read_of_size(*size);
    }
}
