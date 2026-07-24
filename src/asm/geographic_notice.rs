//  SPDX-FileCopyrightText: 2026-2026. Aeolionics, LLC
//
//  SPDX-License-Identifier: Apache-2.0

use super::area_notice_v2::{Boundary, Description, Leg, Radius, Scale, SectorBoundary};
use super::{Linkage, Version};
use crate::types::sixbit::SixBitText;
use crate::types::{Minutes, MonthDayHourMinute, OutOfRange};
use deku::{DekuRead, DekuWrite};

/// Structure of a Geographic Notice message.
///
/// This added version and action to the main message and added
/// linkage for polyline and polygon sub-aras.
#[derive(DekuRead, DekuWrite)]
pub struct GeographicNotice {
    #[deku(pad_bits_after = "3")]
    pub version: Version,
    pub linkage: Linkage,
    pub description: Description,
    pub start: MonthDayHourMinute,
    pub duration: Minutes,
    #[deku(pad_bits_after = "2")]
    pub action: Action,
    #[deku(read_all)]
    pub subareas: Vec<SubArea>,
}

#[repr(u8)]
#[derive(DekuRead, DekuWrite)]
#[deku(id_type = "u8", bits = 1)]
pub enum Action {
    Advisement = 0,
    Directive = 1,
}

/// SubArea definition, version 2.
///
/// This version requires 96 bits.
#[derive(DekuRead, DekuWrite)]
#[deku(id_type = "u8", bits = 3)]
pub enum SubArea {
    #[deku(id = 0)]
    Circle {
        scale: Scale,
        position: Position,
        radius: Radius,
        #[deku(pad_bits_after = "19")]
        link: Link,
    },
    #[deku(id = 1)]
    Rectangle {
        scale: Scale,
        position: Position,
        #[deku(pad_bits_after = "8")]
        boundary: Boundary,
    },
    #[deku(id = 2)]
    Sector {
        scale: Scale,
        position: Position,
        #[deku(pad_bits_after = "3")]
        boundary: SectorBoundary,
    },
    #[deku(id = 3)]
    Polyline(#[deku(pad_bits_after = "5")] Shape),
    #[deku(id = 4)]
    Polygon(#[deku(pad_bits_after = "5")] Shape),
    #[deku(id = 5)]
    AssociatedText(#[deku(pad_bits_after = "3")] SixBitText<15>),
}

#[derive(DekuRead, DekuWrite)]
pub struct Position {
    #[deku(bits = "28", endian = "big")]
    longitude: i32,
    #[deku(bits = "27", endian = "big")]
    latitude: i32,
    #[deku(bits = "3")]
    precision: u8,
}

impl Position {
    /// Construct from raw components.
    ///
    /// Latitude and longitude are signed values with a resolution of 1/1_000 minute.
    /// SAFETY: caller must ensure values are within range
    pub unsafe fn new_raw(latitude: i32, longitude: i32, precision: u8) -> Self {
        Self { latitude, longitude, precision }
    }

    /// Construct a position from latitude and longitude (degrees).
    ///
    /// Function asserts values are in range.
    pub fn new(latitude: f64, longitude: f64, precision: u8) -> Self {
        assert!(latitude >= -90.0 && latitude <= 90.0);
        assert!(longitude >= -180.0 && longitude <= 180.0);
        let latitude = (latitude * 60.0 * 10_000.0) as i32;
        let longitude = (longitude * 60.0 * 10_000.0) as i32;
        // SAFETY: Bounds have been checked.
        unsafe { Self::new_raw(latitude, longitude, precision) }
    }
}

impl TryFrom<(f64, f64)> for Position {
    type Error = OutOfRange;

    fn try_from((latitude, longitude): (f64, f64)) -> Result<Self, Self::Error> {
        if latitude < -90.0 || latitude > 90.0 || longitude < -180.0 || longitude > 180.0 {
            return Err(OutOfRange);
        }
        Ok(Self::new(latitude, longitude, 4))
    }
}

impl Default for Position {
    fn default() -> Self {
        Self {
            latitude: 181 * 60 * 1_000,
            longitude: 91 * 60 * 1_000,
            precision: 4,
        }
    }
}

#[derive(DekuRead, DekuWrite)]
pub struct Link(#[deku(bits = "2")] pub u8);

#[derive(DekuRead, DekuWrite)]
pub struct Shape {
    pub scale: Scale,
    pub points: [Leg; 4],
    pub link: Link,
}
