//! sys_reboot and sys_shutdown implementations

use super::{SysResult, SCHEDULER};

use libc_binding::Errno;

use crate::drivers::ACPI;

/// Reboot thw computer
pub fn sys_reboot() -> SysResult<u32> {
    unpreemptible_context!({
        match *ACPI.lock() {
            Some(mut acpi) => match acpi.reboot_computer() {
                Ok(_) => {}
                Err(e) => {
                    log::error!(
                        "ACPI reboot failure: {:?}. Trying with PS/2 controler ...",
                        e
                    );
                    // Try to reboot with PS/2 controler
                    SCHEDULER.lock().reboot_computer();
                }
            },
            None => {
                // Try to reboot with PS/2 controler
                SCHEDULER.lock().reboot_computer();
            }
        }
    });
    Err(Errno::EACCES)
}
