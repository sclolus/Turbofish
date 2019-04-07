//! This file contains the code related to the interrupt managing system, i.e the InterruptManger

use alloc::boxed::Box;
// use alloc::vec::Vec;
use HandlingState::*;

pub static mut INTERRUPT_MANAGER: Option<Manager> = None;

pub unsafe extern "C" fn generic_handler(interrupt_number: u8) {
    assert!(INTERRUPT_MANAGER.is_some());
    match INTERRUPT_MANAGER.as_mut().unwrap().dispatch(interrupt_number) {
        Handled => (),
        NotHandled => log::warn!("Interrupt of number {} was not handled", interrupt_number),
    }
}

pub struct Manager {
    // 256 entries in the IDT, put a constant here please.
    handlers: [Option<Box<InterruptHandler>>; 256],
}

pub trait InterruptManager {
    fn dispatch(&mut self, _interrupt_number: u8) -> HandlingState {
        NotHandled
    }

    fn register(&mut self, handler: Box<InterruptHandler>) -> Result<(), ()>;
}

impl Manager {
    pub fn new() -> Self {
        use core::mem;
        let mut handlers: [Option<Box<InterruptHandler>>; 256] = unsafe { core::mem::uninitialized() };

        for handler in handlers.iter_mut() {
            mem::forget(mem::replace(handler, None));
        }

        Self { handlers }
    }
}

impl InterruptManager for Manager {
    fn dispatch(&mut self, interrupt_number: u8) -> HandlingState {
        match &mut self.handlers[interrupt_number as usize] {
            Some(handler) => handler.handle(interrupt_number),
            None => {
                log::warn!("No handled registered for interrupt number {}", interrupt_number);
                NotHandled
            }
        }
    }

    fn register(&mut self, handler: Box<InterruptHandler>) -> Result<(), ()> {
        let interrupt_number = handler.interrupt_number();

        match &mut self.handlers[interrupt_number as usize] {
            Some(handler) => {
                log::warn!(
                    "Handler {} for interrupt number {} is already registered",
                    handler.name(),
                    interrupt_number
                );
                return Err(());
            }
            None => self.handlers[interrupt_number as usize] = Some(handler),
        }
        Ok(())
    }
}

pub trait InterruptHandler {
    fn name(&self) -> &str {
        fn type_name_of<T: ?Sized>() -> &'static str {
            extern crate core;
            unsafe { core::intrinsics::type_name::<T>() }
        }

        type_name_of::<Self>()
    }

    fn handle(&mut self, _interrupt_number: u8) -> HandlingState {
        NotHandled
    }

    fn interrupt_number(&self) -> u8;
}

pub enum HandlingState {
    Handled,
    NotHandled,
}

pub struct DummyHandler {
    interrupt_number: u8,
}

impl DummyHandler {
    pub fn new(interrupt_number: u8) -> Self {
        Self { interrupt_number }
    }
}

impl InterruptHandler for DummyHandler {
    fn handle(&mut self, _interrupt_number: u8) -> HandlingState {
        println!("Dummy Handler was dispatched");
        NotHandled
    }

    fn interrupt_number(&self) -> u8 {
        self.interrupt_number
    }
}
