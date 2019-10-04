use super::{Driver, FileOperation, InodeId, IpcResult, ProcFsOperations, SysResult, VFS};

use alloc::borrow::Cow;
use alloc::sync::Arc;

use fallible_collections::FallibleArc;

use libc_binding::OpenFlags;
use sync::DeadMutex;

type Mutex<T> = DeadMutex<T>;

use libc_binding::{off_t, Whence};

#[derive(Debug, Clone)]
pub struct MeminfoDriver {
    inode_id: InodeId,
}

impl MeminfoDriver {
    pub fn new(inode_id: InodeId) -> Self {
        Self { inode_id }
    }
}

unsafe impl Send for MeminfoDriver {}

impl Driver for MeminfoDriver {
    fn open(&mut self, _flags: OpenFlags) -> SysResult<IpcResult<Arc<Mutex<dyn FileOperation>>>> {
        let res = Arc::try_new(Mutex::new(MeminfoOperations {
            inode_id: self.inode_id,
            offset: 0,
        }))?;
        Ok(IpcResult::Done(res))
    }
}

#[derive(Debug, Default)]
pub struct MeminfoOperations {
    // offset: u64,
    inode_id: InodeId,
    offset: usize,
}

impl ProcFsOperations for MeminfoOperations {
    fn get_seq_string(&self) -> SysResult<Cow<str>> {
        let meminfo_string = "MemTotal:        42 kB
MemFree:          42 kB
MemAvailable:    42 kB
Buffers:          42 kB
Cached:          42 kB
SwapCached:         42 kB
Active:          42 kB
Inactive:        42 kB
Active(anon):    42 kB
Inactive(anon):   42 kB
Active(file):    42 kB
Inactive(file):   42 kB
Unevictable:         42 kB
Mlocked:             42 kB
SwapTotal:       42 kB
SwapFree:        42 kB
Dirty:               42 kB
Writeback:             42 kB
AnonPages:       42 kB
Mapped:           42 kB
Shmem:            42 kB
Slab:             42 kB
SReclaimable:     42 kB
SUnreclaim:       42 kB
KernelStack:       42 kB
PageTables:        42 kB
NFS_Unstable:          42 kB
Bounce:                42 kB
WritebackTmp:          42 kB
CommitLimit:     42 kB
Committed_AS:   42 kB
VmallocTotal:   42 kB
VmallocUsed:           42 kB
VmallocChunk:          42 kB
Percpu:             42 kB
HardwareCorrupted:     42 kB
AnonHugePages:    42 kB
ShmemHugePages:        42 kB
ShmemPmdMapped:        42 kB
HugePages_Total:       42
HugePages_Free:        42
HugePages_Rsvd:        42
HugePages_Surp:        42
Hugepagesize:       42 kB
Hugetlb:               42 kB
DirectMap4k:      42 kB
DirectMap2M:     42 kB
DirectMap1G:     42 kB";

        Ok(Cow::from(meminfo_string))
    }
    fn get_offset(&mut self) -> &mut usize {
        &mut self.offset
    }
}

impl FileOperation for MeminfoOperations {
    fn get_inode_id(&self) -> SysResult<InodeId> {
        Ok(self.inode_id)
    }

    fn read(&mut self, buf: &mut [u8]) -> SysResult<IpcResult<u32>> {
        self.seq_read(buf)
    }

    fn lseek(&mut self, offset: off_t, whence: Whence) -> SysResult<off_t> {
        self.proc_lseek(offset, whence)
    }
}

impl Drop for MeminfoOperations {
    fn drop(&mut self) {
        VFS.lock().close_file_operation(self.inode_id);
    }
}
