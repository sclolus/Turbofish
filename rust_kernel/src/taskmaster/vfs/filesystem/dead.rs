use super::FileSystem;
use super::SysResult;
use super::{DirectoryEntry, InodeData, VfsError, VfsResult};
use alloc::vec::Vec;
use libc_binding::{gid_t, uid_t, Errno, FileType, OpenFlags};

#[derive(Debug)]
pub struct DeadFileSystem;

impl FileSystem for DeadFileSystem {
    fn lookup_directory(&self, _inode_nbr: u32) -> VfsResult<Vec<(DirectoryEntry, InodeData)>> {
        Err(VfsError::Errno(Errno::ENOSYS))
    }
    fn root(&self) -> VfsResult<(DirectoryEntry, InodeData)> {
        Err(VfsError::Errno(Errno::ENOSYS))
    }
    fn chmod(&self, _inode_nbr: u32, _mode: FileType) -> VfsResult<()> {
        Err(VfsError::Errno(Errno::ENOSYS))
    }
    fn chown(&self, _inode_nbr: u32, _owner: uid_t, _group: gid_t) -> VfsResult<()> {
        Err(VfsError::Errno(Errno::ENOSYS))
    }
    fn unlink(&self, _dir_inode_nbr: u32, _name: &str) -> VfsResult<()> {
        Err(VfsError::Errno(Errno::ENOSYS))
    }
    fn create(
        &mut self,
        _filename: &str,
        _parent_inode_nbr: u32,
        _flags: OpenFlags,
        _mode: FileType,
    ) -> VfsResult<(DirectoryEntry, InodeData)> {
        Err(VfsError::Errno(Errno::ENOSYS))
    }
    fn write(&mut self, _inode_number: u32, _offset: &mut u64, _buf: &[u8]) -> SysResult<u32> {
        Err(Errno::ENOSYS)
    }
    fn read(&mut self, _inode_number: u32, _offset: &mut u64, _buf: &mut [u8]) -> SysResult<u32> {
        Err(Errno::ENOSYS)
    }
}
