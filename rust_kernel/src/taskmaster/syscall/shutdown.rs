use super::SysResult;

use errno::Errno;
use keyboard::PS2_CONTROLER;

use crate::drivers::ACPI;
use crate::system::i8086_payload_apm_shutdown;

/// Shutdown the computer
pub fn sys_shutdown() -> SysResult<u32> {
    unpreemptible_context!({
        match *ACPI.lock() {
            Some(mut acpi) => match unsafe { acpi.shutdown() } {
                Ok(_) => {}
                Err(e) => {
                    log::error!("ACPI shudown failure: {:?}. Trying with APM ...", e);
                    match i8086_payload_apm_shutdown() {
                        Ok(_) => {}
                        Err(e) => log::error!("APM shutdown error: {:?}", e),
                    }
                }
            },
            None => match i8086_payload_apm_shutdown() {
                Ok(_) => {}
                Err(e) => log::error!("APM shutdown error: {:?}", e),
            },
        }
        log::error!("shutdown failure ... it is very disapointing ...");
    });
    Err(Errno::Eacces)
}
