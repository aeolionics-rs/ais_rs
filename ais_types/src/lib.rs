//  SPDX-FileCopyrightText: 2026-2026. Aeolionics, LLC
//
//  SPDX-License-Identifier: Apache-2.0

use chrono::{DateTime, Datelike, Timelike, Utc};
use deku::bitvec::{BitSlice, BitVec, Msb0};
use deku::no_std_io::{Read, Seek, Write};
use deku::prelude::{DekuError, DekuRead, DekuReader, DekuWrite, DekuWriter, Reader, Writer};
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::ops::Deref;

#[derive(Debug, DekuRead, DekuWrite)]
pub struct MonthDayHourMinute {
    #[deku(bits = 4)]
    pub month: u8,
    #[deku(bits = 5)]
    pub day: u8,
    #[deku(bits = 5)]
    pub hour: u8,
    #[deku(bits = 6)]
    pub minute: u8,
}

impl From<DateTime<Utc>> for MonthDayHourMinute {
    fn from(value: DateTime<Utc>) -> Self {
        Self {
            month: value.month() as u8,
            day: value.day() as u8,
            hour: value.hour() as u8,
            minute: value.minute() as u8,
        }
    }
}

impl Default for MonthDayHourMinute {
    fn default() -> Self {
        Self { month: 0, day: 0, hour: 24, minute: 60 }
    }
}

impl Display for MonthDayHourMinute {
    /// The month, day and time, `10-25 12:00 UTC`, or `n/a` when any of them is not available.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if !(1..=12).contains(&self.month) || !(1..=31).contains(&self.day) || self.hour > 23 || self.minute > 59 {
            return f.write_str("n/a");
        }
        write!(f, "{:02}-{:02} {:02}:{:02} UTC", self.month, self.day, self.hour, self.minute)
    }
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct Minutes(#[deku(bits = 18, endian = "big")] pub u32);

impl Display for Minutes {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} min", self.0)
    }
}

/// A latitude and longitude in degrees, written to so many decimal places with their
/// hemispheres, `51.490000°N 0.120000°W`: for the positions of the messages to display.
pub struct LatLon {
    pub latitude: f64,
    pub longitude: f64,
    /// Decimal places, as many as the position's resolution has.
    pub decimals: usize,
}

impl Display for LatLon {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let decimals = self.decimals;
        let north = if self.latitude < 0.0 { 'S' } else { 'N' };
        let east = if self.longitude < 0.0 { 'W' } else { 'E' };
        write!(f, "{:.decimals$}°{north} {:.decimals$}°{east}", self.latitude.abs(), self.longitude.abs())
    }
}

#[derive(Debug)]
pub struct OutOfRange;

impl Display for OutOfRange {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("out of range")
    }
}

impl Error for OutOfRange {}

/// Binary data that extends to the end of a message.
///
/// The data is not necessarily a whole number of bytes, so it is held as bits.
#[derive(Clone, Default, PartialEq, Eq)]
pub struct BinaryData(pub BitVec<u8, Msb0>);

impl<'a> DekuReader<'a, ()> for BinaryData {
    /// Read all the remaining bits.
    fn from_reader_with_ctx<R: Read + Seek>(reader: &mut Reader<R>, _: ()) -> Result<Self, DekuError> {
        let mut bits = BitVec::new();
        // Reading past the end loses any buffered bits, so only read the bits that end() buffers.
        while !reader.end() {
            let buffered = reader.rest().len();
            if let Some(read) = reader.read_bits(buffered, deku::ctx::Order::Msb0)? {
                bits.extend_from_bitslice(&read);
            }
        }
        Ok(Self(bits))
    }
}

impl DekuWriter<()> for BinaryData {
    fn to_writer<W: Write + Seek>(&self, writer: &mut Writer<W>, _: ()) -> Result<(), DekuError> {
        writer.write_bits(&self.0)
    }
}

impl Deref for BinaryData {
    type Target = BitSlice<u8, Msb0>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<BitVec<u8, Msb0>> for BinaryData {
    fn from(bits: BitVec<u8, Msb0>) -> Self {
        Self(bits)
    }
}

impl From<Vec<u8>> for BinaryData {
    fn from(bytes: Vec<u8>) -> Self {
        Self(BitVec::from_vec(bytes))
    }
}

impl From<&[u8]> for BinaryData {
    fn from(bytes: &[u8]) -> Self {
        Self(BitVec::from_slice(bytes))
    }
}

impl Display for BinaryData {
    /// How many bits: the data itself is for [`Debug`].
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} bits", self.0.len())
    }
}

impl Debug for BinaryData {
    /// Debug output as a quoted string of bits.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let bits: String = self.0.iter().map(|bit| if *bit { '1' } else { '0' }).collect();
        f.debug_tuple("BinaryData").field(&bits).finish()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use deku::bitvec::bitvec;
    use deku::{DekuContainerRead, DekuContainerWrite};

    #[derive(Debug, DekuRead, DekuWrite)]
    struct Message {
        #[deku(bits = 3)]
        header: u8,
        data: BinaryData,
    }

    #[test]
    fn binary_data() {
        let bytes = [0b1011_1001, 0b0101_0110];

        // 3 header bits followed by 13 data bits.
        let (_, message) = Message::from_bytes((&bytes, 0)).unwrap();
        assert_eq!(message.header, 0b101);
        assert_eq!(message.data.0, bitvec![u8, Msb0; 1, 1, 0, 0, 1, 0, 1, 0, 1, 0, 1, 1, 0]);

        // Starting 1 bit in, the data is 12 bits and is written back exactly.
        let (_, message) = Message::from_bytes((&bytes, 1)).unwrap();
        assert_eq!(message.header, 0b011);
        assert_eq!(message.data.0, bitvec![u8, Msb0; 1, 0, 0, 1, 0, 1, 0, 1, 0, 1, 1, 0]);
        assert_eq!(message.to_bits().unwrap(), BitSlice::<u8, Msb0>::from_slice(&bytes)[1..]);
    }

    #[test]
    fn display() {
        assert_eq!(BinaryData::from(vec![0xa5, 0x5a]).to_string(), "16 bits");
        let start = MonthDayHourMinute { month: 10, day: 25, hour: 9, minute: 5 };
        assert_eq!(start.to_string(), "10-25 09:05 UTC");
        assert_eq!(MonthDayHourMinute::default().to_string(), "n/a");
        assert_eq!(Minutes(120).to_string(), "120 min");
        let position = LatLon { latitude: 51.49, longitude: -0.12, decimals: 6 };
        assert_eq!(position.to_string(), "51.490000°N 0.120000°W");
        let position = LatLon { latitude: -33.5, longitude: 151.25, decimals: 3 };
        assert_eq!(position.to_string(), "33.500°S 151.250°E");
    }

    #[test]
    fn binary_data_debug() {
        assert_eq!(format!("{:?}", BinaryData::from(vec![0xa5])), "BinaryData(\"10100101\")");
    }
}
