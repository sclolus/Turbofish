//! This crate provides a toolkit to measure time

use core::ops::AddAssign;
use core::time::Duration;
use libc_binding::{rusage, timeval};

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

#[derive(Debug)]
/// Main Global Time structure
pub struct GlobalTime {
    cpu_frequency: u64,
    last_mesured_time: Option<u64>,
    global_user_time: Duration,
    global_system_time: Duration,
    global_idle_time: Duration,
    process_user_time: Duration,
    process_system_time: Duration,
}

/// Kind of Times
pub enum TimeSession {
    User,
    System,
    Idle,
}

#[derive(Copy, Clone, Debug)]
/// Each process got his user time and his system time
pub struct ProcessDuration {
    user_time: Duration,
    system_time: Duration,
}

impl ProcessDuration {
    pub fn user_time(&self) -> Duration {
        self.user_time
    }

    pub fn system_time(&self) -> Duration {
        self.system_time
    }
}

/// Default boilerplate for ProcessDuration
impl Default for ProcessDuration {
    fn default() -> Self {
        Self {
            user_time: Duration::default(),
            system_time: Duration::default(),
        }
    }
}

/// AddAssign boilerplate for ProcessDuration
impl AddAssign for ProcessDuration {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            user_time: self.user_time + other.user_time,
            system_time: self.system_time + other.system_time,
        }
    }
}

/// From boilerplate from ProcessDuration to rusage
impl From<ProcessDuration> for rusage {
    fn from(process_duration: ProcessDuration) -> Self {
        Self {
            ru_utime: timeval {
                tv_sec: process_duration.user_time.as_secs() as i32,
                tv_usec: process_duration.user_time.subsec_micros(),
            },
            ru_stime: timeval {
                tv_sec: process_duration.system_time.as_secs() as i32,
                tv_usec: process_duration.system_time.subsec_micros(),
            },
        }
    }
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
            global_user_time: Duration::default(),
            global_system_time: Duration::default(),
            global_idle_time: Duration::default(),
            process_user_time: Duration::default(),
            process_system_time: Duration::default(),
        }
    }

    /// Initialize the Timer
    pub fn init(&mut self) {
        self.last_mesured_time = Some(get_cpu_time());
    }

    /// Update the system global time
    pub fn update_global_time(&mut self, session: TimeSession) {
        use TimeSession::*;
        let duration = self.get_time().expect("Woot ?");
        match session {
            User => {
                self.global_user_time += duration;
                self.process_user_time += duration;
            }
            System => {
                self.global_system_time += duration;
                self.process_system_time += duration;
            }
            Idle => self.global_idle_time += duration,
        }
    }

    /// Get the Time Summary for a process and reset it
    pub fn get_process_time(&mut self) -> ProcessDuration {
        let res: ProcessDuration = ProcessDuration {
            user_time: self.process_user_time,
            system_time: self.process_system_time,
        };
        self.process_user_time = Duration::default();
        self.process_system_time = Duration::default();
        res
    }

    /// Set the new time and get the duration between the last call
    fn get_time(&mut self) -> Option<Duration> {
        self.last_mesured_time.map(|old_cpu_time| {
            let new_cpu_time = get_cpu_time();
            self.last_mesured_time = Some(new_cpu_time);
            Duration::from_micros((new_cpu_time - old_cpu_time) * 1000000 / self.cpu_frequency)
        })
    }

    pub fn global_user_time(&self) -> Duration {
        self.global_user_time
    }

    pub fn global_system_time(&self) -> Duration {
        self.global_system_time
    }

    pub fn global_idle_time(&self) -> Duration {
        self.global_idle_time
    }

    pub fn cpu_frequency(&self) -> u64 {
        self.cpu_frequency
    }
}

/// Main globale
pub static mut GLOBAL_TIME: Option<GlobalTime> = None;
