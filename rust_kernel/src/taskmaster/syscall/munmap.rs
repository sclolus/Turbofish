use super::SysResult;
use crate::memory::tools::Virt;

/// Unmap files or devices into memory
pub unsafe fn sys_munmap(_addr: Virt, _length: usize) -> SysResult<u32> {
    unpreemptible_context!({
        // TODO: Unallocate
    });
    Ok(0)
    //Err((0, Errno::Eperm))
}
