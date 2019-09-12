use super::{DirectoryEntry, SysResult};
use super::{DirectoryEntryBuilder, Filename, InodeData, InodeId, Path};
use alloc::vec::Vec;
use core::fmt::Debug;
use libc_binding::{gid_t, uid_t, Errno, FileType, OpenFlags};

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

    fn unlink(&self, _dir_inode_nbr: u32, _name: &str) -> SysResult<()> {
        Err(Errno::ENOSYS)
    }

    fn create(
        &mut self,
        _filename: &str,
        _parent_inode_nbr: u32,
        _flags: OpenFlags,
        _mode: FileType,
    ) -> SysResult<(DirectoryEntry, InodeData)> {
        Err(Errno::ENOSYS)
    }

    fn write(&mut self, _inode_number: u32, _offset: &mut u64, _buf: &[u8]) -> SysResult<u32> {
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
    // fn link: Option<fn(&mut Superblock)>,
    // fn statfs: Option<fn(&mut Superblock)>,
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
