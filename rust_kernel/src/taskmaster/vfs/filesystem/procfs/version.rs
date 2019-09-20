use super::{
    DirectoryEntry, Driver, FileOperation, FileSystem, FileSystemId, ProcFsDriver, SysResult,
};
use super::{DirectoryEntryBuilder, Filename, Inode, InodeId, Path, VfsInodeData};
use super::{IpcResult, KeyGenerator, Mapper};
use alloc::collections::btree_map::BTreeMap;
use alloc::vec::Vec;
use core::fmt::Debug;
use libc_binding::{gid_t, statfs, uid_t, utimbuf, Errno, FileType};

#[derive(Debug, Clone)]
pub struct VersionDriver;

unsafe impl Send for VersionDriver {}

impl ProcFsDriver for VersionDriver {
    type Operations = VersionOperations;
}

#[derive(Debug, Default)]
pub struct VersionOperations {
    // offset: u64,
    offset: usize,
}

const KERNEL_VERSION: &'static str = "Turbofish v?.?.?\n";

impl FileOperation for VersionOperations {
    fn read(&mut self, buf: &mut [u8]) -> SysResult<IpcResult<u32>> {
        if buf.len() > u32::max_value() as usize {
            return Err(Errno::EOVERFLOW);
        }

        if self.offset >= KERNEL_VERSION.len() {
            return Ok(IpcResult::Done(0));
        }

        let version = &KERNEL_VERSION[self.offset as usize..];

        let mut bytes = version.bytes();

        let mut ret = 0;
        for (index, to_fill) in buf.iter_mut().enumerate() {
            match bytes.next() {
                Some(byte) => *to_fill = byte,
                None => {
                    ret = index + 1;
                    break;
                }
            }
        }

        log::warn!("Version read: ret = {}, buf.len() = {}", ret, buf.len());
        self.offset += ret;
        Ok(IpcResult::Done(ret as u32))
    }
}
