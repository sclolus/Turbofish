use crate::taskmaster::drivers::{
    BiosInt13hInstance, DiskDriver, DiskFileOperation, DiskWrapper, TtyDevice,
};
use alloc::format;
use alloc::sync::Arc;
use core::convert::TryFrom;
use fallible_collections::FallibleArc;
use sync::DeadMutex;

use super::*;
use crate::drivers::storage::{BlockIo, DiskDriverType, NbrSectors, Sector, BIOS_INT13H};
use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;
use ext2::Ext2Filesystem;
use mbr::Mbr;

pub static mut EXT2: Option<Ext2Filesystem> = None;

fn init_sda(_vfs: &mut Vfs, driver: DiskDriverType) {
    log::info!("Active disk driver: {:?}", driver);

    let size_read = NbrSectors(1);
    let mut v1: Vec<u8> = vec![0; size_read.into()];

    match driver {
        DiskDriverType::Bios => {
            unsafe {
                BIOS_INT13H
                    .as_mut()
                    .unwrap()
                    .read(Sector(0x0), size_read, v1.as_mut_ptr())
                    .expect("bios read failed");
            }

            let mut a = [0; 512];
            for (i, elem) in a.iter_mut().enumerate() {
                *elem = v1[i];
            }
            let mbr = unsafe { Mbr::new(&a) };
            let disk = DiskFileOperation::new(
                BiosInt13hInstance,
                mbr.parts[0].start as u64 * 512,
                mbr.parts[0].size as u64 * 512,
            );

            let mut disk_driver = DiskDriver::new(Arc::new(DeadMutex::new(disk)));

            let file_operation = disk_driver
                .open()
                .expect("disk driver open failed")
                .expect("disk driver open failed");
            let ext2_disk = DiskWrapper(file_operation);
            unsafe {
                EXT2 = Some(
                    //TODO: remove the box
                    Ext2Filesystem::new(Box::new(ext2_disk)).expect("ext2 filesystem new failed"),
                );
            }
        }
        DiskDriverType::Ide => {
            // TODO: handle Ide
            unimplemented!();
        }
        _ => unimplemented!(),
    }
    //TODO: mount on /dev/sda
    // log::info!("/dev/sda initialized");
}

fn init_tty(vfs: &mut Vfs) {
    let mode = FilePermissions::from_bits(0o777).expect("file permission creation failed");

    let flags = OpenFlags::O_CREAT | OpenFlags::O_DIRECTORY;
    let path = Path::try_from("/dev").expect("path creation failed");

    let mut current = Current {
        cwd: DirectoryEntryId::new(2),
        uid: 0,
        euid: 0,
        gid: 0,
        egid: 0,
        open_fds: BTreeMap::new(),
    };
    // println!("{}", path);
    vfs.open(&mut current, path, flags, mode)
        .expect("/dev creation failed");
    for i in 1..=4 {
        // C'est un exemple, le ou les FileOperation peuvent aussi etre alloues dans le new() ou via les open()
        let driver = Arc::try_new(DeadMutex::new(TtyDevice::try_new(i).unwrap())).unwrap();
        // L'essentiel pour le vfs c'est que j'y inscrive un driver attache a un pathname
        let path =
            Path::try_from(format!("/dev/tty{}", i).as_ref()).expect("path tty creation failed");
        let mode = FilePermissions::from_bits(0o777).expect("file permission creation failed");

        vfs.new_driver(&mut current, path, mode, driver)
            .expect("failed to add new driver tty to vfs");
    }
    log::info!("vfs initialized");
}

lazy_static! {
    pub static ref VFS: DeadMutex<Vfs> = DeadMutex::new(init());
}

pub fn init() -> Vfs {
    let mut vfs = Vfs::new().expect("vfs initialisation failed");
    init_tty(&mut vfs);
    init_sda(&mut vfs, DiskDriverType::Bios);
    vfs
}
