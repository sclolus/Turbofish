//! Main of tests

#![feature(process_exitcode_placeholder)]

mod ext2_filesystem;
use ext2_filesystem::Ext2Filesystem;

use std::fs::OpenOptions;

#[allow(dead_code)]
fn find_string(path: &str, patern: &[u8]) {
    let data = std::fs::read(path).unwrap();
    for i in 0..data.len() - patern.len() {
        if &data[i..i + patern.len()] == patern {
            println!("match");
            // dbg!(i);
        }
    }
}

use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    let f = OpenOptions::new().write(true).read(true).open(&args[1]).unwrap();
    let ext2 = Ext2Filesystem::new(f);
    dbg!(ext2);

    // let (inode, _) = ext2.find_inode(2);
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
    ExitCode::SUCCESS
}
