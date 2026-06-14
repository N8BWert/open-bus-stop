//!
//! Library for the informal bus stop display
//!

#![no_std]

use display_utils::{
    special_service_entry::{find_special_service_entry, SpecialServiceType},
    stop_time_entry::{find_next_stop_time_entry, StopTimeEntry},
};
use embassy_rp::rtc::{DateTime, DayOfWeek};

/// The special service data for the bus stop
pub const SPECIAL_SERVICE_DATA: &'static [u8] =
    include_bytes!("./stop_data/special_service_data.bin");
/// The weekday stop times for the bus stop
pub const WEEKDAY_STOP_TIMES: &'static [u8] = include_bytes!("./stop_data/weekday_stop_times.bin");
/// The Saturday stop times for the bus stop
pub const SATURDAY_STOP_TIMES: &'static [u8] =
    include_bytes!("./stop_data/saturday_stop_times.bin");
/// The Sunday stop times for the bus stop
pub const SUNDAY_STOP_TIMES: &'static [u8] = include_bytes!("./stop_data/sunday_stop_times.bin");

/// Parse the current day of the week from the build environment variable
const fn u8_to_day_of_week(n: u8) -> DayOfWeek {
    match n {
        1 => DayOfWeek::Monday,
        2 => DayOfWeek::Tuesday,
        3 => DayOfWeek::Wednesday,
        4 => DayOfWeek::Thursday,
        5 => DayOfWeek::Friday,
        6 => DayOfWeek::Saturday,
        7 => DayOfWeek::Sunday,
        _ => DayOfWeek::Monday,
    }
}

/// The Start date and time for initializing the RTC.
pub const START: DateTime = DateTime {
    year: const_str::parse!(env!("BUILD_YEAR"), u16),
    month: const_str::parse!(env!("BUILD_MONTH"), u8),
    day: const_str::parse!(env!("BUILD_DAY"), u8),
    day_of_week: u8_to_day_of_week(const_str::parse!(env!("BUILD_DAY_OF_WEEK"), u8)),
    hour: const_str::parse!(env!("BUILD_HOUR"), u8),
    minute: const_str::parse!(env!("BUILD_MINUTE"), u8),
    second: const_str::parse!(env!("BUILD_SECOND"), u8),
};

pub fn get_next_stop_time(now: &DateTime) -> &'static StopTimeEntry {
    if let Some(special_service_entry) =
        find_special_service_entry(SPECIAL_SERVICE_DATA, now.year, now.month, now.day)
    {
        match special_service_entry.service_type() {
            SpecialServiceType::Weekday => {
                find_next_stop_time_entry(WEEKDAY_STOP_TIMES, now.hour, now.minute, now.second)
            }
            SpecialServiceType::Saturday => {
                find_next_stop_time_entry(SATURDAY_STOP_TIMES, now.hour, now.minute, now.second)
            }
            SpecialServiceType::Sunday => {
                find_next_stop_time_entry(SUNDAY_STOP_TIMES, now.hour, now.minute, now.second)
            }
            SpecialServiceType::Unknown(u8) => {
                defmt::warn!("Unknown special service type: {}", u8);
                find_next_stop_time_entry(WEEKDAY_STOP_TIMES, now.hour, now.minute, now.second)
            }
        }
    } else {
        match now.day_of_week {
            DayOfWeek::Monday
            | DayOfWeek::Tuesday
            | DayOfWeek::Wednesday
            | DayOfWeek::Thursday
            | DayOfWeek::Friday => {
                find_next_stop_time_entry(WEEKDAY_STOP_TIMES, now.hour, now.minute, now.second)
            }
            DayOfWeek::Saturday => {
                find_next_stop_time_entry(SATURDAY_STOP_TIMES, now.hour, now.minute, now.second)
            }
            DayOfWeek::Sunday => {
                find_next_stop_time_entry(SUNDAY_STOP_TIMES, now.hour, now.minute, now.second)
            }
        }
    }
}
