use super::{Driver, FileOperation, IpcResult, SysResult};

use alloc::{boxed::Box, sync::Arc};

use fallible_collections::{boxed::FallibleBox, FallibleArc};

use core::fmt::Debug;

use libc_binding::OpenFlags;
use sync::DeadMutex;

type Mutex<T> = DeadMutex<T>;

use libc_binding::Errno;

#[derive(Debug, Clone)]
pub struct MeminfoDriver;

unsafe impl Send for MeminfoDriver {}

impl Driver for MeminfoDriver {
    fn open(
        &mut self,
        _flags: OpenFlags,
    ) -> SysResult<IpcResult<Arc<DeadMutex<dyn FileOperation>>>> {
        let res = Arc::try_new(DeadMutex::new(MeminfoOperations { offset: 0 }))?;
        Ok(IpcResult::Done(res))
    }
}

#[derive(Debug, Default)]
pub struct MeminfoOperations {
    // offset: u64,
    offset: usize,
}

impl FileOperation for MeminfoOperations {
    fn read(&mut self, buf: &mut [u8]) -> SysResult<IpcResult<u32>> {
        if buf.len() > u32::max_value() as usize {
            return Err(Errno::EOVERFLOW);
        }

        //TODO: Unfailible context.
        //TODO: This is dummy.
        let meminfo_string = format!(
            "MemTotal:        0 kB
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
DirectMap1G:     0 kB"
        );

        if self.offset >= meminfo_string.len() {
            return Ok(IpcResult::Done(0));
        }

        let version = &meminfo_string[self.offset as usize..];

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
        self.offset += ret;
        Ok(IpcResult::Done(ret as u32))
    }
}
