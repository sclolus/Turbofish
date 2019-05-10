use ext2::ext2_filesystem::Ext2Filesystem;
use std::fs::{File, OpenOptions};
use std::process::Command;

pub const DISK_NAME: &str = "disk";
pub const DISK_MOUNTED_NAME: &str = "disk_mounted/";

pub fn exec_shell(cmd: &str) {
    let exit_code = Command::new("bash")
        .args(&["-c"])
        .args(&[cmd])
        .status()
        .unwrap();
    if !exit_code.success() {
        eprintln!("command failed while crating disk: {}", cmd);
    }
}

pub fn create_disk(size: usize) {
    exec_shell(&format!("sync"));
    exec_shell(&format!("losetup -D"));
    exec_shell(&format!("mkdir -p {}", DISK_MOUNTED_NAME));
    exec_shell(&format!("umount {}", DISK_MOUNTED_NAME));
    exec_shell(&format!(
        "dd if=/dev/zero of={} bs=1024 count={}",
        DISK_NAME,
        size / 1024
    ));
    exec_shell(&format!("mkfs.ext2 {}", DISK_NAME));
    // exec_shell("echo bonjour");
}

pub fn mount_disk() {
    exec_shell(&format!("sync"));
    exec_shell(&format!("loop=$(losetup -f)"));
    exec_shell(&format!("losetup -fP {}", DISK_NAME));
    exec_shell(&format!("mount {} {}", DISK_NAME, DISK_MOUNTED_NAME));
}

pub fn umount_disk() {
    exec_shell(&format!("sync"));
    exec_shell(&format!("losetup -D $loop"));
    exec_shell(&format!("sync"));
    exec_shell(&format!("umount {}", DISK_MOUNTED_NAME));
}

pub const DIRECT_MAX_SIZE: usize = 12 * 1024;
pub const SINGLY_MAX_SIZE: usize = DIRECT_MAX_SIZE + (1024 / 4) * 1024;
pub const DOUBLY_MAX_SIZE: usize = SINGLY_MAX_SIZE + (1024 / 4) * (1024 / 4) * 1024;

#[allow(dead_code)]
pub fn read_ext2(filename: &str, buf: &mut [u8]) -> usize {
    let f = File::open(DISK_NAME).expect("open filesystem failed");
    let mut ext2 = Ext2Filesystem::new(f);
    let mut file = ext2.open(filename).expect("open on filesystem failed");

    ext2.read(&mut file, buf)
        .expect("read on filesystem failed")
}

#[allow(dead_code)]
pub fn write_ext2(filename: &str, buf: &[u8]) -> usize {
    let f = OpenOptions::new()
        .read(true)
        .write(true)
        .open(DISK_NAME)
        .expect("open filesystem failed");
    let mut ext2 = Ext2Filesystem::new(f);
    let mut file = ext2.open(filename).expect("open on filesystem failed");
    ext2.write(&mut file, buf)
        .expect("write on filesystem failed")
}

// println!("{:#?}", file);

// println!("READ");
// let mut buf = [42; 1024];
// let count = ext2.read(&mut file, &mut buf).unwrap();
// unsafe { println!("string: {}", core::str::from_utf8_unchecked(&buf[0..count])) };

// let mut buf_std = [42; 1024];
// let mut f = OpenOptions::new().write(true).read(true).open("./simple_disk_mounted/".to_owned() + filename).unwrap();
// let count_std = f.read(&mut buf_std).unwrap();
// assert_eq!(count, count_std);
// assert_eq!(buf[..], buf_std[..]);

// dbg!(inode);
// let dir_entry = ext2.find_entry(&inode, 0);
// dbg!(dir_entry);
// for e in ext2.try_clone().unwrap().iter_entries(&inode).unwrap().skip(2) {
//     dbg!(e.get_filename());
//     let (inode, _) = ext2.find_inode(e.inode);
//     println!("{:?}", inode);
//     println!("inner");
//     for e in ext2.iter_entries(&inode).unwrap().skip(2) {
//         dbg!(e.get_filename());
//         dbg!(e);
//     }
//     println!("end inner");
// }
// let mut file = ext2.open("dir/banane").unwrap();
// println!("{:#?}", file);

// println!("READ");
// let mut buf = [42; 10];
// let count = ext2.read(&mut file, &mut buf).unwrap();
// unsafe {
//     println!("string: {}", core::str::from_utf8_unchecked(&buf[0..count]));
// }

// file.seek(SeekFrom::Start(0));
// println!("WRITE");
// let s = "123456789a".repeat(1000);
// ext2.write(&mut file, &s.as_bytes()).expect("write failed");

// file.seek(SeekFrom::Start(0));
// println!("READ");
// let mut buf = [42; 10000];
// let count = ext2.read(&mut file, &mut buf).unwrap();
// unsafe {
//     println!("string: {}", core::str::from_utf8_unchecked(&buf[0..count]));
// }

// let mut file = ext2.open("dir/indirect").unwrap();
// println!("{:#?}", file);
// let mut buf = [42; 1024];
// let mut indirect_dump = StdFile::create("indirect_dump").unwrap();
// while {
//     let x = ext2.read(&mut file, &mut buf).unwrap();
//     indirect_dump.write(&buf[0..x]).unwrap();
//     x > 0
// } {}
// let mut file = ext2.open("dir/doubly_indirect").unwrap();
// println!("{:#?}", file);
// let mut buf = [42; 10];
// let mut indirect_dump = StdFile::create("doubly_indirect_dump").unwrap();
// while {
//     let x = ext2.read(&mut file, &mut buf).unwrap();
//     indirect_dump.write(&buf[0..x]).unwrap();
//     x > 0
// } {}
// let mut file = ext2.open("dir/triply_indirect").unwrap();
// println!("{:#?}", file);
// let mut buf = [42; 1024];
// let mut indirect_dump = StdFile::create("triply_indirect_dump").unwrap();
// while {
//     let x = ext2.read(&mut file, &mut buf).unwrap();
//     indirect_dump.write(&buf[0..x]).unwrap();
//     x > 0
// } {}
// while let Some(x) = ext2.alloc_block() {
//     dbg!(x);
// }
// dbg!(count);

// assert!(ext2.open("dir/artichaud").is_err());
// find_string("simple_diskp1", "lescarotessontcuites".as_bytes());
