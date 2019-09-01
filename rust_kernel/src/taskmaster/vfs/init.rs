use crate::taskmaster::drivers::{
    BiosInt13hInstance, DiskDriver, DiskFileOperation, DiskWrapper, IdeAtaInstance, TtyDevice,
};
use alloc::format;
use alloc::sync::Arc;
use core::convert::TryFrom;
use fallible_collections::FallibleArc;
use sync::DeadMutex;

use super::*;
use crate::drivers::storage::{BlockIo, DiskDriverType, NbrSectors, Sector};
use alloc::boxed::Box;
use ext2::Ext2Filesystem;
use mbr::Mbr;

pub static mut EXT2: Option<Ext2Filesystem> = None;

fn read_mbr(disk: &dyn BlockIo) -> Mbr {
    let size_read = NbrSectors(1);
    let mut v1 = [0; 512];

    disk.read(Sector(0x0), size_read, v1.as_mut_ptr())
        .expect("bios read failed");

    let mut a = [0; 512];
    for (i, elem) in a.iter_mut().enumerate() {
        *elem = v1[i];
    }
    unsafe { Mbr::new(&a) }
}

fn init_sda(vfs: &mut Vfs, driver: DiskDriverType) {
    log::info!("Active disk driver: {:?}", driver);

    let disk_driver = match driver {
        DiskDriverType::Bios => {
            let disk = BiosInt13hInstance;
            let mbr = read_mbr(&disk);
            let disk = DiskFileOperation::new(
                disk,
                mbr.parts[0].start as u64 * 512,
                mbr.parts[0].size as u64 * 512,
            );
            DiskDriver::new(Arc::new(DeadMutex::new(disk)))
        }
        DiskDriverType::Ide => {
            let disk = IdeAtaInstance;
            let mbr = read_mbr(&disk);
            let disk = DiskFileOperation::new(
                disk,
                mbr.parts[0].start as u64 * 512,
                mbr.parts[0].size as u64 * 512,
            );
            DiskDriver::new(Arc::new(DeadMutex::new(disk)))
        }
        _ => unimplemented!(),
    };

    let mut current = Current {
        cwd: DirectoryEntryId::new(2),
        uid: 0,
        euid: 0,
        gid: 0,
        egid: 0,
        open_fds: BTreeMap::new(),
    };
    let path = Path::try_from(format!("/dev/sda1").as_ref()).expect("path sda1 creation failed");
    let mode = FilePermissions::from_bits(0o777).expect("file permission creation failed");
    vfs.new_driver(
        &mut current,
        path.clone(),
        mode,
        Arc::new(DeadMutex::new(disk_driver)),
    )
    .expect("failed to add new driver sda1 to vfs");

    let flags = libc_binding::OpenFlags::O_RDWR;

    let file_operation = vfs
        .open(&mut current, path, flags, mode)
        .expect("open sda1 failed")
        .expect("disk driver open failed");

    let ext2_disk = DiskWrapper(file_operation);
    unsafe {
        EXT2 = Some(
            //TODO: remove the box
            Ext2Filesystem::new(Box::new(ext2_disk)).expect("ext2 filesystem new failed"),
        );
    }
    log::info!("/dev/sda initialized");
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
    init_sda(&mut vfs, DiskDriverType::Ide);
    vfs
}
