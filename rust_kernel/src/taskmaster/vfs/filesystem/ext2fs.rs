use super::FileSystem;
use super::{DirectoryEntry, FileSystemId, InodeData, VfsResult};
use super::{DirectoryEntryBuilder, Filename, InodeId, SysResult};
use crate::drivers::rtc::CURRENT_UNIX_TIME;
use alloc::sync::Arc;
use alloc::vec::Vec;
use core::convert::TryFrom;
use core::default::Default;
use core::sync::atomic::Ordering;
use ext2::{DirectoryEntryType, Ext2Filesystem};
use libc_binding::{gid_t, uid_t, FileType, OpenFlags};

use sync::DeadMutex;

#[derive(Debug)]
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

impl From<ext2::Inode> for InodeData {
    fn from(inode_ext2: ext2::Inode) -> InodeData {
        InodeData {
            //TODO: check if we can put the right types here
            id: Default::default(),
            link_number: inode_ext2.nbr_hard_links as i32,
            access_mode: inode_ext2.type_and_perm,
            uid: inode_ext2.user_id as u32,
            gid: inode_ext2.group_id as u32,
            atime: inode_ext2.last_access_time,
            mtime: inode_ext2.last_modification_time,
            ctime: inode_ext2.last_access_time,
            size: inode_ext2.get_size(),
        }
    }
}

impl Ext2fs {
    fn convert_entry_ext2_to_vfs(
        &self,
        direntry: ext2::DirectoryEntry,
        inode: ext2::Inode,
    ) -> (DirectoryEntry, InodeData) {
        let inode_nbr = direntry.get_inode();
        let inode_id = InodeId::new(inode_nbr, Some(self.fs_id));

        let direntry = {
            let mut builder = DirectoryEntryBuilder::new();
            builder
                .set_filename(Filename::new(
                    direntry.filename.0,
                    direntry.header.name_length as usize,
                ))
                .set_inode_id(inode_id);
            if direntry.header.type_indicator == DirectoryEntryType::Directory {
                builder.set_directory();
            } else if direntry.header.type_indicator == DirectoryEntryType::RegularFile {
                builder.set_regular();
            }
            builder.build()
        };

        let mut inode_data = InodeData::from(inode);
        inode_data.set_id(inode_id);
        (direntry, inode_data)
    }
}

impl FileSystem for Ext2fs {
    fn root(&self) -> VfsResult<(DirectoryEntry, InodeData)> {
        let root_inode = self.ext2.lock().root_inode()?;

        let inode_id = InodeId::new(2, Some(self.fs_id));

        let direntry = {
            let mut builder = DirectoryEntryBuilder::new();
            builder
                .set_filename(Filename::try_from("ext2Root").unwrap())
                .set_inode_id(inode_id)
                .set_directory();
            builder.build()
        };

        let mut inode_data = InodeData::from(root_inode);
        inode_data.set_id(inode_id);
        Ok((direntry, inode_data))
    }
    // fn name(&self) -> &str {
    //     "Ext2fs"
    // }

    fn lookup_directory(&self, inode_nbr: u32) -> VfsResult<Vec<(DirectoryEntry, InodeData)>> {
        let res = self.ext2.lock().lookup_directory(inode_nbr)?;
        Ok(res
            .into_iter()
            .filter_map(|(direntry, inode)| {
                if unsafe { direntry.get_filename() == ".." || direntry.get_filename() == "." } {
                    None
                } else {
                    Some(self.convert_entry_ext2_to_vfs(direntry, inode))
                }
            })
            .collect())
    }
    fn chmod(&self, inode_nbr: u32, mode: FileType) -> VfsResult<()> {
        Ok(self.ext2.lock().chmod(inode_nbr, mode)?)
    }

    fn chown(&self, inode_nbr: u32, owner: uid_t, group: gid_t) -> VfsResult<()> {
        Ok(self.ext2.lock().chown(inode_nbr, owner, group)?)
    }

    fn unlink(&self, dir_inode_nbr: u32, name: &str) -> VfsResult<()> {
        Ok(self.ext2.lock().unlink(dir_inode_nbr, name)?)
    }
    fn create(
        &mut self,
        filename: &str,
        parent_inode_nbr: u32,
        flags: OpenFlags,
        mode: FileType,
    ) -> VfsResult<(DirectoryEntry, InodeData)> {
        // We probably should provide it as a parameter to this method.
        let timestamp = unsafe { CURRENT_UNIX_TIME.load(Ordering::Relaxed) };
        let (direntry, inode) =
            self.ext2
                .lock()
                .create(filename, parent_inode_nbr, flags, timestamp, mode)?;
        Ok(self.convert_entry_ext2_to_vfs(direntry, inode))
    }
    fn write(&mut self, inode_number: u32, offset: &mut u64, buf: &[u8]) -> SysResult<u32> {
        Ok(self.ext2.lock().write(inode_number, offset, buf)? as u32)
    }
    fn read(&mut self, inode_number: u32, offset: &mut u64, buf: &mut [u8]) -> SysResult<u32> {
        Ok(self.ext2.lock().read(inode_number, offset, buf)? as u32)
    }
}
