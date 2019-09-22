//! This crate provide methods to read Master Boot record
#![cfg_attr(not(test), no_std)]

use core::convert::TryFrom;
use core::{fmt, fmt::Display};

#[derive(Debug, Copy, Clone)]
#[allow(dead_code)]
pub enum Month {
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
#[derive(Debug, Copy, Clone)]
pub struct Date {
    pub sec: u8,
    pub minutes: u8,
    pub hours: u8,
    pub month: Month,
    pub day_of_month: u8,
    pub year: u32,
}

/// Default boilerplate for Date
impl Default for Date {
    fn default() -> Self {
        Self { sec: 0, minutes: 0, hours: 0, month: Month::January, day_of_month: 1, year: 1664 }
    }
}

impl Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            // "{} {:?} {} {}h{:>02}:{:>02}s",
            // self.day_of_month, self.month, self.year, self.hours, self.minutes, self.sec
            "{}h{:>02}:{:>02}s", self.hours, self.minutes, self.sec
        )
    }
}

/// Gets the day's day number for this year.
/// I tried to make this a const fn, but currently conditionals are not allowed in const fns.
pub fn get_day_number(month: Month, day: usize) -> usize {
    const NUMBER_OF_DAYS: [usize; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

    // assert!(day <= 31); // Can't do that in const fn, don't mess up!
    let month_index = month as usize - 1;

    let mut days = day;
    for index in 0..month_index {
        days += NUMBER_OF_DAYS[index];
    }
    days
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
