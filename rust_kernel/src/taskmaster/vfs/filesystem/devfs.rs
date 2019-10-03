use super::super::inode::InodeNumber;
use super::super::tools::KeyGenerator;
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
use core::convert::TryFrom;
use core::sync::atomic::Ordering;
use fallible_collections::{btree::BTreeMap, FallibleBox, TryCollect};
use libc_binding::{statfs, time_t, FileType};

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
}

const ROOT_ID: InodeNumber = 2;

impl KeyGenerator<InodeNumber> for Devfs {
    fn gen_filter(&self, id: InodeNumber) -> bool {
        !(id == ROOT_ID)
            && !self
                .files
                .values()
                .any(|inode_data| inode_data.0.id.inode_number == id)
    }
}

/// the ext2 wrapper which implement filesystem
impl Devfs {
    pub fn new(fs_id: FileSystemId) -> Self {
        Self {
            fs_id,
            files: BTreeMap::new(),
        }
    }

    pub fn gen_inode_id(&mut self) -> InodeId {
        InodeId::new(self.gen(), Some(self.fs_id))
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

    fn statfs(&self, _buf: &mut statfs) -> SysResult<()> {
        unimplemented!()
        // let fs = self.ext2.lock();
        // let superblock = fs.get_superblock();

        // Ok(*buf = statfs {
        //     f_type: EXT2_SUPER_MAGIC,
        //     f_bsize: fs.get_block_size(), // Actually Depends on underlying implementation of Disk I/O.
        //     f_blocks: superblock.nbr_blocks,
        //     f_bfree: superblock.nbr_free_blocks,
        //     f_bavail: superblock.nbr_free_blocks, // is nbr_blocks_reserved counted in this or not?
        //     f_files: superblock.nbr_inode,
        //     f_ffree: superblock.nbr_free_inodes,
        //     f_fsid: self.fs_id.0 as u32, // consider method/Into<u32> implementation.
        //     f_namelen: NAME_MAX - 1,
        //     f_frsize: 1024 << superblock.log2_fragment_size,
        //     f_flags: 0, // TODO: For now this does not seem implementable.
        // })
    }
}
