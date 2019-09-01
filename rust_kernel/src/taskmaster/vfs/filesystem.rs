use super::{DirectoryEntry, Inode, VfsResult};
use alloc::sync::Arc;
use alloc::vec::Vec;
use ext2::Ext2Filesystem;
use sync::DeadMutex;

pub trait FileSystem: Send {
    // fn name(&self) -> &str;
    // fn load_inode(&self, inode_number: InodeNumber) -> VfsResult<Inode>;
    fn lookup_directory(&self, inode_nbr: u32) -> VfsResult<Vec<(DirectoryEntry, Inode)>>;
    fn root(&self) -> VfsResult<(DirectoryEntry, Inode)>;
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

pub struct Ext2fs {
    ext2: Arc<DeadMutex<Ext2Filesystem>>,
    fs_id: FileSystemId,
}

impl Ext2fs {
    pub fn new(ext2: Ext2Filesystem, fs_id: FileSystemId) -> Self {
        Self {
            ext2: Arc::new(DeadMutex::new(ext2)),
            fs_id,
        }
    }
}

impl FileSystem for Ext2fs {
    fn root(&self) -> VfsResult<(DirectoryEntry, Inode)> {
        unimplemented!()
    }
    // fn name(&self) -> &str {
    //     "Ext2fs"
    // }

    // fn root_dentry(&self) -> DirectoryEntry {
    //     unimplemented!()
    // }

    // fn root_inode(&self) -> Inode {
    //     unimplemented!()
    // }

    // fn load_inode(&self, _inode_number: InodeNumber) -> VfsResult<Inode> {
    //     unimplemented!()
    // }

    fn lookup_directory(&self, inode_nbr: u32) -> VfsResult<Vec<(DirectoryEntry, Inode)>> {
        let _res = self.ext2.lock().lookup_directory(inode_nbr)?;
        unimplemented!()
    }
}
