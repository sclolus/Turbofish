
use crate::io::{Io, Pio};
use bit_field::BitField;
use core::convert::{TryFrom, TryInto};
use core::{fmt, fmt::Display};

#[derive(Debug)]
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
#[derive(Debug)]
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

impl Rtc {
    pub const fn new() -> Self {
        Self { register_selector: Pio::new(0x70), data: Pio::new(0x71) }
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
    fn read_register(&mut self, reg: u8) -> u8 {
        self.register_selector.write(reg);
        self.data.read()
    }

    pub fn read_date(&mut self) -> Date {
        let format = self.read_register(0x0B);
        let is_24hour_format = format.get_bit(1);
        let is_binary_format = format.get_bit(2);

        //println!("format is_24hour{:?} is_binary{:?}", is_24hour_format, is_binary_format);

        let convert_to_binary = |x| if is_binary_format { x } else { ((x / 16) * 10) + (x & 0xF) };
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

        Date {
            sec: convert_to_binary(self.read_register(0x00)),
            minutes: convert_to_binary(self.read_register(0x02)),
            hours: convert_to_binary_24hour(self.read_register(0x04)),
            day_of_month: self.read_register(0x07),
            month: self.read_register(0x08).try_into().unwrap(),
            year: {
                let year: u32 = convert_to_binary(self.read_register(0x09)) as u32;
                if year > 90 {
                    1900 + year
                } else {
                    2000 + year
                }
            },
        }
    }
}
