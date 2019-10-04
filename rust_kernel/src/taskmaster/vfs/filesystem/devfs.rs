use super::super::inode::InodeNumber;
use super::super::tools::{Incrementor, KeyGenerator};
use super::DefaultDriver;
use super::Driver;
use super::FileOperation;
use super::FileSystem;
use super::{get_file_op_uid, IpcResult};
use super::{DirectoryEntry, FileSystemId, InodeData};
use super::{DirectoryEntryBuilder, Filename, InodeId, SysResult};
use crate::taskmaster::kmodules::CURRENT_UNIX_TIME;
use alloc::boxed::Box;
use alloc::vec::Vec;
use core::convert::{TryFrom, TryInto};
use core::sync::atomic::Ordering;
use fallible_collections::{btree::BTreeMap, btree::BTreeSet, FallibleBox, TryCollect};
use libc_binding::{
    dev_t, gid_t, statfs, time_t, uid_t, Errno, FileType, DEVFS_SUPER_MAGIC, NAME_MAX, PAGE_SIZE,
};

pub mod tty;
pub use tty::TtyDevice;

pub mod null;
pub use null::{DevNull, NullDevice};

pub mod zero;
pub use zero::{DevZero, ZeroDevice};

pub mod random;
pub use random::{DevRandom, RandomDevice};

pub mod sda;
pub use sda::{BiosInt13hInstance, DiskDriver, DiskFileOperation, DiskWrapper, IdeAtaInstance};

#[derive(Debug)]
pub struct Devfs {
    fs_id: FileSystemId,
    files: BTreeMap<Filename, (InodeData, Option<Box<dyn Driver>>)>,
    tty_minors: BTreeSet<dev_t>,
}

const ROOT_ID: InodeNumber = 2;
const TTY_MAJOR: dev_t = 4;

impl KeyGenerator<InodeNumber> for Devfs {
    fn gen_filter(&self, id: InodeNumber) -> bool {
        !(id == ROOT_ID)
            && !self
                .files
                .values()
                .any(|inode_data| inode_data.0.id.inode_number == id)
    }
}

impl Incrementor for dev_t {
    fn incr(&mut self) {
        *self += 1;
    }
}

impl KeyGenerator<dev_t> for Devfs {
    fn gen_filter(&self, minor: dev_t) -> bool {
        !self.tty_minors.contains(&minor) && i32::try_from(minor).is_ok()
    }
}

/// the ext2 wrapper which implement filesystem
impl Devfs {
    pub fn new(fs_id: FileSystemId) -> Self {
        Self {
            fs_id,
            files: BTreeMap::new(),
            tty_minors: BTreeSet::new(),
        }
    }

    pub fn gen_inode_id(&mut self) -> InodeId {
        InodeId::new(self.gen(), Some(self.fs_id))
    }

    /// Tries to register a tty with a specific `minor`.
    pub fn register_specific_tty(
        &mut self,
        mut permissions: FileType,
        (owner, group): (uid_t, gid_t),
        minor: dev_t,
    ) -> SysResult<InodeId> {
        if self.tty_minors.contains(&minor) {
            return Err(Errno::EEXIST);
        }

        self._register_tty(permissions, (owner, group), minor)
    }

    fn _register_tty(
        &mut self,
        mut permissions: FileType,
        (owner, group): (uid_t, gid_t),
        minor: dev_t,
    ) -> SysResult<InodeId> {
        assert!(
            !permissions.is_typed(),
            "The permissions of a tty should be purely permissions bits"
        );

        permissions |= FileType::CHARACTER_DEVICE;

        // L'essentiel pour le vfs c'est que j'y inscrive un driver attache a un pathname

        let inode_id = self.gen_inode_id();

        let driver = Box::try_new(TtyDevice::try_new(
            minor
                .try_into()
                .expect("Should have generated a valid minor"),
            inode_id,
        )?)?;

        let filename = Filename::from_str_unwrap(tryformat!(48, "tty{}", minor as usize)?.as_str());

        let timestamp = unsafe { CURRENT_UNIX_TIME.load(Ordering::Relaxed) };
        let inode_data = InodeData {
            id: inode_id,
            major: TTY_MAJOR,
            minor: minor,
            link_number: 1,
            access_mode: permissions,

            uid: owner,
            gid: group,

            atime: timestamp as time_t,
            mtime: timestamp as time_t,
            ctime: timestamp as time_t,

            size: 0,
            nbr_disk_sectors: 0,
        };
        self.files
            .try_insert(filename, (inode_data, Some(driver)))?;
        self.tty_minors.try_insert(minor)?;
        Ok(inode_id)
    }

    pub fn register_tty(
        &mut self,
        mut permissions: FileType,
        (owner, group): (uid_t, gid_t),
    ) -> SysResult<InodeId> {
        let new_minor: dev_t = self.gen();

        self._register_tty(permissions, (owner, group), new_minor)
    }

    pub fn add_driver(
        &mut self,
        filename: Filename,
        filetype: FileType,
        driver: Box<dyn Driver>,
        inode_id: InodeId,
    ) -> SysResult<()> {
        let timestamp = unsafe { CURRENT_UNIX_TIME.load(Ordering::Relaxed) };
        let inode_data = InodeData {
            id: inode_id,
            major: 42,
            minor: 42,
            link_number: 1,
            access_mode: filetype,

            uid: 0,
            gid: 0,

            atime: timestamp as time_t,
            mtime: timestamp as time_t,
            ctime: timestamp as time_t,

            size: 0,
            nbr_disk_sectors: 0,
        };
        self.files
            .try_insert(filename, (inode_data, Some(driver)))?;
        Ok(())
    }
}

impl FileSystem for Devfs {
    fn root(&self) -> SysResult<(DirectoryEntry, InodeData, Box<dyn Driver>)> {
        let inode_id = InodeId::new(ROOT_ID, Some(self.fs_id));

        let direntry = {
            let mut builder = DirectoryEntryBuilder::new();
            builder
                .set_filename(Filename::try_from("devfsroot").unwrap())
                .set_inode_id(inode_id)
                .set_directory();
            builder.build()
        };

        let inode_data = InodeData {
            id: inode_id,
            major: 0,
            minor: 0,
            link_number: 1,
            access_mode: FileType::DIRECTORY | FileType::from_bits(0o777).unwrap(),

            uid: 0,
            gid: 0,

            atime: 0,
            mtime: 0,
            ctime: 0,

            size: 0,
            nbr_disk_sectors: 0,
        };
        Ok((direntry, inode_data, Box::try_new(DefaultDriver)?))
    }

    fn lookup_directory(
        &mut self,
        _inode_nbr: u32,
    ) -> SysResult<Vec<(DirectoryEntry, InodeData, Box<dyn Driver>)>> {
        // just returning all files in dev,
        Ok(self
            .files
            .iter_mut()
            .map(|(filename, (inode_data, driver))| {
                let inode_id = inode_data.id;
                let direntry = {
                    let mut builder = DirectoryEntryBuilder::new();
                    builder.set_filename(*filename).set_inode_id(inode_id);
                    builder.set_chardevice();
                    builder.build()
                };

                let driver: Box<dyn Driver> = driver.take().unwrap();
                (direntry, *inode_data, driver)
            })
            .try_collect()?)
    }

    fn statfs(&self, buf: &mut statfs) -> SysResult<()> {
        Ok(*buf = statfs {
            f_type: DEVFS_SUPER_MAGIC,
            f_bsize: PAGE_SIZE,
            f_blocks: 0,
            f_bfree: 0,
            f_bavail: 0,
            f_files: 0,
            f_ffree: 0,
            f_fsid: self.fs_id.0 as u32,
            f_namelen: NAME_MAX - 1,
            f_frsize: 0,
            f_flags: 0,
        })
    }
}
