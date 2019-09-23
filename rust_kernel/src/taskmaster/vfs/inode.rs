pub type InodeNumber = u32;
use super::DeadFileSystem;
use super::DefaultDriver;
use super::Driver;
use super::FileSystem;
use super::{FileOperation, IpcResult, OpenFlags};
use crate::taskmaster::SysResult;
// use super::{FileSystemId, VfsError, VfsHandler, VfsHandlerKind, VfsHandlerParams, VfsResult};
use super::FileSystemId;
use alloc::boxed::Box;
use alloc::sync::Arc;
use fallible_collections::{FallibleArc, FallibleBox};
use libc_binding::{
    dev_t, gid_t, ino_t, mode_t, nlink_t, off_t, stat, time_t, timespec, uid_t, Errno, FileType,
};
use sync::DeadMutex;

#[derive(Debug)]
pub struct Inode {
    inode_data: InodeData,
    pub driver: Box<dyn Driver>,
    /// a reference counter of open file operation on this inode
    nbr_open_file_operation: usize,
    /// if true, the inode need to be unlink when
    /// nbr_open_file_operation reach to 0
    lazy_unlink: bool,
    pub filesystem: Arc<DeadMutex<dyn FileSystem>>,
}

use core::ops::{Deref, DerefMut};

impl Deref for Inode {
    type Target = InodeData;
    fn deref(&self) -> &Self::Target {
        &self.inode_data
    }
}

impl DerefMut for Inode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inode_data
    }
}

impl Inode {
    pub fn open(
        &mut self,
        flags: OpenFlags,
    ) -> SysResult<IpcResult<Arc<DeadMutex<dyn FileOperation>>>> {
        if flags.contains(OpenFlags::O_TRUNC) {
            if self
                .filesystem
                .lock()
                .truncate(self.id.inode_number, 0)
                .is_ok()
            {
                self.inode_data.set_size(0);
            }
        }
        let res = self.driver.open(flags)?;
        self.nbr_open_file_operation += 1;
        Ok(res)
    }
    pub fn new(
        filesystem: Arc<DeadMutex<dyn FileSystem>>,
        driver: Box<dyn Driver>,
        inode_data: InodeData,
    ) -> Self {
        Self {
            inode_data,
            filesystem,
            driver,
            nbr_open_file_operation: 0,
            lazy_unlink: false,
        }
    }
    pub fn root_inode() -> SysResult<Self> {
        Ok(Self {
            inode_data: InodeData::root_inode(),
            driver: Box::try_new(DefaultDriver)?,
            filesystem: Arc::try_new(DeadMutex::new(DeadFileSystem))?,
            nbr_open_file_operation: 0,
            lazy_unlink: false,
        })
    }
    pub fn stat(&self, stat: &mut stat) -> SysResult<u32> {
        self.inode_data.stat(stat)
    }
    pub fn write(&mut self, offset: &mut u64, buf: &[u8]) -> SysResult<u32> {
        // TODO: update size in inode data
        Ok(self
            .filesystem
            .lock()
            .write(self.id.inode_number, offset, buf)? as u32)
    }
    pub fn read(&mut self, offset: &mut u64, buf: &mut [u8]) -> SysResult<u32> {
        if self.inode_data.is_directory() {
            return Err(Errno::EISDIR);
        }
        Ok(self
            .filesystem
            .lock()
            .read(self.id.inode_number, offset, buf)? as u32)
    }

    /// return if we can unlink directly or not
    pub fn close(&mut self) -> bool {
        assert!(self.nbr_open_file_operation > 0);
        self.nbr_open_file_operation -= 1;
        self.link_number == 0 && self.nbr_open_file_operation == 0
    }

    /// increment artificialy open file operation field. Used for
    /// binding a socket
    pub unsafe fn incr_nbr_open_file_operation(&mut self) {
        self.nbr_open_file_operation += 1;
    }

    /// return if we can unlink directly or not
    pub fn unlink(&mut self) -> bool {
        assert!(self.link_number > 0);
        self.link_number -= 1;
        if self.link_number == 0 && self.nbr_open_file_operation > 0 {
            self.lazy_unlink = true;
        }
        self.link_number == 0 && !self.lazy_unlink
    }

    pub fn get_id(&self) -> InodeId {
        self.inode_data.get_id()
    }

    pub fn get_driver(&mut self) -> &mut dyn Driver {
        &mut *self.driver as &mut dyn Driver
    }

    pub fn set_driver(&mut self, new_driver: Box<dyn Driver>) {
        self.driver = new_driver;
    }
}

#[derive(Default, Debug)]
pub struct InodeData {
    /// This inode's id.
    pub id: InodeId,

    /// This inode's hard link number
    pub link_number: nlink_t,
    pub access_mode: FileType,

    //TODO: chnage this to (owner, group),
    pub uid: uid_t,
    pub gid: gid_t,

    pub atime: time_t,
    pub mtime: time_t,
    pub ctime: time_t,

    pub size: u64,
}

impl InodeData {
    pub fn set_size(&mut self, size: u64) {
        self.size = size;
    }

    pub fn stat(&self, stat: &mut stat) -> SysResult<u32> {
        *stat = stat {
            st_dev: 42 as dev_t,                   // Device ID of device containing file.
            st_ino: self.id.inode_number as ino_t, // File serial number.
            st_mode: self.access_mode.bits() as mode_t, // Mode of file (see below).
            st_nlink: self.link_number,            // Number of hard links to the file.
            st_uid: self.uid,                      // User ID of file.
            st_gid: self.gid,                      // Group ID of file.
            st_rdev: 0 as dev_t, //TODO // Device ID (if file is character or block special).
            st_size: self.size as off_t, // For regular files, the file size in bytes.
            st_atim: timespec {
                // Last data access timestamp.
                tv_sec: self.atime as time_t,
                tv_nsec: 0,
            },
            st_mtim: timespec {
                tv_sec: self.mtime as time_t,
                tv_nsec: 0,
            }, // Last data modification timestamp.
            st_ctim: timespec {
                tv_sec: self.ctime as time_t,
                tv_nsec: 0,
            }, // Last file status change timestamp.
            st_blksize: 42, //self.ext2.lock().get_block_size() as blksize_t, // A file system-specific preferred I/O block size
            st_blocks: 42, //self.nbr_disk_sectors as blkcnt_t, // Number of blocks allocated for this object.
        };
        Ok(0)
    }
    // Builder Pattern
    pub fn set_id(&mut self, id: InodeId) -> &mut Self {
        self.id = id;
        self
    }

    pub fn set_access_mode(&mut self, mode: FileType) -> &mut Self {
        self.access_mode = mode;
        self
    }

    pub fn set_uid(&mut self, uid: uid_t) -> &mut Self {
        self.uid = uid;
        self
    }

    pub fn set_gid(&mut self, gid: gid_t) -> &mut Self {
        self.gid = gid;
        self
    }

    pub fn set_link_number(&mut self, link_number: nlink_t) -> &mut Self {
        self.link_number = link_number;
        self
    }

    // Builder Pattern end

    pub fn get_id(&self) -> InodeId {
        self.id
    }

    pub fn root_inode() -> Self {
        let access_mode = FileType::S_IRWXU | FileType::DIRECTORY;

        Self {
            id: InodeId::new(2, None),
            link_number: 1,
            access_mode,
            uid: 0,
            gid: 0,
            atime: 0,
            ctime: 0,
            mtime: 0,
            size: 4096,
        }
    }

    pub fn is_character_device(&self) -> bool {
        self.access_mode.is_character_device()
    }

    pub fn is_fifo(&self) -> bool {
        self.access_mode.is_fifo()
    }

    pub fn is_regular(&self) -> bool {
        self.access_mode.is_regular()
    }

    pub fn is_directory(&self) -> bool {
        self.access_mode.is_directory()
    }

    pub fn is_symlink(&self) -> bool {
        self.access_mode.is_symlink()
    }

    pub fn is_socket(&self) -> bool {
        self.access_mode.is_socket()
    }
}

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct InodeId {
    pub inode_number: InodeNumber,
    pub filesystem_id: Option<FileSystemId>,
}

impl InodeId {
    pub fn new(inode_number: InodeNumber, filesystem_id: Option<FileSystemId>) -> Self {
        Self {
            inode_number,
            filesystem_id,
        }
    }
}

#[cfg(test)]
mod test {
    // use super::VfsHandlerParams;
    use super::*;

    macro_rules! make_test {
        ($body: expr, $name: ident) => {
            #[test]
            fn $name() {
                $body
            }
        };
        (failing, $body: expr, $name: ident) => {
            #[test]
            #[should_panic]
            fn $name() {
                $body
            }
        };
    }

    // fn test_open(_params: VfsHandlerParams) -> VfsResult<i32> {
    //     Ok(0)
    // }

    // make_test! {
    //     {
    //         let mut inode = Inode::default();
    //         let mut file = File::new(InodeId::new(0, FileSystemId::new(0)), DirectoryEntryId::new(0));

    //         let mut inode_operations = InodeOperations::default()
    //             .set_test_open(test_open);

    //         inode.set_inode_operations(inode_operations);
    //         let params = VfsHandlerParams::new()
    //             .set_inode(&inode)
    //             .set_file(&file);

    //         let res = inode.dispatch_handler(params, VfsHandlerKind::TestOpen).unwrap();
    //         assert_eq!(res, 0);
    //     }, inode_open
    // }
}
