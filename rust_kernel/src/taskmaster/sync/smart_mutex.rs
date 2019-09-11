//! This file contains a smart mutex with dump backtrace of last locker feature
use core::sync::atomic::{AtomicU32, Ordering};
use lock_api::{GuardSend, RawMutex};

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
unsafe impl RawMutex for RawSmartMutex {
    const INIT: RawSmartMutex = RawSmartMutex(AtomicU32::new(0));

    // A spinlock guard can be sent to another thread and unlocked there
    type GuardMarker = GuardSend;

    /// Lock the mutex
    fn lock(&self) {
        if !self.try_lock() {
            panic!("Dead lock");
        }
    }

    /// Try to lock the mutex
    fn try_lock(&self) -> bool {
        let current_ebp: u32;

        unsafe {
            asm!("mov %ebp, %eax" : "={eax}"(current_ebp):);
        }
        let ebp = self.0.compare_and_swap(0, current_ebp, Ordering::Relaxed) as *const u32;
        if ebp != 0 as *const u32 {
            // Here a DeadLock, we trace back the process which had put his EBP in the mutex
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

// 3. Export the wrappers. This are the types that your users will actually use.
pub type SmartMutex<T> = lock_api::Mutex<RawSmartMutex, T>;
pub type SmartMutexGuard<'a, T> = lock_api::MutexGuard<'a, RawSmartMutex, T>;
