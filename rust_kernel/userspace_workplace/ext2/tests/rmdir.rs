use ext2::ext2_filesystem::Ext2Filesystem;
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

    let mut ext2_clone = ext2.try_clone().unwrap();
    for entry in ext2.iter_entries(2).expect("iter entries failed") {
        dbg!(entry);
        dbg!(ext2_clone.get_inode(entry.0.inode).unwrap());
    }
}
