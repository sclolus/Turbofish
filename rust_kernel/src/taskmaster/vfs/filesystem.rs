use super::{DirectoryEntry, SysResult};
use super::{DirectoryEntryBuilder, Filename, InodeData, InodeId, Path};
use alloc::vec::Vec;
use core::fmt::Debug;
use libc_binding::{gid_t, statfs, uid_t, utimbuf, Errno, FileType};

pub mod dead;
pub use dead::DeadFileSystem;
pub mod ext2fs;
pub use ext2fs::Ext2fs;

pub trait FileSystem: Send + Debug {
    // fn name(&self) -> &str;
    // fn load_inode(&self, inode_number: InodeNumber) -> SysResult<Inode>;
    /// return all the directory entry and inode present in the inode_nbr
    fn lookup_directory(&mut self, _inode_nbr: u32) -> SysResult<Vec<(DirectoryEntry, InodeData)>> {
        Err(Errno::ENOSYS)
    }

    /// return the (possibly virtual) directory entry and inode of the root
    fn root(&self) -> SysResult<(DirectoryEntry, InodeData)> {
        Err(Errno::ENOSYS)
    }

    fn chmod(&self, _inode_nbr: u32, _mode: FileType) -> SysResult<()> {
        Err(Errno::ENOSYS)
    }

    fn chown(&self, _inode_nbr: u32, _owner: uid_t, _group: gid_t) -> SysResult<()> {
        Err(Errno::ENOSYS)
    }

    fn unlink(&self, _dir_inode_nbr: u32, _name: &str, _free_inode_data: bool) -> SysResult<()> {
        Err(Errno::ENOSYS)
    }

    fn remove_inode(&mut self, _inode_nbr: u32) -> SysResult<()> {
        Err(Errno::ENOSYS)
    }

    fn truncate(&mut self, _inode_nbr: u32, _new_size: u64) -> SysResult<()> {
        Err(Errno::ENOSYS)
    }

    fn create(
        &mut self,
        _filename: &str,
        _parent_inode_nbr: u32,
        _mode: FileType,
        (_owner, _group): (uid_t, gid_t),
    ) -> SysResult<(DirectoryEntry, InodeData)> {
        Err(Errno::ENOSYS)
    }

    fn write(&mut self, _inode_number: u32, _offset: &mut u64, _buf: &[u8]) -> SysResult<(u32, InodeData)> {
        Err(Errno::ENOSYS)
    }

    fn read(&mut self, _inode_number: u32, _offset: &mut u64, _buf: &mut [u8]) -> SysResult<u32> {
        Err(Errno::ENOSYS)
    }

    fn create_dir(
        &mut self,
        _parent_inode_nbr: u32,
        _filename: &str,
        _mode: FileType,
        (_owner, _group): (uid_t, gid_t),
    ) -> SysResult<(DirectoryEntry, InodeData)> {
        Err(Errno::ENOSYS)
    }

    fn rmdir(&mut self, _parent_inode_nbr: u32, _filename: &str) -> SysResult<()> {
        Err(Errno::ENOSYS)
    }

    fn symlink(
        &mut self,
        _parent_inode_nbr: u32,
        _target: &str,
        _filename: &str,
    ) -> SysResult<(DirectoryEntry, InodeData)> {
        Err(Errno::ENOSYS)
    }

    fn link(
        &mut self,
        _parent_inode_nbr: u32,
        _target_inode_nbr: u32,
        _filename: &str,
    ) -> SysResult<DirectoryEntry> {
        Err(Errno::ENOSYS)
    }

    fn rename(
        &mut self,
        _parent_inode_nbr: u32,
        _filename: &str,
        _new_parent_inode_nbr: u32,
        _new_filename: &str,
    ) -> SysResult<()> {
        Err(Errno::ENOSYS)
    }

    fn statfs(&self, _buf: &mut statfs) -> SysResult<()> {
        Err(Errno::ENOSYS)
    }

    fn utime(&mut self, _inode_number: u32, _times: Option<&utimbuf>) -> SysResult<()> {
        Err(Errno::ENOSYS)
    }
    // fn lookup: Option<fn(&mut Superblock)>,
    // fn create: Option<fn(&mut Superblock)>,
    // fn unlink: Option<fn(&mut Superblock)>,
    // fn link: Option<fn(&mut Superblock)>,
    // fn symlink: Option<fn(&mut Superblock)>,
    // fn statfs: Option<fn(&mut Superblock)>,
    // fn mkdir: Option<fn(&mut Superblock)>,
    // fn rmdir: Option<fn(&mut Superblock)>,
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Default, Eq, PartialEq)]
pub struct FileSystemId(pub usize);

impl FileSystemId {
    pub fn new(id: usize) -> Self {
        Self(id)
    }
}

impl core::ops::Add<usize> for FileSystemId {
    type Output = Self;
    fn add(self, rhs: usize) -> Self::Output {
        Self(self.0 + rhs)
    }
}
