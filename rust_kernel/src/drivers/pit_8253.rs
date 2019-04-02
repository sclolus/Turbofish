//! This files contains the code related to the Programmable Interval Timer
//! ([PIT](https://wiki.osdev.org/Programmable_Interval_Timer)) chip (also called an 8253/8254 chip)
use super::{pic_8259, PIC_8259};
use crate::interrupts;
use crate::Spinlock;
use bit_field::BitField;
use core::time::Duration;
use io::{Io, Pio};
use lazy_static::lazy_static;

#[derive(Debug)]
pub struct Pit {
    /// The PIT's command port.
    command: Pio<u8>,

    /// The PIT's data port.
    data: Pio<u8>,

    channel: Channel,

    /// stock configured operating mode
    operating_mode: Option<OperatingMode>,

    /// period between 2 interrupts in s
    pub period: f32,
}

lazy_static! {
    pub static ref PIT0: Spinlock<Pit> = Spinlock::new(Pit::new(Channel::Channel0));
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Channel {
    Channel0,
    Channel1,
    Channel2,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum AccessMode {
    LatchCount,
    LobyteOnly,
    HibyteOnly,
    LobyteHibyte,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum OperatingMode {
    InterruptOnTerminal,
    OneShot,
    RateGenerator,
    SquareWaveGenerator,
    SoftwareTriggeredStrobe,
    HardwareTriggeredStrobe,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PitError {
    PicNotInitialized,
    BadFrequency,
}

#[no_mangle]
pub fn debug_pit(tic: u32) -> () {
    print!("{} ", tic);
}

extern "C" {
    fn _sleep(next_tic: u32) -> ();
}

impl Pit {
    /// Channel 0 data port (read/write)
    const CHANEL0_PORT: u16 = 0x40;

    /// Mode/Command register (write only, a read is ignored)
    const COMMAND_PORT: u16 = 0x43;

    /// period in second if the pit is configured at its highest frequency
    const PERIOD_MIN: f32 = 0.00000083809534;

    /// base frequency wich is also the highest frequency
    const BASE_FREQUENCY: f32 = 1193181.6666;

    pub const fn new(channel: Channel) -> Pit {
        Pit {
            command: Pio::new(Self::COMMAND_PORT),
            channel,
            data: Pio::new(Self::CHANEL0_PORT + (channel as u16)),
            operating_mode: None,
            period: 0.,
        }
    }

    ///Bits         Usage
    /// 6 and 7      Select channel :
    ///                 0 0 = Channel 0
    ///                 0 1 = Channel 1
    ///                 1 0 = Channel 2
    ///                 1 1 = Read-back command (8254 only)
    /// 4 and 5      Access mode :
    ///                 0 0 = Latch count value command
    ///                 0 1 = Access mode: lobyte only
    ///                 1 0 = Access mode: hibyte only
    ///                 1 1 = Access mode: lobyte/hibyte
    /// 1 to 3       Operating mode :
    ///                 0 0 0 = Mode 0 (interrupt on terminal count)
    ///                 0 0 1 = Mode 1 (hardware re-triggerable one-shot)
    ///                 0 1 0 = Mode 2 (rate generator)
    ///                 0 1 1 = Mode 3 (square wave generator)
    ///                 1 0 0 = Mode 4 (software triggered strobe)
    ///                 1 0 1 = Mode 5 (hardware triggered strobe)
    ///                 1 1 0 = Mode 2 (rate generator, same as 010b)
    ///                 1 1 1 = Mode 3 (square wave generator, same as 011b)
    /// 0            BCD/Binary mode: 0 = 16-bit binary, 1 = four-digit BCD
    pub fn configure(&mut self, operating_mode: OperatingMode) {
        let mut cmd = 0;
        cmd.set_bits(1..4, operating_mode as u8);
        cmd.set_bits(4..6, AccessMode::LobyteHibyte as u8);
        cmd.set_bits(6..8, self.channel as u8);
        self.operating_mode = Some(operating_mode);
        self.command.write(cmd);
    }

    /// get frequency in Hertz from 18.2065 Hz to Self::BASE_FREQUENCY (1.1931816666 MHz)
    /// if not in this range take the closest frequency
    /// freq = base_freq / divisor -> divisor = base_freq / freq
    pub fn start_at_frequency(&mut self, freq: f32) -> Result<(), PitError> {
        if freq < 18.0 || freq > Self::BASE_FREQUENCY {
            return Err(PitError::BadFrequency);
        }
        unsafe {
            if PIC_8259.lock().is_initialized() == false {
                return Err(PitError::PicNotInitialized);
            }
            PIC_8259.lock().disable_irq(pic_8259::Irq::SystemTimer);
        }
        let mut divisor = (Self::BASE_FREQUENCY / freq) as u32;
        if divisor > core::u16::MAX as u32 {
            divisor = core::u16::MAX as u32;
        } else if divisor == 0 {
            divisor = 1;
        }
        self.period = Self::PERIOD_MIN * divisor as f32;
        self.data.write(divisor.get_bits(0..8) as u8);
        self.data.write(divisor.get_bits(8..16) as u8);
        unsafe {
            PIC_8259.lock().enable_irq(pic_8259::Irq::SystemTimer);
        }
        Ok(())
    }

    /// assume that PIT is correctely configured, 8259 bit 0 is clear and interrupts are enable
    /// i'am not sure that it is easy to ensure the PIT is well configured
    pub fn sleep(&mut self, duration: Duration) -> () {
        assert!(interrupts::get_interrupts_state());
        use crate::math::convert::Convert;

        let ms = duration.as_millis();
        let next_tic = ms as f32 / 1000 as f32 / self.period;
        unsafe {
            _sleep(next_tic.round() as u32);
        }
    }
}
