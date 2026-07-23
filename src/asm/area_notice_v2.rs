//  SPDX-FileCopyrightText: 2026-2026. Aeolionics, LLC
//
//  SPDX-License-Identifier: Apache-2.0

use super::Linkage;
pub use super::area_notice::{Boundary, Description, Radius, Scale, SectorBoundary};
use crate::types::sixbit::SixBitText;
use crate::types::{Minutes, MonthDayHourMinute};
use deku::{DekuRead, DekuWrite};

/// Structure of an Area Notice message, version 2.
///
/// This modified SubArea position to increase precision
/// but removed the indicator from Rectangle and Sector.
#[derive(DekuRead, DekuWrite)]
pub struct AreaNoticeV2 {
    pub linkage: Linkage,
    pub description: Description,
    pub start: MonthDayHourMinute,
    pub duration: Minutes,
    #[deku(read_all)]
    pub subareas: Vec<SubArea>,
}

/// SubArea definition, version 2.
///
/// This version requires 90 bits.
#[derive(DekuRead, DekuWrite)]
#[deku(id_type = "u8", bits = 3)]
pub enum SubArea {
    #[deku(id = 0)]
    Circle {
        scale: Scale,
        position: Position,
        #[deku(bits = "3")]
        precision: u8,
        #[deku(pad_bits_after = "15")]
        radius: Radius,
    },
    #[deku(id = 1)]
    Rectangle {
        scale: Scale,
        position: Position,
        #[deku(pad_bits_after = "5")]
        boundary: Boundary,
    },
    #[deku(id = 2)]
    Sector { scale: Scale, position: Position, boundary: SectorBoundary },
    #[deku(id = 3)]
    Polyline(#[deku(pad_bits_after = "1")] Shape),
    #[deku(id = 4)]
    Polygon(#[deku(pad_bits_after = "1")] Shape),
    #[deku(id = 5)]
    AssociatedText(#[deku(pad_bits_after = "3")] SixBitText<14>),
}

#[derive(DekuRead, DekuWrite)]
pub struct Position {
    #[deku(bits = "28", endian = "big")]
    pub longitude: i32,
    #[deku(bits = "27", endian = "big")]
    pub latitude: i32,
}

#[derive(DekuRead, DekuWrite)]
pub struct Leg {
    #[deku(bits = "10", endian = "big")]
    pub angle: u16,
    #[deku(bits = "11", endian = "big")]
    pub distance: u16,
}

#[derive(DekuRead, DekuWrite)]
pub struct Shape {
    pub scale: Scale,
    pub points: [Leg; 4],
}
