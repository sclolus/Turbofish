use super::{DirectoryEntry, Inode, InodeNumber, VfsResult};

pub trait FileSystem: Send {
    fn name(&self) -> &str;
    fn get_superblock(&self) -> Superblock;
    fn root_dentry(&self) -> DirectoryEntry;
    fn root_inode(&self) -> Inode;
    fn load_inode(&self, inode_number: InodeNumber) -> VfsResult<Inode>;
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

pub struct SuperblockOperations {
    #[allow(unused)]
    lookup: Option<fn(&mut Superblock)>,
    #[allow(unused)]
    create: Option<fn(&mut Superblock)>,
    #[allow(unused)]
    unlink: Option<fn(&mut Superblock)>,
    #[allow(unused)]
    link: Option<fn(&mut Superblock)>,
    #[allow(unused)]
    symlink: Option<fn(&mut Superblock)>,
    #[allow(unused)]
    statfs: Option<fn(&mut Superblock)>,
    #[allow(unused)]
    mkdir: Option<fn(&mut Superblock)>,
    #[allow(unused)]
    rmdir: Option<fn(&mut Superblock)>,
}

pub struct Superblock {
    // filesystem_type: FileSystemType,
    #[allow(unused)]
    operations: SuperblockOperations,
}

pub struct StandardFileSystem {}

impl StandardFileSystem {
    pub fn new() -> Self {
        Self {}
    }
}

impl FileSystem for StandardFileSystem {
    fn name(&self) -> &str {
        "StandardFileSystem"
    }

    fn get_superblock(&self) -> Superblock {
        let operations = SuperblockOperations {
            lookup: None,
            create: None,
            unlink: None,
            link: None,
            symlink: None,
            statfs: None,
            mkdir: None,
            rmdir: None,
        };

        Superblock { operations }
    }

    fn root_dentry(&self) -> DirectoryEntry {
        unimplemented!()
    }

    fn root_inode(&self) -> Inode {
        unimplemented!()
    }

    fn load_inode(&self, _inode_number: InodeNumber) -> VfsResult<Inode> {
        unimplemented!()
    }
}
