use super::{DirectoryEntry, VfsError, VfsResult};
use super::{DirectoryEntryBuilder, Filename, InodeData, InodeId, SysResult};
use alloc::vec::Vec;
use core::fmt::Debug;
use libc_binding::{gid_t, uid_t, FileType, OpenFlags};

pub mod dead;
pub use dead::DeadFileSystem;
pub mod ext2fs;
pub use ext2fs::Ext2fs;

pub trait FileSystem: Send + Debug {
    // fn name(&self) -> &str;
    // fn load_inode(&self, inode_number: InodeNumber) -> VfsResult<Inode>;
    /// return all the directory entry and inode present in the inode_nbr
    fn lookup_directory(&self, inode_nbr: u32) -> VfsResult<Vec<(DirectoryEntry, InodeData)>>;
    /// return the (possibly virtual) directory entry and inode of the root
    fn root(&self) -> VfsResult<(DirectoryEntry, InodeData)>;
    fn chmod(&self, inode_nbr: u32, mode: FileType) -> VfsResult<()>;
    fn chown(&self, inode_nbr: u32, owner: uid_t, group: gid_t) -> VfsResult<()>;
    fn unlink(&self, dir_inode_nbr: u32, name: &str) -> VfsResult<()>;
    fn create(
        &mut self,
        filename: &str,
        parent_inode_nbr: u32,
        flags: OpenFlags,
        mode: FileType,
    ) -> VfsResult<(DirectoryEntry, InodeData)>;
    fn write(&mut self, inode_number: u32, offset: &mut u64, buf: &[u8]) -> SysResult<u32>;
    fn read(&mut self, inode_number: u32, offset: &mut u64, buf: &mut [u8]) -> SysResult<u32>;
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
