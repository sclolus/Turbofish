//! This file contains usefull tools for this entire module methods

use super::SysResult;

use errno::Errno;

use crate::memory::tools::{AllocFlags, Virt};
use crate::memory::VirtualPageAllocator;

/// Check if a pointer given by user process is not bullshit
pub fn check_user_ptr<T>(ptr: *const T, v: &VirtualPageAllocator) -> SysResult<()> {
    let start_ptr = Virt(ptr as usize);
    let end_ptr = Virt((ptr as usize).checked_add(core::mem::size_of::<T>() - 1).ok_or::<Errno>(Errno::Efault)?);

    Ok(v.check_page_range(start_ptr.into(), end_ptr.into(), AllocFlags::USER_MEMORY).map_err(|_| Errno::Efault)?)
}
