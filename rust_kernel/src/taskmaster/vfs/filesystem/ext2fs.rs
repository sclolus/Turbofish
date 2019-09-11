use super::FileSystem;
use super::{DirectoryEntry, FileSystemId, InodeData, Path};
use super::{DirectoryEntryBuilder, Filename, InodeId, SysResult};
use crate::drivers::rtc::CURRENT_UNIX_TIME;
use alloc::sync::Arc;
use alloc::vec::Vec;
use core::convert::TryFrom;
use core::default::Default;
use core::str;
use core::sync::atomic::Ordering;
use ext2::{DirectoryEntryType, Ext2Filesystem};
use fallible_collections::TryCollect;
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
            id: Default::default(),
            link_number: inode_ext2.nbr_hard_links,
            access_mode: inode_ext2.type_and_perm,
            uid: inode_ext2.user_id,
            gid: inode_ext2.group_id,
            atime: inode_ext2.last_access_time,
            mtime: inode_ext2.last_modification_time,
            ctime: inode_ext2.last_access_time,
            size: inode_ext2.get_size(),
        }
    }
}

impl Ext2fs {
    fn convert_entry_ext2_to_vfs(
        &mut self,
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
            } else if direntry.header.type_indicator == DirectoryEntryType::Fifo {
                builder.set_fifo();
            } else if direntry.header.type_indicator == DirectoryEntryType::SymbolicLink {
                builder.set_symlink(self.read_symlink(&inode, inode_nbr).unwrap());
            }
            builder.build()
        };

        let mut inode_data = InodeData::from(inode);
        inode_data.set_id(inode_id);
        (direntry, inode_data)
    }
    fn read_symlink(&mut self, inode: &ext2::Inode, inode_number: u32) -> SysResult<Path> {
        // dbg!(inode);
        let mut buf = [0; 255];
        // if inode size < 60 it is a fast symbolic link (ie the
        // string is stocked directly in the struct inode)
        let pathname = if inode.get_size() <= ext2::Inode::FAST_SYMLINK_SIZE_MAX as u64 {
            inode.read_symlink()
        } else {
            let mut offset = 0;
            self.read(inode_number, &mut offset, &mut buf)?;
            str::from_utf8(&buf[0..offset as usize]).ok()
        };
        Ok(Path::try_from(pathname.unwrap_or("corrupted link"))?)
    }
}

impl FileSystem for Ext2fs {
    fn root(&self) -> SysResult<(DirectoryEntry, InodeData)> {
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

    fn lookup_directory(&mut self, inode_nbr: u32) -> SysResult<Vec<(DirectoryEntry, InodeData)>> {
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
            .try_collect()?)
    }

    fn chmod(&self, inode_nbr: u32, mode: FileType) -> SysResult<()> {
        Ok(self.ext2.lock().chmod(inode_nbr, mode)?)
    }

    fn chown(&self, inode_nbr: u32, owner: uid_t, group: gid_t) -> SysResult<()> {
        Ok(self.ext2.lock().chown(inode_nbr, owner, group)?)
    }

    fn unlink(&self, dir_inode_nbr: u32, name: &str) -> SysResult<()> {
        Ok(self.ext2.lock().unlink(dir_inode_nbr, name)?)
    }

    fn create(
        &mut self,
        filename: &str,
        parent_inode_nbr: u32,
        flags: OpenFlags,
        mode: FileType,
    ) -> SysResult<(DirectoryEntry, InodeData)> {
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

    fn create_dir(
        &mut self,
        parent_inode_nbr: u32,
        filename: &str,
        mode: FileType,
    ) -> SysResult<(DirectoryEntry, InodeData)> {
        let (direntry, inode) = self
            .ext2
            .lock()
            .create_dir(parent_inode_nbr, filename, mode)?;
        Ok(self.convert_entry_ext2_to_vfs(direntry, inode))
    }
    fn rmdir(&mut self, parent_inode_nbr: u32, filename: &str) -> SysResult<()> {
        self.ext2.lock().rmdir(parent_inode_nbr, filename)?;
        Ok(())
    }
    fn symlink(
        &mut self,
        parent_inode_nbr: u32,
        target: &str,
        filename: &str,
    ) -> SysResult<(DirectoryEntry, InodeData)> {
        let timestamp = unsafe { CURRENT_UNIX_TIME.load(Ordering::Relaxed) };
        let (direntry, inode) =
            self.ext2
                .lock()
                .symlink(parent_inode_nbr, target, filename, timestamp)?;
        Ok(self.convert_entry_ext2_to_vfs(direntry, inode))
    }
    fn link(
        &mut self,
        parent_inode_nbr: u32,
        target_inode_nbr: u32,
        filename: &str,
    ) -> SysResult<DirectoryEntry> {
        let (direntry, inode) =
            self.ext2
                .lock()
                .link(parent_inode_nbr, target_inode_nbr, filename)?;
        Ok(self.convert_entry_ext2_to_vfs(direntry, inode).0)
    }
}
