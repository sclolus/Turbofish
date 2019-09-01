use super::{DirectoryEntry, Inode, VfsResult};
use alloc::sync::Arc;
use alloc::vec::Vec;
use core::convert::TryFrom;
use ext2::{DirectoryEntryType, Ext2Filesystem};
use sync::DeadMutex;

pub trait FileSystem: Send {
    // fn name(&self) -> &str;
    // fn load_inode(&self, inode_number: InodeNumber) -> VfsResult<Inode>;
    /// return all the directory entry and inode present in the inode_nbr
    fn lookup_directory(&self, inode_nbr: u32) -> VfsResult<Vec<(DirectoryEntry, Inode)>>;
    /// return the (possibly virtual) directory entry and inode of the root
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

/// the ext2 wrapper which implement filesystem
impl Ext2fs {
    pub fn new(ext2: Ext2Filesystem, fs_id: FileSystemId) -> Self {
        Self {
            ext2: Arc::new(DeadMutex::new(ext2)),
            fs_id,
        }
    }
}

use super::{DirectoryEntryBuilder, Filename, InodeData, InodeId};
use crate::taskmaster::drivers::Ext2DriverFile;
use core::default::Default;

impl FileSystem for Ext2fs {
    fn root(&self) -> VfsResult<(DirectoryEntry, Inode)> {
        let root_inode = self.ext2.lock().root_inode();

        let inode_id = InodeId::new(2, Some(self.fs_id));

        let direntry = {
            let mut builder = DirectoryEntryBuilder::new();
            builder
                .set_filename(Filename::try_from("ext2Root").unwrap())
                .set_inode_id(inode_id)
                .set_directory();
            builder.build()
        };

        let mut inode_data = InodeData::default();
        inode_data.set_id(inode_id);
        // TODO get more fields from the ext2 inode

        let inode = Inode::new(
            Arc::new(DeadMutex::new(Ext2DriverFile::new(self.ext2.clone(), 2))),
            inode_data,
        );
        Ok((direntry, inode))
    }
    // fn name(&self) -> &str {
    //     "Ext2fs"
    // }

    fn lookup_directory(&self, inode_nbr: u32) -> VfsResult<Vec<(DirectoryEntry, Inode)>> {
        let res = self.ext2.lock().lookup_directory(inode_nbr)?;
        Ok(res
            .into_iter()
            .filter_map(|(direntry, inode)| {
                if unsafe { direntry.get_filename() == ".." || direntry.get_filename() == "." } {
                    None
                } else {
                    let inode_id = InodeId::new(direntry.get_inode() as usize, Some(self.fs_id));

                    let direntry = {
                        let mut builder = DirectoryEntryBuilder::new();
                        builder
                            .set_filename(Filename(
                                direntry.filename.0,
                                direntry.header.name_length as usize,
                            ))
                            .set_inode_id(inode_id);
                        if direntry.header.type_indicator == DirectoryEntryType::Directory {
                            builder.set_directory();
                        } else if direntry.header.type_indicator == DirectoryEntryType::RegularFile
                        {
                            builder.set_regular();
                        }
                        builder.build()
                    };

                    let mut inode_data = InodeData::default();
                    inode_data.set_id(inode_id);
                    // TODO get more fields from the ext2 inode

                    let inode = Inode::new(
                        Arc::new(DeadMutex::new(Ext2DriverFile::new(self.ext2.clone(), 2))),
                        inode_data,
                    );
                    Some((direntry, inode))
                }
            })
            .collect())
    }
}
