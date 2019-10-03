//! This crate provides a toolkit to measure time

use core::time::Duration;

use crate::drivers::PIT0;

/// Get the new read time stamp counter
#[inline(always)]
fn get_cpu_time() -> u64 {
    let mut eax: u32;
    let mut edx: u32;

    unsafe {
        asm!("rdtsc" : "={eax}"(eax), "={edx}"(edx));
    }
    ((edx as u64) << 32) + eax as u64
}

/// Main Global Time structure
pub struct GlobalTime {
    cpu_frequency: u64,
    last_mesured_time: Option<u64>,
}

/// Main Global Time implementation
impl GlobalTime {
    const TEST_PERIOD_DIVISOR: u64 = 10;

    /// Create a new global time context. Get the CPU-FREQUENCY
    pub fn new() -> Self {
        let old_cpu_time = get_cpu_time();
        PIT0.lock()
            .sleep(Duration::from_millis(1000 / Self::TEST_PERIOD_DIVISOR));
        let new_cpu_time = get_cpu_time();
        let cpu_frequency = (new_cpu_time - old_cpu_time) * Self::TEST_PERIOD_DIVISOR;
        log::info!("CPU FREQUENCY DETECTED: {} mhz", cpu_frequency / 1000000);
        Self {
            cpu_frequency,
            last_mesured_time: None,
        }
    }

    /// Initialize the Timer
    pub fn init(&mut self) {
        self.last_mesured_time = Some(get_cpu_time());
    }

    /// Set the new time and get the duration between the last call
    pub fn get_time(&mut self) -> Option<Duration> {
        self.last_mesured_time.map(|old_cpu_time| {
            let new_cpu_time = get_cpu_time();
            self.last_mesured_time = Some(new_cpu_time);
            Duration::from_micros((new_cpu_time - old_cpu_time) * 1000000 / self.cpu_frequency)
        })
    }
}

/// Main globale
pub static mut GLOBAL_TIME: Option<GlobalTime> = None;
