//  SPDX-FileCopyrightText: 2026-2026. Aeolionics, LLC
//
//  SPDX-License-Identifier: Apache-2.0

use super::area_notice_v2::{Boundary, Description, Leg, Position, Radius, Scale, SectorBoundary};
use super::{Linkage, Version};
use crate::types::sixbit::SixBitText;
use crate::types::{Minutes, MonthDayHourMinute};
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
pub struct Link(#[deku(bits = "2")] pub u8);

#[derive(DekuRead, DekuWrite)]
pub struct Shape {
    pub scale: Scale,
    pub points: [Leg; 4],
    pub link: Link,
}
