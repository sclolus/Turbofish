use super::scheduler::SCHEDULER;
use super::SysResult;
use alloc::vec::Vec;
use errno::Errno;
use fallible_collections::FallibleVec;
use libc_binding::gid_t;

/// setgroups() sets the supplementary group IDs for the calling
/// process.  Appropriate privileges are required (see the description
/// of the EPERM error, below).  The size argument specifies the
/// number of supplementary group IDs in the buffer pointed to by
/// list.
pub fn sys_setgroups(gidsetsize: i32, grouplist: *const gid_t) -> SysResult<u32> {
    unpreemptible_context!({
        if gidsetsize < 0 {
            return Err(Errno::Einval);
        }
        let mut scheduler = SCHEDULER.lock();
        let grouplist = {
            let v = scheduler
                .current_task()
                .unwrap_process()
                .get_virtual_allocator();

            // Check if pointer exists in user virtual address space
            v.check_user_ptr_with_len(grouplist, gidsetsize as usize)?;
            unsafe { core::slice::from_raw_parts(grouplist, gidsetsize as usize) }
        };
        let thread_group = scheduler.current_thread_group_mut();
        let cred = &mut thread_group.credentials;
        if cred.uid != 0 {
            return Err(Errno::Eperm);
        }
        let mut new_groups = Vec::new();
        new_groups.try_extend_from_slice(grouplist)?;
        cred.groups = new_groups;
        Ok(0)
    })
}
