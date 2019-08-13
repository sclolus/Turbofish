use super::SysResult;

use bitflags::bitflags;
use errno::Errno;

use crate::memory::tools::Virt;

bitflags! {
    pub struct MmapProt: u32 {
        ///Pages may not be accessed.
        const NONE = 0;
        /// Pages may be read.
        const READ = 0x1;
        ///Pages may be written.
        const WRITE = 0x2;
        /// Pages may be executed.
        const EXEC = 0x4;
    }
}

/// Set protection on a region of memory
pub unsafe fn sys_mprotect(_addr: Virt, _length: usize, _prot: MmapProt) -> SysResult<u32> {
    unpreemptible_context!({
        // TODO: Change Entry range
    });
    Err(Errno::Eperm)
}
