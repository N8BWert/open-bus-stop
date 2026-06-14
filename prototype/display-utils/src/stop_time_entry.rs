//!
//! Stop Time Data is packed in this format to make it easy
//! to parse and use in the display code.
//! 

use zerocopy::{FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned};
use zerocopy::byteorder::little_endian::U16;

/// Iterate through a sorted list of stop time entries to find the next entry that is greater than or equal to the given time.
pub fn find_next_stop_time_entry<'a>(data: &'a [u8], hour: u8, minute: u8, second: u8) -> &'a StopTimeEntry {
    for chunk in data.chunks_exact(StopTimeEntry::SIZE) {
        if let Ok(entry) = StopTimeEntry::ref_from_bytes(chunk) {
            if entry.greater_than_equal(hour, minute, second) {
                return entry;
            }
        }
    }
    StopTimeEntry::ref_from_bytes(&data[0..StopTimeEntry::SIZE]).unwrap()
}

#[derive(FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(packed)]
/// An entry in the stop time data
pub struct StopTimeEntry {
    pub route: U16,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
}

impl StopTimeEntry {
    pub const SIZE: usize = core::mem::size_of::<Self>();

    pub fn greater_than_equal(&self, hour: u8, minute: u8, second: u8) -> bool {
        (self.hour, self.minute, self.second) >= (hour, minute, second)
    }

    pub fn minutes_until_stop(&self, hour: u8, minute: u8, second: u8) -> u16 {
        let stop_hour = if self.hour >= 24 {
            self.hour - 24
        } else {
            self.hour
        };

        let stop_time = (stop_hour as u32) * 3600 + (self.minute as u32) * 60 + (self.second as u32);
        let now = (hour as u32) * 3600 + (minute as u32) * 60 + (second as u32);
        if stop_time >= now {
            ((stop_time - now) / 60) as u16
        } else {
            let max_time = 24 * 60 * 60;
            ((max_time - now + stop_time) / 60) as u16
        }
    }

    pub fn passed(&self, hour: u8, minute: u8, second: u8) -> bool {
        let stop_hour = if self.hour >= 24 {
            self.hour - 24
        } else {
            self.hour
        };
        let stop_time = (stop_hour as u32) * 3600 + (self.minute as u32) * 60 + (self.second as u32);
        let target_time = (hour as u32) * 3600 + (minute as u32) * 60 + (second as u32);
        stop_time < target_time
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_stop_time_entry() {
        let data = [2, 0, 5, 8, 51];
        let entry = StopTimeEntry::read_from_bytes(&data).unwrap();
        assert_eq!(entry.route, 2);
        assert_eq!(entry.hour, 5);
        assert_eq!(entry.minute, 8);
        assert_eq!(entry.second, 51);
    }

    #[test]
    fn test_greater_than_equal() {
        let stop_time = StopTimeEntry {
            route: 1.into(),
            hour: 5,
            minute: 8,
            second: 51,
        };
        assert!(stop_time.greater_than_equal(4, 7, 50));
        assert!(!stop_time.greater_than_equal(6, 9, 52));
    }

    #[test]
    fn test_minutes_until() {
        let stop_time = StopTimeEntry {
            route: 1.into(),
            hour: 5,
            minute: 8,
            second: 51,
        };
        assert_eq!(stop_time.minutes_until_stop(5, 6, 51), 2);
        assert_eq!(stop_time.minutes_until_stop(5, 10, 51), 1438);
    }

    #[test]
    fn test_minutes_until_next_day() {
        let stop_time = StopTimeEntry {
            route: 1.into(),
            hour: 24,
            minute: 10,
            second: 0,
        };
        assert_eq!(stop_time.minutes_until_stop(0, 5, 0), 5);
    }

    #[test]
    fn test_minutes_until_overflow() {
        let stop_time = StopTimeEntry {
            route: 1.into(),
            hour: 5,
            minute: 50,
            second: 0,
        };
        assert_eq!(stop_time.minutes_until_stop(23, 30, 0), 6 * 60 + 20);
    }

    #[test]
    fn test_next_stop_time_entry() {
        let data = [
            1, 0, 5, 8, 51, // 05:08:51
            2, 0, 6, 0, 0,   // 06:00:00
            3, 0, 7, 30, 0,  // 07:30:00
        ];
        let entry = find_next_stop_time_entry(&data, 5, 9, 0);
        assert_eq!(entry.route, 2);
        assert_eq!(entry.hour, 6);
        assert_eq!(entry.minute, 0);
        assert_eq!(entry.second, 0);

        let entry = find_next_stop_time_entry(&data, 6, 0, 0);
        assert_eq!(entry.route, 2);
        assert_eq!(entry.hour, 6);
        assert_eq!(entry.minute, 0);
        assert_eq!(entry.second, 0);

        let entry = find_next_stop_time_entry(&data, 7, 30, 1);
        assert_eq!(entry.route, 1);
        assert_eq!(entry.hour, 5);
        assert_eq!(entry.minute, 8);
        assert_eq!(entry.second, 51);
    }
}