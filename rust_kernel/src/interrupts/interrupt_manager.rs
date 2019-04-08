//! This file contains the code related to the interrupt managing system, i.e the InterruptManger

use crate::utils::{Either, Either::*};
use alloc::boxed::Box;
use alloc::vec::Vec;
use HandlingState::*;

pub static mut INTERRUPT_MANAGER: Option<Manager> = None;

pub unsafe extern "C" fn generic_handler(interrupt_number: u8) {
    assert!(INTERRUPT_MANAGER.is_some());
    match INTERRUPT_MANAGER.as_mut().unwrap().dispatch(interrupt_number) {
        Handled => (),
        NotHandled => log::warn!("Interrupt of number {} was not handled", interrupt_number),
    }
}

/// The type of the interrupt manager, which centralises the interrupts.
/// The Manager dispatch the interrupts to the registered InterruptHandler implementors.
/// The Manager implements InterruptManager.
/// The InterruptHandler implementors can also implement InterruptManager,
/// enabling them to further dispatch the interrupt to a list of registered InterruptHandler.
pub struct Manager {
    // 256 entries in the IDT, put a constant here please.
    handlers: [Option<Box<InterruptHandler>>; 256],
}

/// The InterruptManager trait.
/// The Manager implements it.
pub trait InterruptManager {
    fn dispatch(&mut self, _interrupt_number: u8) -> HandlingState {
        NotHandled
    }

    fn register(&mut self, handler: Box<InterruptHandler>, interrupt_number: u8) -> Result<(), ()>;
}

impl<T> InterruptHandler for T
where
    T: InterruptManager,
{
    fn handle(&mut self, interrupt_number: u8) -> HandlingState {
        self.dispatch(interrupt_number)
    }

    fn kind(&mut self) -> Either<&mut dyn InterruptHandler, &mut dyn InterruptManager> {
        Right(self)
    }
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

    fn register(&mut self, handler: Box<InterruptHandler>, interrupt_number: u8) -> Result<(), ()> {
        match &mut self.handlers[interrupt_number as usize] {
            Some(registered_handler) => registered_handler
                .kind()
                .map_left(|handler| {
                    log::warn!(
                        "Handler {} for interrupt number {} is already registered",
                        handler.name(),
                        interrupt_number
                    );
                    Err(())
                })
                .map_right(|interrupt_manager| interrupt_manager.register(handler, interrupt_number))
                .move_out()?,
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

    // fn interrupt_number(&self) -> u8;
    fn kind(&mut self) -> Either<&mut dyn InterruptHandler, &mut dyn InterruptManager>; // {
                                                                                        //     Left(&self)
                                                                                        // }
}

// Please, find a way to make this work.
// impl<T: FnMut(u8) -> HandlingState> InterruptHandler for Box<T> {
//     fn handle(&mut self, interrupt_number: u8) -> HandlingState {
//         self(interrupt_number)
//     }

//     fn kind(&mut self) -> Either<&mut dyn InterruptHandler, &mut dyn InterruptManager> {
//         Left(self)
//     }
// }

pub enum HandlingState {
    Handled,
    NotHandled,
}

pub struct DummyHandler;

impl DummyHandler {
    pub fn new() -> Self {
        Self
    }
}

impl InterruptHandler for DummyHandler {
    fn handle(&mut self, _interrupt_number: u8) -> HandlingState {
        println!("Dummy Handler was dispatched");
        NotHandled
    }

    fn kind(&mut self) -> Either<&mut dyn InterruptHandler, &mut dyn InterruptManager> {
        Left(self)
    }
}

pub struct GenericManager {
    handlers: Vec<Box<InterruptHandler>>,
}

impl GenericManager {
    pub fn new() -> Self {
        Self { handlers: Vec::new() }
    }
}

impl InterruptManager for GenericManager {
    fn dispatch(&mut self, interrupt_number: u8) -> HandlingState {
        for handler in self.handlers.iter_mut() {
            if let Handled = handler.handle(interrupt_number) {
                return Handled;
            }
        }
        NotHandled
    }

    fn register(&mut self, handler: Box<InterruptHandler>, _interrupt_number: u8) -> Result<(), ()> {
        Ok(self.handlers.push(handler))
    }
}

/// So this is the abstraction used for making InterruptHandler from closures.
/// It's unfortunate but the above-tried generic implementation of InterruptHandler for FnMut(u8) -> HandlingState does not work.
pub struct FnHandler {
    callback: Box<FnMut(u8) -> HandlingState>,
}

impl InterruptHandler for FnHandler {
    fn handle(&mut self, interrupt_number: u8) -> HandlingState {
        (self.callback)(interrupt_number)
    }

    fn kind(&mut self) -> Either<&mut dyn InterruptHandler, &mut dyn InterruptManager> {
        Left(self)
    }
}

impl FnHandler {
    pub fn new(callback: Box<FnMut(u8) -> HandlingState>) -> Self {
        FnHandler { callback }
    }
}

// impl From<Box<FnMut(u8) -> HandlingState>> for FnHandler {
//     fn from(callback: Box<FnMut(u8) -> HandlingState>) -> Self {
//         FnHandler { callback }
//     }
// }
