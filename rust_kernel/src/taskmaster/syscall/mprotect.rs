use super::SysResult;

use bitflags::bitflags;
use errno::Errno;

use super::SCHEDULER;
use crate::memory::tools::{Address, AllocFlags, NbrPages, Virt, PAGE_SIZE};
use libc_binding::{PROT_EXEC, PROT_NONE, PROT_READ, PROT_WRITE};

bitflags! {
    pub struct MmapProt: u32 {
        ///Pages may not be accessed.
        const NONE = PROT_NONE;
        /// Pages may be read.
        const READ = PROT_READ;
        ///Pages may be written.
        const WRITE = PROT_WRITE;
        /// Pages may be executed.
        const EXEC = PROT_EXEC;
    }
}

/// Convert flags from ELF loader into MMU AllocFlags
impl From<MmapProt> for AllocFlags {
    fn from(flags: MmapProt) -> AllocFlags {
        let entry = AllocFlags::default() | AllocFlags::USER_MEMORY;
        // We doesnt' handle the PROT_NONE flags alone, we
        // just put the entry in READ_ONLY if there is
        // PROT_NONE or there is no PROT_WRITE
        if flags == MmapProt::NONE || !flags.contains(MmapProt::WRITE) {
            return entry | AllocFlags::READ_ONLY;
        }
        entry
    }
}

/// The mprotect() function shall change the access protections to be
/// that specified by prot for those whole pages containing any part
/// of the address space of the process starting at address addr and
/// continuing for len bytes. The parameter prot determines whether
/// read, write, execute, or some combination of accesses are
/// permitted to the data being mapped. The prot argument should be
/// either PROT_NONE or the bitwise-inclusive OR of one or more of
/// PROT_READ, PROT_WRITE, and PROT_EXEC.
///
/// If an implementation cannot support the combination of access
/// types specified by prot, the call to mprotect() shall fail.
///
/// An implementation may permit accesses other than those specified
/// by prot; however, no implementation shall permit a write to
/// succeed where PROT_WRITE has not been set or shall permit any
/// access where PROT_NONE alone has been set. Implementations shall
/// support at least the following values of prot: PROT_NONE,
/// PROT_READ, PROT_WRITE, and the bitwise-inclusive OR of PROT_READ
/// and PROT_WRITE. If PROT_WRITE is specified, the application shall
/// ensure that it has opened the mapped objects in the specified
/// address range with write permission, unless MAP_PRIVATE was
/// specified in the original mapping, regardless of whether the file
/// descriptors used to map the objects have since been closed.
///
/// The implementation may require that addr be a multiple of the page
/// size as returned by sysconf().
///
/// The behavior of this function is unspecified if the mapping was
/// not established by a call to mmap().
///
/// When mprotect() fails for reasons other than [EINVAL], the
/// protections on some of the pages in the range [addr,addr+len) may
/// have been changed.
pub unsafe fn sys_mprotect(addr: *mut u8, length: usize, prot: MmapProt) -> SysResult<u32> {
    unpreemptible_context!({
        // TODO: Change Entry range
        let vaddr = Virt(addr as usize);
        if !vaddr.is_aligned_on(PAGE_SIZE) {
            log::warn!("a munmap addr was unaligned: {:?}", vaddr);
            return Err(Errno::Einval);
        }
        let mut scheduler = SCHEDULER.lock();
        let mut v = scheduler
            .current_thread_mut()
            .unwrap_process_mut()
            .get_virtual_allocator();
        match v.check_user_ptr_with_len(addr, length) {
            Err(e) => log::warn!(
                "a mprotect was bullshit, error: {:?}, {:?}, size {}",
                e,
                vaddr,
                length
            ),
            _ => {
                // good because we know that vaddr is aligned
                v.change_flags_range_page_entry(
                    vaddr.into(),
                    NbrPages::from(length),
                    AllocFlags::from(prot),
                );
            }
        }
    });
    Ok(0)
}
