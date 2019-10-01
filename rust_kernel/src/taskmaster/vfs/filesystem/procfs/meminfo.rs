use super::{Driver, FileOperation, InodeId, IpcResult, ProcFsOperations, SysResult, VFS};

use alloc::borrow::Cow;
use alloc::sync::Arc;

use fallible_collections::FallibleArc;

use libc_binding::OpenFlags;
use sync::DeadMutex;

type Mutex<T> = DeadMutex<T>;

use libc_binding::{off_t, Errno, Whence};

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
        let meminfo_string = "MemTotal:        0 kB
MemFree:          0 kB
MemAvailable:    0 kB
Buffers:          0 kB
Cached:          0 kB
SwapCached:         0 kB
Active:          0 kB
Inactive:        0 kB
Active(anon):    0 kB
Inactive(anon):   0 kB
Active(file):    0 kB
Inactive(file):   0 kB
Unevictable:         0 kB
Mlocked:             0 kB
SwapTotal:       0 kB
SwapFree:        0 kB
Dirty:               0 kB
Writeback:             0 kB
AnonPages:       0 kB
Mapped:           0 kB
Shmem:            0 kB
Slab:             0 kB
SReclaimable:     0 kB
SUnreclaim:       0 kB
KernelStack:       0 kB
PageTables:        0 kB
NFS_Unstable:          0 kB
Bounce:                0 kB
WritebackTmp:          0 kB
CommitLimit:     0 kB
Committed_AS:   0 kB
VmallocTotal:   0 kB
VmallocUsed:           0 kB
VmallocChunk:          0 kB
Percpu:             0 kB
HardwareCorrupted:     0 kB
AnonHugePages:    0 kB
ShmemHugePages:        0 kB
ShmemPmdMapped:        0 kB
HugePages_Total:       0
HugePages_Free:        0
HugePages_Rsvd:        0
HugePages_Surp:        0
Hugepagesize:       0 kB
Hugetlb:               0 kB
DirectMap4k:      0 kB
DirectMap2M:     0 kB
DirectMap1G:     0 kB";

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
