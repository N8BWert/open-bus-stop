//!
//! Special Service Data is packed in this format to make it easy
//! to parse and use the special service data in the display code.
//!

use zerocopy::{TryFromBytes, IntoBytes, KnownLayout, Immutable, Unaligned};
use zerocopy::byteorder::little_endian::U16;

/// Find the special servie entry for the given date in the provided data.
pub fn find_special_service_entry<'a>(data: &'a [u8], year: u16, month: u8, day: u8) -> Option<&'a SpecialServiceEntry> {
    for chunk in data.chunks_exact(SpecialServiceEntry::SIZE) {
        if let Ok(entry) = SpecialServiceEntry::try_ref_from_bytes(chunk) {
            if entry.is_active_on(year, month, day) {
                return Some(entry);
            }
        }
    }
    None
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum SpecialServiceType {
    Weekday = 0,
    Saturday = 1,
    Sunday = 2,
    Unknown(u8),
}

#[derive(TryFromBytes, IntoBytes, KnownLayout, Immutable, Unaligned, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(packed)]
/// An entry in the special service data.
pub struct SpecialServiceEntry {
    pub year: U16,
    pub month: u8,
    pub day: u8,
    service_type: u8,
}

impl SpecialServiceEntry {
    pub const SIZE: usize = core::mem::size_of::<Self>();

    /// Checks if the entry is active on the given date.
    pub fn is_active_on(&self, year: u16, month: u8, day: u8) -> bool {
        self.year.get() == year && self.month == month && self.day == day
    }

    /// Get the service type of the entry
    pub fn service_type(&self) -> SpecialServiceType {
        match self.service_type {
            0 => SpecialServiceType::Weekday,
            1 => SpecialServiceType::Saturday,
            2 => SpecialServiceType::Sunday,
            other => SpecialServiceType::Unknown(other),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_special_service_entry() {
        let data = [0xEA, 0x07, 0x05, 0x19, 0x01]; // 2026-05-25
        let entry = SpecialServiceEntry::try_read_from_bytes(&data).unwrap();
        assert_eq!(entry.year.get(), 2026);
        assert_eq!(entry.month, 5);
        assert_eq!(entry.day, 25);
        assert_eq!(entry.service_type(), SpecialServiceType::Saturday);
        assert!(entry.is_active_on(2026, 5, 25));
        assert!(!entry.is_active_on(2026, 5, 26));
    }

    #[test]
    fn test_special_service_entry_size() {
        assert_eq!(SpecialServiceEntry::SIZE, 5);
    }

    #[test]
    fn test_find_special_service_entry() {
        let data = [
            0xEA, 0x07, 0x05, 0x19, 0x01, // 2026-05-25
            0xEB, 0x07, 0x06, 0x20, 0x02, // 2027-06-32 (invalid date but for testing)
        ];
        let entry = find_special_service_entry(&data, 2026, 5, 25).unwrap();
        assert_eq!(entry.year.get(), 2026);
        assert_eq!(entry.month, 5);
        assert_eq!(entry.day, 25);
        assert_eq!(entry.service_type(), SpecialServiceType::Saturday);

        let entry = find_special_service_entry(&data, 2027, 6, 32).unwrap();
        assert_eq!(entry.year.get(), 2027);
        assert_eq!(entry.month, 6);
        assert_eq!(entry.day, 32);
        assert_eq!(entry.service_type(), SpecialServiceType::Sunday);

        let entry = find_special_service_entry(&data, 2028, 1, 1);
        assert!(entry.is_none());
    }
}
