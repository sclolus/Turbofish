//! This file contains the main function of the module

use kernel_modules::{
    KernelSymbolList, KeyboardReturn, ModConfig, ModError, ModResult, ModReturn, ModSpecificReturn,
    SymbolList,
};

use keyboard::keysymb::KeySymb;
use keyboard::{CallbackKeyboard, KeyboardDriver, Ps2Controler};

use kernel_modules::{Irq, MessageTo};

static mut CTX: Option<Ctx> = None;

/// Main Context of the module
struct Ctx {
    keyboard_driver: KeyboardDriver,
    ps2_controler: Ps2Controler,
    enable_irq: fn(Irq, unsafe extern "C" fn()),
    disable_irq: fn(Irq),
    send_fn: fn(MessageTo),
    kernel_symbol_list: KernelSymbolList,
}

/// Main Context implementation
impl Ctx {
    /// New fn
    fn new(
        enable_irq: fn(Irq, unsafe extern "C" fn()),
        disable_irq: fn(Irq),
        send_fn: fn(MessageTo),
        kernel_symbol_list: KernelSymbolList,
    ) -> Self {
        print!("New Keyboard Context created !");
        Self {
            keyboard_driver: KeyboardDriver::new(None),
            ps2_controler: Ps2Controler::new(),
            enable_irq,
            disable_irq,
            send_fn,
            kernel_symbol_list,
        }
    }
}

/// Drop boilerplate implementation
impl Drop for Ctx {
    fn drop(&mut self) {
        print!("Keyboard Context droped !");
    }
}

/// Constructor
pub fn module_start(symtab_list: SymbolList) -> ModResult {
    unsafe {
        kernel_modules::init_config(&symtab_list, &mut super::MEMORY_MANAGER);
    }
    if let ModConfig::Keyboard(keyboard_config) = symtab_list.kernel_callback {
        unsafe {
            CTX = Some(Ctx::new(
                keyboard_config.enable_irq,
                keyboard_config.disable_irq,
                keyboard_config.callback,
                symtab_list.kernel_symbol_list,
            ));
        }

        // Register the keyboard callback
        unsafe {
            CTX.as_mut()
                .unwrap()
                .keyboard_driver
                .bind(CallbackKeyboard::RequestKeySymb(handle_key_press));
            without_interrupts!({
                (CTX.as_ref().unwrap().enable_irq)(
                    Irq::KeyboardController,
                    keyboard_interrupt_handler,
                );
                // Just do a test for kernel symbol list
                let ksymbol_test = CTX
                    .as_ref()
                    .unwrap()
                    .kernel_symbol_list
                    .get_entry("symbol_list_test");
                if let Some(addr) = ksymbol_test {
                    let f: fn() = core::mem::transmute(addr);
                    f();
                } else {
                    print!("Symbol Test Not Founded !");
                }
            });
        }

        Ok(ModReturn {
            stop: drop_module,
            configurable_callbacks_opt: None,
            spec: ModSpecificReturn::Keyboard(KeyboardReturn { reboot_computer }),
        })
    } else {
        Err(ModError::BadIdentification)
    }
}

/// Use the PS/2 controler to reboot the computer
fn reboot_computer() {
    unsafe {
        CTX.as_mut().unwrap().ps2_controler.reboot_computer();
    }
}

/// Destructor
fn drop_module() {
    unsafe {
        without_interrupts!({
            (CTX.as_ref().unwrap().disable_irq)(Irq::KeyboardController);
        });
        CTX = None;
    }
}

/// Global Keyboard interrupt handler
#[no_mangle]
unsafe extern "C" fn keyboard_interrupt_handler() {
    if let Some(ctx) = CTX.as_mut() {
        let scancode = ctx.ps2_controler.read_scancode();
        if let Some(scancode) = scancode {
            ctx.keyboard_driver.interrupt_handler(scancode);
        }
    }
}

/// we send a message
pub fn handle_key_press(key_pressed: KeySymb) {
    // in the keyboard interrupt handler, after reading the keysymb,
    // we send a message to the tty which will be handled in the next
    // schedule
    unsafe { (CTX.as_ref().unwrap().send_fn)(MessageTo::Tty { key_pressed }) }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
