use super::filesystem::devfs::{
    BiosInt13hInstance, DiskDriver, DiskWrapper, IdeAtaInstance, NullDevice, TtyDevice, ZeroDevice,
};
use super::filesystem::{Devfs, Ext2fs};
use super::SmartMutex;
use crate::taskmaster::drivers::Driver;
use alloc::format;
use alloc::sync::Arc;
use core::convert::TryFrom;
use fallible_collections::vec::FallibleVec;
use fallible_collections::{FallibleArc, FallibleBox};
use libc_binding::OpenFlags;
use sync::DeadMutex;

use super::filesystem::procfs::ProcFs;
use super::*;
use crate::drivers::storage::{BlockIo, DiskDriverType, NbrSectors, Sector};
use alloc::boxed::Box;
use ext2::Ext2Filesystem;
use mbr::Mbr;

lazy_static! {
    pub static ref VFS: SmartMutex<Vfs> = SmartMutex::new(init());
}

/// init the vfs
pub fn init() -> Vfs {
    let mut vfs = Vfs::new().expect("vfs initialisation failed");
    // we start by bootstraping ext2
    let fs_id = FileSystemId(2);
    let mut devfs = Devfs::new(fs_id);

    init_ext2(&mut vfs, &mut devfs, DiskDriverType::Ide);
    init_procfs(&mut vfs).expect("Failed to init /proc (procfs)");
    // then init tty on /dev/tty
    init_tty(&mut devfs);
    mount_devfs(&mut vfs, devfs, fs_id);
    vfs
}

fn mount_devfs(vfs: &mut Vfs, mut devfs: Devfs, fs_id: FileSystemId) {
    let root_creds = Credentials::ROOT;

    let mode = FileType::from_bits(0o777).expect("file permission creation failed")
        | FileType::CHARACTER_DEVICE;

    let inode_id = devfs.gen_inode_id();
    devfs
        .add_driver(
            Filename::try_from("null").expect("path sda creation failed"),
            mode,
            Box::new(NullDevice::try_new(inode_id).expect("null device creation failed")),
            inode_id,
        )
        .expect("failed to add new driver sda to devfs");

    let inode_id = devfs.gen_inode_id();
    devfs
        .add_driver(
            Filename::try_from("zero").expect("path sda creation failed"),
            mode,
            Box::new(ZeroDevice::try_new(inode_id).expect("zero device creation failed")),
            inode_id,
        )
        .expect("failed to add new driver sda to devfs");
    let dev_id = vfs
        .pathname_resolution(&Path::root(), &root_creds, &Path::try_from("/dev").unwrap())
        .unwrap();
    vfs.mount_filesystem(
        Arc::try_new(DeadMutex::new(devfs)).expect("arc new ext2fs failed"),
        fs_id,
        dev_id,
    )
    .expect("mounting /dev failed");
}

/// bootstrap the ext2 and construct /dev/sda
fn init_ext2(vfs: &mut Vfs, devfs: &mut Devfs, driver_type: DiskDriverType) {
    log::info!("Active disk driver: {:?}", driver_type);
    let (sda_driver, mut partition_drivers) =
        new_disk_drivers(driver_type).expect("initialisation of disk drivers failed");

    let file_operation = partition_drivers[0]
        .open(OpenFlags::O_RDWR)
        .expect("open sda1 failed")
        .expect("disk driver open failed");

    let ext2_disk = DiskWrapper(file_operation);
    let ext2 = Ext2Filesystem::new(Box::new(ext2_disk)).expect("ext2 filesystem new failed");
    let fs_id = FileSystemId(0);
    let ext2fs = Ext2fs::new(ext2, fs_id);
    vfs.mount_filesystem(
        Arc::try_new(DeadMutex::new(ext2fs)).expect("arc new ext2fs failed"),
        fs_id,
        DirectoryEntryId::new(2),
    )
    .expect("mount filesystem failed");

    init_sda(devfs, sda_driver, partition_drivers);
}

/// mount /dev/sda1 on the vfs, WARNING: must be call after ext2 is
/// mounted on root
fn init_sda(
    devfs: &mut Devfs,
    sda_driver: Box<dyn Driver>,
    partition_drivers: Vec<Box<dyn Driver>>,
) {
    // let path = Path::try_from(format!("/dev/sda").as_ref()).expect("path sda creation failed");
    let mode = FileType::from_bits(0o777).expect("file permission creation failed")
        | FileType::CHARACTER_DEVICE;
    let inode_id = devfs.gen_inode_id();
    devfs
        .add_driver(
            Filename::try_from("sda").expect("path sda creation failed"),
            mode,
            sda_driver,
            inode_id,
        )
        .expect("failed to add new driver sda to devfs");
    for (i, d) in partition_drivers.into_iter().enumerate() {
        let filename = Filename::try_from(format!("sda{}", i + 1).as_ref())
            .expect("filename sda_i creation failed");
        let mode = FileType::from_bits(0o777).expect("file permission creation failed")
            | FileType::CHARACTER_DEVICE;
        let inode_id = devfs.gen_inode_id();
        devfs
            .add_driver(filename, mode, d, inode_id)
            .expect("failed to add new driver sda1 to devfs");
    }
    log::info!("/dev/sda initialized");
}

// TODO: make a Initer abstraction that takes a &mut of Vfs.
fn init_procfs(vfs: &mut Vfs) -> Result<(), Errno> {
    const PROCFS_ROOT: &str = "/proc";

    log::info!("Creating ProcFs");

    let procfs_root = Path::try_from(PROCFS_ROOT)?;
    let fs_id = FileSystemId(1);

    let procfs = ProcFs::new(fs_id)?;

    let root_creds = Credentials::ROOT;
    let cwd = Path::try_from("/")?;

    let ret = vfs.pathname_resolution(&cwd, &root_creds, &procfs_root);
    let procfs_dir_perms = FileType::from_bits(0555).ok_or(Errno::EINVAL)?;

    let proc_dir_directory_id = match ret {
        Err(Errno::ENOENT) => {
            vfs.mkdir(
                &cwd,
                &root_creds,
                // Well, we need ownership here, we should decide what to do about this.
                procfs_root.try_clone()?,
                procfs_dir_perms,
            )?;
            vfs.pathname_resolution(&cwd, &root_creds, &procfs_root)?
        }
        Err(e) => return Err(e),
        Ok(id) => id,
    };

    assert_eq!(
        2,
        vfs.opendir(&cwd, &root_creds, procfs_root.try_clone()?)?
            .len()
    );

    vfs.mount_filesystem(
        Arc::try_new(DeadMutex::new(procfs))?,
        fs_id,
        proc_dir_directory_id,
    )
}

/// create device /dev/tty on the vfs, WARNING: must be call after
/// ext2 is mounted on root
fn init_tty(devfs: &mut Devfs) {
    for i in 1..=4 {
        let inode_id = devfs.gen_inode_id();
        // C'est un exemple, le ou les FileOperation peuvent aussi etre alloues dans le new() ou via les open()
        let driver = Box::try_new(TtyDevice::try_new(i, inode_id).unwrap()).unwrap();
        // L'essentiel pour le vfs c'est que j'y inscrive un driver attache a un pathname
        let filename =
            Filename::try_from(format!("tty{}", i).as_ref()).expect("path tty creation failed");
        let mode = FileType::from_bits(0o777).expect("file permission creation failed");

        devfs
            .add_driver(filename, mode, driver, inode_id)
            .expect("failed to add new driver tty to vfs");
        // vfs.new_driver(&Path::root(), &Credentials::ROOT, path, mode, driver)
    }
    log::info!("vfs initialized");
}

/// read the mbr form a disk
fn read_mbr(disk: &mut dyn BlockIo) -> Mbr {
    let size_read = NbrSectors(1);
    let mut v1 = [0; 512];

    disk.read(Sector(0x0), size_read, v1.as_mut_ptr())
        .expect("MBR read failed");
    unsafe { Mbr::new(&v1) }
}

/// returns the sda driver and sda1,2,.. drivers
fn new_disk_drivers(
    driver_type: DiskDriverType,
) -> SysResult<(Box<dyn Driver>, Vec<Box<dyn Driver>>)> {
    fn _new_disk_drivers<D: BlockIo + Copy + Clone + Debug + 'static>(
        mut disk: D,
        disk_size: u64,
    ) -> SysResult<(Box<dyn Driver>, Vec<Box<dyn Driver>>)> {
        let mbr = read_mbr(&mut disk);
        let sda = Box::try_new(DiskDriver::new(disk, 0, disk_size))?;
        let mut drivers: Vec<Box<dyn Driver>> = Vec::new();
        for part in &mbr.parts {
            if part.is_used() {
                drivers.try_push(Box::try_new(DiskDriver::new(
                    disk,
                    part.start as u64 * 512,
                    part.size as u64 * 512,
                ))?)?
            }
        }
        Ok((sda, drivers))
    }

    match driver_type {
        DiskDriverType::Bios => {
            let disk = BiosInt13hInstance;
            let disk_size = disk.disk_size();
            _new_disk_drivers(disk, disk_size)
        }
        DiskDriverType::Ide => {
            let disk_size = {
                // TODO: how to know the size with an IdeAta
                let disk = BiosInt13hInstance;
                disk.disk_size()
            };
            let disk = IdeAtaInstance;
            _new_disk_drivers(disk, disk_size)
        }
        _ => unimplemented!(),
    }
}
