use super::filesystem::Ext2fs;
use crate::taskmaster::drivers::{
    BiosInt13hInstance, DiskDriver, DiskWrapper, Driver, IdeAtaInstance, TtyDevice,
};
use alloc::format;
use alloc::sync::Arc;
use core::convert::TryFrom;
use fallible_collections::boxed::FallibleBox;
use sync::DeadMutex;

use super::*;
use crate::drivers::storage::{BlockIo, DiskDriverType, NbrSectors, Sector};
use alloc::boxed::Box;
use ext2::Ext2Filesystem;
use mbr::Mbr;

/// read the mbr form a disk
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

/// mount /dev/sda1 on the vfs, WARNING: must be call after ext2 is
/// mounted on root
fn init_sda(vfs: &mut Vfs, disk_driver: Box<dyn Driver>) {
    let path = Path::try_from(format!("/dev/sda1").as_ref()).expect("path sda1 creation failed");
    let mode = FileType::from_bits(0o777).expect("file permission creation failed");
    vfs.new_driver(
        &Path::root(),
        &Credentials::ROOT,
        path.clone(),
        mode,
        disk_driver,
    )
    .expect("failed to add new driver sda1 to vfs");
}

/// bootstrap the ext2 and construct /dev/sda
fn init_ext2(vfs: &mut Vfs, driver: DiskDriverType) {
    log::info!("Active disk driver: {:?}", driver);

    let mut disk_driver: Box<dyn Driver> = match driver {
        DiskDriverType::Bios => {
            let disk = BiosInt13hInstance;
            let mbr = read_mbr(&disk);
            Box::new(DiskDriver::new(
                disk,
                mbr.parts[0].start as u64 * 512,
                mbr.parts[0].size as u64 * 512,
            ))
        }
        DiskDriverType::Ide => {
            let disk = IdeAtaInstance;
            let mbr = read_mbr(&disk);
            Box::new(DiskDriver::new(
                disk,
                mbr.parts[0].start as u64 * 512,
                mbr.parts[0].size as u64 * 512,
            ))
        }
        _ => unimplemented!(),
    };

    let file_operation = disk_driver
        .open()
        .expect("open sda1 failed")
        .expect("disk driver open failed");

    let ext2_disk = DiskWrapper(file_operation);
    let ext2 = Ext2Filesystem::new(Box::new(ext2_disk)).expect("ext2 filesystem new failed");
    let fs_id: FileSystemId = vfs.gen();
    let ext2fs = Ext2fs::new(ext2, fs_id);
    vfs.mount_filesystem(
        Arc::new(DeadMutex::new(ext2fs)),
        fs_id,
        DirectoryEntryId::new(2),
    )
    .expect("mount filesystem failed");

    // mount /dev/sda on the vfs
    init_sda(vfs, disk_driver);
    log::info!("/dev/sda initialized");
}

/// create device /dev/tty on the vfs, WARNING: must be call after
/// ext2 is mounted on root
fn init_tty(vfs: &mut Vfs) {
    for i in 1..=4 {
        // C'est un exemple, le ou les FileOperation peuvent aussi etre alloues dans le new() ou via les open()
        let driver = Box::try_new(TtyDevice::try_new(i).unwrap()).unwrap();
        // L'essentiel pour le vfs c'est que j'y inscrive un driver attache a un pathname
        let path =
            Path::try_from(format!("/dev/tty{}", i).as_ref()).expect("path tty creation failed");
        let mode = FileType::from_bits(0o777).expect("file permission creation failed");

        vfs.new_driver(&Path::root(), &Credentials::ROOT, path, mode, driver)
            .expect("failed to add new driver tty to vfs");
    }
    log::info!("vfs initialized");
}

lazy_static! {
    pub static ref VFS: DeadMutex<Vfs> = DeadMutex::new(init());
}

/// init the vfs
pub fn init() -> Vfs {
    let mut vfs = Vfs::new().expect("vfs initialisation failed");
    // we start by bootstraping ext2
    init_ext2(&mut vfs, DiskDriverType::Bios);
    // then init tty on /dev/tty
    init_tty(&mut vfs);
    vfs
}
