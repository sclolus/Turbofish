mod common;
use common::*;

use ext2::ext2_filesystem::OpenFlags;
use std::fs::File;

#[test]
fn create_file() {
    create_disk(1024 * 1024 * 10);
    let filename = "banane";
    let filename_mounted = DISK_MOUNTED_NAME.to_owned() + "/" + filename;

    open_ext2(filename, OpenFlags::Creat | OpenFlags::ReadWrite).expect("create file failed");
    open_ext2(filename, OpenFlags::ReadWrite).expect("open just created file failed");

    mount_disk();
    {
        File::open(&filename_mounted).expect("open std failed");
    }
    umount_disk();
}
