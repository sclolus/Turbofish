//! This file contains the main function of the module

use kernel_modules::{
    KeyboardReturn, ModConfig, ModError, ModResult, ModReturn, ModSpecificReturn, SymbolList,
    WRITER,
};

use keyboard::keysymb::KeySymb;
use keyboard::{CallbackKeyboard, KeyboardDriver, Ps2Controler};

use kernel_modules::{Irq, MessageTo};

static mut CTX: Option<Ctx> = None;

/// Main Context of the module
struct Ctx {
    keyboard_driver: KeyboardDriver,
    ps2_controler: Ps2Controler,
    set_idt_entry: fn(Irq, Option<unsafe extern "C" fn()>),
    send_fn: fn(MessageTo),
}

/// Main Context implementation
impl Ctx {
    /// New fn
    fn new(set_idt_entry: fn(Irq, Option<unsafe extern "C" fn()>), send_fn: fn(MessageTo)) -> Self {
        print!("New Keyboard Context created !");
        Self {
            keyboard_driver: KeyboardDriver::new(None),
            ps2_controler: Ps2Controler::new(),
            set_idt_entry,
            send_fn,
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
pub fn rust_main(symtab_list: SymbolList) -> ModResult {
    unsafe {
        WRITER.set_write_callback(symtab_list.write);
        #[cfg(not(test))]
        crate::MEMORY_MANAGER.set_methods(symtab_list.alloc_tools);
    }
    if let ModConfig::Keyboard(keyboard_config) = symtab_list.kernel_callback {
        unsafe {
            CTX = Some(Ctx::new(
                keyboard_config.set_idt_entry,
                keyboard_config.callback,
            ));
        }

        // Register the keyboard callback
        unsafe {
            CTX.as_mut()
                .unwrap()
                .keyboard_driver
                .bind(CallbackKeyboard::RequestKeySymb(handle_key_press));
            (CTX.as_ref().unwrap().set_idt_entry)(
                Irq::KeyboardController,
                Some(keyboard_interrupt_handler),
            );
        }

        Ok(ModReturn {
            stop: drop_module,
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
        (CTX.as_ref().unwrap().set_idt_entry)(Irq::KeyboardController, None);
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
