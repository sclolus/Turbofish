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
            0x10 => Ok(October),
            0x11 => Ok(November),
            0x12 => Ok(December),
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

/// Convert a Date into a universal unix timestamp expressed in second
impl From<Date> for u32 {
    fn from(date: Date) -> Self {
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
        tm_sec + tm_min * 60 + tm_hour * 3600 + tm_yday * 86400 + (tm_year - 70) * 31536000
        + ((tm_year - 69) / 4) * 86400
        - ((tm_year - 1) / 100) * 86400
        + ((tm_year + 299) / 400) * 86400 // How the fuck, does RTC count leapdays.
    }
}

/// Default boilerplate for Date
impl Default for Date {
    fn default() -> Self {
        Self {
            sec: 0,
            minutes: 0,
            hours: 0,
            month: Month::January,
            day_of_month: 1,
            year: 1664,
        }
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
