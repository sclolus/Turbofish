//! This file contains the main function of the module

use kernel_modules::{
    ModConfig, ModError, ModResult, ModReturn, ModSpecificReturn, SymbolList, WRITER,
};

use kernel_modules::Irq;

static mut CTX: Option<Ctx> = None;

/// Main Context of the module
struct Ctx {
    set_idt_entry: fn(Irq, Option<unsafe extern "C" fn()>),
}

/// Main Context implementation
impl Ctx {
    /// New fn
    fn new(set_idt_entry: fn(Irq, Option<unsafe extern "C" fn()>)) -> Self {
        print!("New RTC Context created !");
        Self { set_idt_entry }
    }
}

/// Drop boilerplate implementation
impl Drop for Ctx {
    fn drop(&mut self) {
        print!("RTC Context droped !");
    }
}

/// Constructor
pub fn rust_main(symtab_list: SymbolList) -> ModResult {
    unsafe {
        WRITER.set_write_callback(symtab_list.write);
        #[cfg(not(test))]
        crate::MEMORY_MANAGER.set_methods(symtab_list.alloc_tools);
    }
    if let ModConfig::RTC(rtc_config) = symtab_list.kernel_callback {
        unsafe {
            CTX = Some(Ctx::new(rtc_config.set_idt_entry));
        }

        // Register the RTC callback
        unsafe {
            (CTX.as_ref().unwrap().set_idt_entry)(Irq::RealTimeClock, Some(rtc_interrupt_handler));
        }

        Ok(ModReturn {
            stop: drop_module,
            spec: ModSpecificReturn::RTCReturn,
        })
    } else {
        Err(ModError::BadIdentification)
    }
}

/// Destructor
fn drop_module() {
    unsafe {
        CTX = None;
    }
}

/// Global RTC interrupt handler
#[no_mangle]
unsafe extern "C" fn rtc_interrupt_handler() {
    if let Some(_ctx) = CTX.as_mut() {}
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
