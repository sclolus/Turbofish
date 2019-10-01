//! This file contains the main function of the module

use kernel_modules::{
    ModConfig, ModError, ModResult, ModReturn, ModSpecificReturn, RTCReturn, SymbolList,
    EMERGENCY_WRITER, WRITER,
};

use bit_field::BitField;
use core::sync::atomic::{AtomicU32, Ordering};
use kernel_modules::Irq;

mod rtc;
use rtc::{Rtc, RtcRegister};
use time::get_day_number;

static mut CTX: Option<Ctx> = None;

/// Main Context of the module
struct Ctx {
    enable_irq: fn(Irq, unsafe extern "C" fn()),
    disable_irq: fn(Irq),
    current_unix_time: &'static AtomicU32,
}

/// Main Context implementation
impl Ctx {
    /// New fn
    fn new(
        enable_irq: fn(Irq, unsafe extern "C" fn()),
        disable_irq: fn(Irq),
        current_unix_time: &'static AtomicU32,
    ) -> Self {
        print!("New RTC Context created !");
        Self {
            enable_irq,
            disable_irq,
            current_unix_time,
        }
    }
}

/// Drop boilerplate implementation
impl Drop for Ctx {
    fn drop(&mut self) {
        print!("RTC Context droped !");
    }
}

/// Constructor
pub fn module_start(symtab_list: SymbolList) -> ModResult {
    unsafe {
        WRITER.set_write_callback(symtab_list.write);
        EMERGENCY_WRITER.set_write_callback(symtab_list.emergency_write);
        #[cfg(not(test))]
        crate::MEMORY_MANAGER.set_methods(symtab_list.alloc_tools);
    }
    if let ModConfig::RTC(rtc_config) = symtab_list.kernel_callback {
        unsafe {
            CTX = Some(Ctx::new(
                rtc_config.enable_irq,
                rtc_config.disable_irq,
                rtc_config.current_unix_time,
            ));
        }

        // Register the RTC callback
        unsafe {
            without_interrupts!({
                let mut rtc = Rtc::new();
                // let date = rtc.read_date();
                rtc.enable_periodic_interrupts(15); // lowest possible frequency for RTC = 2 Hz
                                                    // print!("RTC system seems to be working perfectly: {}", date);
                (CTX.as_ref().unwrap().enable_irq)(Irq::RealTimeClock, rtc_interrupt_handler);
            });
        }

        Ok(ModReturn {
            stop: drop_module,
            configurable_callbacks_opt: None,
            spec: ModSpecificReturn::RTC(RTCReturn { read_date }),
        })
    } else {
        Err(ModError::BadIdentification)
    }
}

/// Destructor
fn drop_module() {
    unsafe {
        without_interrupts!({
            (CTX.as_ref().unwrap().disable_irq)(Irq::RealTimeClock);
        });
        CTX = None;
    }
}

/// Get the current Date
fn read_date() -> time::Date {
    let mut rtc = Rtc::new();
    rtc.read_date()
}

#[no_mangle]
/// The interrupt handler of the RTC, updates the CURRENT_UNIX_TIME atomic variable
/// with the updated value from the RTC.
unsafe extern "C" fn rtc_interrupt_handler() {
    if let Some(ctx) = CTX.as_mut() {
        let mut rtc = Rtc::new();

        let status = rtc.read_register(RtcRegister::StatusC, false);

        // The end-of-update interrupt is marked in the StatusC register by the 4 higher-bits being set to 0xd0.
        if status.get_bits(4..8) == 0xd {
            let date = rtc.read_date();
            // Heuristical way to determine the current century.
            // As we would need to check the ACPI tables to assert the existence of the Century register.

            let tm_sec = date.sec as u32;
            let tm_min = date.minutes as u32;
            let tm_hour = date.hours as u32;
            let tm_yday = get_day_number(date.month, date.day_of_month as usize) as u32;
            let tm_year = date.year - 1900 as u32;

            // The 19 January 2038, at 3am:14:08 UTC, the 2038 Bug will occurs.
            // That is that value will overflow back to Unix epoch.
            // Too bad.
            // This is the posix formula for approximated Unix time.
            let seconds_since_epoch = tm_sec
                + tm_min * 60
                + tm_hour * 3600
                + tm_yday * 86400
                + (tm_year - 70) * 31536000
            // + ((tm_year - 69) / 4) * 86400
            // - ((tm_year - 1) / 100) * 86400
            // + ((tm_year + 299) / 400) * 86400 // How the fuck, does RTC count leapdays.
                ;

            let old = ctx.current_unix_time.load(Ordering::SeqCst);

            assert!(
                old < seconds_since_epoch,
                "We want back in time, Congratulations!"
            );

            ctx.current_unix_time
                .store(seconds_since_epoch, Ordering::SeqCst);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
