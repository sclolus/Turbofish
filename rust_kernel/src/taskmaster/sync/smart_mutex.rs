//! This file contains a smart mutex with dump backtrace of last locker feature
use core::fmt::Debug;
use core::ops::{Deref, DerefMut};
use core::sync::atomic::{AtomicU32, Ordering};

use crate::ffi::c_str;

// 1. Define our raw lock type
#[derive(Debug)]
pub struct RawSmartMutex(AtomicU32);

/// Symbol structure is defined in C file
#[repr(C)]
struct Symbol {
    offset: u32,
    name: c_str,
}

extern "C" {
    fn _get_symbol(eip: u32) -> Symbol;
}

/// Trace structure
struct Trace {
    eip: u32,
    ebp: *const u32,
}

/// Get a trace
unsafe fn get_eip(ebp: *const u32) -> Option<Trace> {
    let eip = *ebp.add(1);
    match eip {
        0 => None,
        eip => Some(Trace {
            eip,
            ebp: *ebp as *const u32,
        }),
    }
}

/// Take the first eip and epb as parameter and trace back up.
unsafe fn trace_back(mut ebp: *const u32) {
    while let Some(trace) = get_eip(ebp) {
        let symbol = _get_symbol(trace.eip);
        eprintln!(
            "{:X?} : {:?}, eip={:X?}",
            symbol.offset, symbol.name, trace.eip
        );
        ebp = trace.ebp;
    }
}

// 2. Implement RawMutex for this type
impl RawSmartMutex {
    const INIT: RawSmartMutex = RawSmartMutex(AtomicU32::new(0));

    // A spinlock guard can be sent to another thread and unlocked there

    /// Try to lock the mutex
    fn try_lock(&self) -> bool {
        let current_ebp: u32;

        unsafe {
            asm!("mov %ebp, %eax" : "={eax}"(current_ebp):);
        }
        let ebp = self.0.compare_and_swap(0, current_ebp, Ordering::Relaxed) as *const u32;
        if ebp != 0 as *const u32 {
            // Here a DeadSmartMutex, we trace back the process which had put his EBP in the mutex
            eprintln!("--- Previous locker backtrace ----");
            unsafe {
                trace_back(ebp);
            }
            eprintln!("----------------------------------");
            false
        } else {
            true
        }
    }

    /// Release the mutex
    fn unlock(&self) {
        self.0.store(0, Ordering::Relaxed);
    }
}

pub struct SmartMutexGuard<'a, T: Debug>(&'a mut SmartMutex<T>);

impl<'a, T: Debug> Drop for SmartMutexGuard<'a, T> {
    fn drop(&mut self) {
        self.0.raw_lock.unlock();
    }
}

impl<'a, T: Debug> Deref for SmartMutexGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0.data
    }
}

impl<'a, T: Debug> DerefMut for SmartMutexGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0.data
    }
}

/// A lock Wrapper for a generic Datatype
pub struct SmartMutex<T: Debug> {
    data: T,
    raw_lock: RawSmartMutex,
}

impl<'a, T: Debug> SmartMutex<T> {
    pub fn new(data: T) -> Self {
        SmartMutex {
            data,
            raw_lock: RawSmartMutex::INIT,
        }
    }
    pub fn lock(&'a self) -> SmartMutexGuard<'a, T> {
        if !self.raw_lock.try_lock() {
            panic!("Dead lock {:?}", self.data);
        }
        SmartMutexGuard(unsafe { &mut *(self as *const Self as *mut Self) })
    }
    pub fn force_unlock(&'a self) {
        self.raw_lock.unlock();
    }
}

unsafe impl<T: Debug> Send for SmartMutex<T> {}
unsafe impl<T: Debug> Sync for SmartMutex<T> {}

// 3. Export the wrappers. This are the types that your users will actually use.
