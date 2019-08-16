//! sys_reboot and sys_shutdown implementations

use super::SysResult;

use errno::Errno;
use keyboard::PS2_CONTROLER;

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
                    unsafe {
                        PS2_CONTROLER.reboot_computer();
                    }
                }
            },
            None => unsafe { PS2_CONTROLER.reboot_computer() },
        }
    });
    Err(Errno::Eacces)
}
