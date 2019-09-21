//! Get current Date using the
//! [CMOS](https://wiki.osdev.org/CMOS#Getting_Current_Date_and_Time_from_RTC) ram on the RTC chip.
use crate::drivers::{pic_8259, Nmi, PIC_8259};
use bit_field::BitField;
use core::cmp::max;
use core::convert::{TryFrom, TryInto};
use core::sync::atomic::{AtomicU32, Ordering};
use core::{fmt, fmt::Display};
use interrupts::{GateType, IdtGateEntry, InterruptTable};
use io::{Io, Pio};

extern "C" {
    fn _isr_cmos();
}

#[derive(Debug, Copy, Clone)]
#[allow(dead_code)]
enum Month {
    January = 1,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

#[derive(Debug, PartialEq, Eq)]
enum RtcRegister {
    /// Contains the current number of seconds of the RTC date. Ranges from 0 to 59.
    Seconds = 0x0,

    /// Contains the current number of minutes of the RTC date. Ranges from 0 to 59.
    Minutes = 0x2,

    /// Contains the current number of hours of the RTC date.
    /// Ranges from 0 to 23 in 24-hour mode.
    /// Ranges from 1 to 12 in 12-hour mode. Highest bit set if pm.
    Hours = 0x4,

    /// Contains the current day of the week of the RTC date. Ranges from 1 to 7, Sunday is defined as 1.
    #[allow(unused)]
    Weekday = 0x6,

    /// Contains the current day of the month of the RTC date. Ranges from 1 to 31.
    DayOfMonth = 0x7,

    /// Contains the current month of the RTC date. Ranges from 1 to 12.
    Month = 0x8,

    /// Contains the current year of the century of the RTC date. Ranges from 0 to 99.
    Year = 0x9,

    /// Contains the current century of the RTC date. Ranges from 19 to 20 (Well we don't really know that.)
    /// The century register is not present on all RTC chips. In order to check it's existence (and possibly location),
    /// it is needed to check the Fixed ACPI Description Table.
    #[allow(unused)]
    Century = 0x32,

    StatusA = 0xA,
    StatusB = 0xB,
    StatusC = 0xC,
}

#[derive(Debug)]
pub struct InvalidMonth(());

impl TryFrom<u8> for Month {
    type Error = InvalidMonth;
    fn try_from(n: u8) -> Result<Self, Self::Error> {
        use Month::*;
        match n {
            1 => Ok(January),
            2 => Ok(February),
            3 => Ok(March),
            4 => Ok(April),
            5 => Ok(May),
            6 => Ok(June),
            7 => Ok(July),
            8 => Ok(August),
            9 => Ok(September),
            10 => Ok(October),
            11 => Ok(November),
            12 => Ok(December),
            _ => Err(InvalidMonth(())),
        }
    }
}

/// date with all field in binary
#[derive(Debug, Copy, Clone)]
pub struct Date {
    sec: u8,
    minutes: u8,
    hours: u8,
    month: Month,
    day_of_month: u8,
    year: u32,
}

impl Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {:?} {} {}h{:>02}:{:>02}s",
            self.day_of_month, self.month, self.year, self.hours, self.minutes, self.sec
        )
    }
}

/// Real Time Clock interface
pub struct Rtc {
    /// used to select register (port 0x70)
    register_selector: Pio<u8>,
    /// used to read register (port 0x71)
    data: Pio<u8>,
}

/// Gets the day's day number for this year.
/// I tried to make this a const fn, but currently conditionals are not allowed in const fns.
fn get_day_number(month: Month, day: usize) -> usize {
    const NUMBER_OF_DAYS: [usize; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

    // assert!(day <= 31); // Can't do that in const fn, don't mess up!
    let month_index = month as usize - 1;

    let mut days = day;
    for index in 0..month_index {
        days += NUMBER_OF_DAYS[index];
    }
    days
}

pub static mut CURRENT_UNIX_TIME: AtomicU32 = AtomicU32::new(0);

#[no_mangle]
/// The interrupt handler of the RTC, updates the CURRENT_UNIX_TIME atomic variable
/// with the updated value from the RTC.
extern "C" fn rtc_handler(_interrupt_name: *const u8) {
    let mut rtc = Rtc::new();

    let status = rtc.read_register(RtcRegister::StatusC, false);

    // The end-of-update interrupt is marked in the StatusC register by the 4 higher-bits being set to 0xd0.
    if status.get_bits(4..8) == 0xd {
        let date = rtc.read_date();
        // Heuristical way to determine the current century.
        // As we would need to check the ACPI tables to assert the existence of the Century register.

        let tm_sec = date.sec as u32;
        let tm_min = date.minutes as u32;
        let tm_hour = date.hours as u32;
        let tm_yday = get_day_number(date.month, date.day_of_month as usize) as u32;
        let tm_year = date.year - 1900 as u32;

        // The 19 January 2038, at 3am:14:08 UTC, the 2038 Bug will occurs.
        // That is that value will overflow back to Unix epoch.
        // Too bad.
        // This is the posix formula for approximated Unix time.
        let seconds_since_epoch = tm_sec
            + tm_min * 60
            + tm_hour * 3600
            + tm_yday * 86400
            + (tm_year - 70) * 31536000
            // + ((tm_year - 69) / 4) * 86400
            // - ((tm_year - 1) / 100) * 86400
            // + ((tm_year + 299) / 400) * 86400 // How the fuck, does RTC count leapdays.
            ;

        unsafe {
            let old = CURRENT_UNIX_TIME.load(Ordering::SeqCst);

            assert!(
                old < seconds_since_epoch,
                "We want back in time, Congratulations!"
            );

            CURRENT_UNIX_TIME.store(seconds_since_epoch, Ordering::SeqCst);
        }
    }
}

impl Rtc {
    pub const fn new() -> Self {
        Self {
            register_selector: Pio::new(0x70),
            data: Pio::new(0x71),
        }
    }

    /// This sets the register index of the RTC/CMOS to the `selected_register`.
    /// `disable_nmi` lets you disable the nmi and read the register in one operation.
    fn set_register_index(&mut self, selected_register: RtcRegister, disable_nmi: bool) {
        let mut index = selected_register as u8;

        if disable_nmi {
            index |= 0x80;
        }
        self.register_selector.write(index);
    }

    /// REGISTER  FIELD
    /// 0x00      Seconds
    /// 0x02      Minutes
    /// 0x04      Hours
    /// 0x06      Weekday
    /// 0x07      Day of Month
    /// 0x08      Month
    /// 0x09      Year
    /// 0x0B      Status Register B: bit 1 (is 24 hour format) bit 2 (is binary mode)
    /// `disable_nmi` lets you disable the nmi and read the register in one operation.
    fn read_register(&mut self, selected_register: RtcRegister, disable_nmi: bool) -> u8 {
        self.set_register_index(selected_register, disable_nmi);
        self.data.read()
    }

    /// This function is unsafe, seriously, don't mess with it or you could fuck up your RTC permanently.
    #[allow(unused)]
    unsafe fn set_register(&mut self, value: u8, selected_register: RtcRegister) {
        without_interrupts!({
            Nmi::disable();
            let index = selected_register as u8;

            self.register_selector.write(index);
            self.data.write(value);
            Nmi::enable();
        })
    }

    /// Enables the periodic interrupts of the RTC at a given rate from 3 to 15 (15 is the lowest rate at 2Hz).
    /// It sets the corresponding interrupt handler in the Interrupt Table (IDT),
    /// and enable the IRQ 8 on the PIC.
    pub fn enable_periodic_interrupts(&mut self, mut rate: u8) {
        rate &= 0x0F; // Ensure that rate is below 16.
        rate = max(3, rate); // Ensure that rate is above 2.

        let mut interrupt_table = unsafe { InterruptTable::current_interrupt_table() };

        let gate_entry = *IdtGateEntry::new()
            .set_storage_segment(false)
            .set_privilege_level(0)
            .set_selector(1 << 3)
            .set_gate_type(GateType::InterruptGate32)
            .set_handler(_isr_cmos as *const u8 as u32);
        unsafe {
            without_interrupts!({
                // Sets handler into the InterruptTable.
                interrupt_table[32 + 8] = gate_entry;

                PIC_8259.lock().enable_irq(pic_8259::Irq::RealTimeClock); // enables the RTC irq.

                let previous_value = self.read_register(RtcRegister::StatusB, true);
                self.set_register_index(RtcRegister::StatusB, true);
                // The bit 6 of Status register B enables the periodic interrupts of the RTC.
                self.data.write(previous_value | 0x40);

                let previous_value = self.read_register(RtcRegister::StatusA, true);
                self.set_register_index(RtcRegister::StatusA, true);

                // The 4 low bits of the Status Register A are the `divider setting`, that is, the rate selector, if you will.
                self.data.write((previous_value & 0xF0) | rate);

                // We need to reenable the NMI here.
                // The NMI was disabled by the calls to `read_register` method.
                // The thing is that if we don't explicitly set the high bit (0x80)
                // on each Port I/O writes, the NMI would be incorrectly reenabled.
                // The symmetry, in this case, really is not obtainable.
                Nmi::enable();
            });
        }
    }

    pub fn read_date(&mut self) -> Date {
        let format = self.read_register(RtcRegister::StatusB, false);
        let is_24hour_format = format.get_bit(1);
        let is_binary_format = format.get_bit(2);

        //println!("format is_24hour{:?} is_binary{:?}", is_24hour_format, is_binary_format);

        let convert_to_binary = |x| {
            if is_binary_format {
                x
            } else {
                ((x / 16) * 10) + (x & 0xF)
            }
        };
        let convert_to_binary_24hour = |mut x: u8| {
            if is_24hour_format {
                convert_to_binary(x)
            } else {
                let pm = x.get_bit(7);
                if pm {
                    (convert_to_binary(*x.set_bit(7, false)) + 12) % 24
                } else {
                    convert_to_binary(x)
                }
            }
        };

        use RtcRegister::*;
        Date {
            sec: convert_to_binary(self.read_register(Seconds, false)),
            minutes: convert_to_binary(self.read_register(Minutes, false)),
            hours: convert_to_binary_24hour(self.read_register(Hours, false)),
            day_of_month: convert_to_binary(self.read_register(DayOfMonth, false)),
            month: self.read_register(Month, false).try_into().unwrap(),
            year: {
                let year: u32 = convert_to_binary(self.read_register(Year, false)) as u32;

                // Heuristical way to determine the current century.
                // As we would need to check the ACPI tables to assert the existence of the Century register.
                if year > 90 {
                    1900 + year
                } else {
                    2000 + year
                }
            },
        }
    }
}
