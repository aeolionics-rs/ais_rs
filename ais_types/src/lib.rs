//  SPDX-FileCopyrightText: 2026-2026. Aeolionics, LLC
//
//  SPDX-License-Identifier: Apache-2.0

use chrono::{DateTime, Datelike, Timelike, Utc};
use deku::{DekuRead, DekuWrite};
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(DekuRead, DekuWrite)]
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

#[derive(DekuRead, DekuWrite)]
pub struct Minutes(#[deku(bits = 18, endian = "big")] pub u32);

#[derive(Debug)]
pub struct OutOfRange;

impl Display for OutOfRange {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("out of range")
    }
}

impl Error for OutOfRange {}
