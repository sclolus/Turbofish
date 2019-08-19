use super::SysResult;
use super::SCHEDULER;
use crate::memory::tools::{Address, NbrPages, Virt, PAGE_SIZE};
use libc_binding::Errno;

/// The munmap() function shall remove any mappings for those entire
/// pages containing any part of the address space of the process
/// starting at addr and continuing for len bytes. Further references
/// to these pages shall result in the generation of a SIGSEGV signal
/// to the process. If there are no mappings in the specified address
/// range, then munmap() has no effect.
///
/// The implementation may require that addr be a multiple of the page
/// size as returned by sysconf().
///
/// If a mapping to be removed was private, any modifications made in
/// this address range shall be discarded.
///
/// [ML|MLR] [Option Start] Any memory locks (see mlock and mlockall)
/// associated with this address range shall be removed, as if by an
/// appropriate call to munlock(). [Option End]
///
/// [TYM] [Option Start] If a mapping removed from a typed memory
/// object causes the corresponding address range of the memory pool
/// to be inaccessible by any process in the system except through
/// allocatable mappings (that is, mappings of typed memory objects
/// opened with the POSIX_TYPED_MEM_MAP_ALLOCATABLE flag), then that
/// range of the memory pool shall become deallocated and may become
/// available to satisfy future typed memory allocation requests.
///
/// A mapping removed from a typed memory object opened with the
/// POSIX_TYPED_MEM_MAP_ALLOCATABLE flag shall not affect in any way
/// the availability of that typed memory for allocation. [Option End]
///
/// The behavior of this function is unspecified if the mapping was
/// not established by a call to mmap().
/// The munmap() function shall fail if:
///
/// [EINVAL]
///     Addresses in the range [addr,addr+len) are outside the valid
///     range for the address space of a process.
/// [EINVAL]
///     The len argument is 0.
///
/// The munmap() function may fail if:
///
/// [EINVAL]
///     The addr argument is not a multiple of the page size as
///     returned by sysconf().
pub unsafe fn sys_munmap(addr: *mut u8, length: usize) -> SysResult<u32> {
    unpreemptible_context!({
        let vaddr = Virt(addr as usize);
        if !vaddr.is_aligned_on(PAGE_SIZE) {
            log::warn!("a munmap addr was unaligned: {:?}", vaddr);
            return Err(Errno::EINVAL);
        }

        let mut scheduler = SCHEDULER.lock();
        let mut v = scheduler
            .current_thread_mut()
            .unwrap_process_mut()
            .get_virtual_allocator();
        match v.check_user_ptr_with_len(addr, length) {
            Err(e) => log::warn!(
                "a munmap was bullshit, error: {:?}, {:?}, size {}",
                e,
                vaddr,
                length
            ),
            _ => {
                // good because we know that vaddr is aligned
                let ret = v.unmap_addr(vaddr.into(), NbrPages::from(length));
                if let Err(e) = ret {
                    log::warn!(
                        "a munmap was bullshit, error: {:?}, {:?}, size {}",
                        e,
                        vaddr,
                        length
                    );
                }
            }
        }
    });
    Ok(0)
}
