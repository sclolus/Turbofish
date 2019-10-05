//! This file contains the main function of the module

use kernel_modules::{
    ModConfig, ModError, ModResult, ModReturn, ModSpecificReturn, RTCReturn, SymbolList,
};

use bit_field::BitField;
use core::sync::atomic::{AtomicU32, Ordering};
use kernel_modules::Irq;

use rtc_toolkit::{Rtc, RtcRegister};
use time::Date;

static mut CTX: Option<Ctx> = None;

enum Status {
    Clear,
    MonthIsBullshit,
    WantBackInTime,
}

/// Main Context of the module
struct Ctx {
    enable_irq: fn(Irq, unsafe extern "C" fn()),
    disable_irq: fn(Irq),
    current_unix_time: &'static AtomicU32,
    current_date: Date,
    status: Status,
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
            current_date: Date::default(),
            status: Status::Clear,
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
        kernel_modules::init_config(&symtab_list, &mut super::MEMORY_MANAGER);
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
fn read_date() -> Date {
    if let Some(ctx) = unsafe { CTX.as_ref() } {
        ctx.current_date
    } else {
        Date::default()
    }
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
            match rtc.read_date() {
                Ok(date) => {
                    let seconds_since_epoch = date.into();
                    let old = ctx.current_unix_time.load(Ordering::SeqCst);

                    if old > seconds_since_epoch {
                        // We want back in time, Congratulations!
                        ctx.status = Status::WantBackInTime;
                        return;
                    }

                    ctx.current_date = date;
                    ctx.current_unix_time
                        .store(seconds_since_epoch, Ordering::SeqCst);
                }
                Err(_) => {
                    // Protect code about month format bullshit
                    ctx.status = Status::MonthIsBullshit;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
