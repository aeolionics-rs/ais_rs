//  SPDX-FileCopyrightText: 2026-2026. Aeolionics, LLC
//
//  SPDX-License-Identifier: Apache-2.0

use super::area_notice_v2::{Boundary, Description, Leg, Radius, Scale, SectorBoundary};
use super::{Linkage, Version};
use ais_sixbit::SixBitText;
use ais_types::{LatLon, Minutes, MonthDayHourMinute, OutOfRange};
use core::fmt::{self, Display, Formatter};
use deku::{DekuRead, DekuWrite};

/// Structure of the EU inland Geographic Notice message (DAC 200, FI 42; addressed in message 6,
/// in force since 18 October 2021, <https://www.e-navigation.nl/content/geographic-notice-1>;
/// broadcast in message 8, a proposal, <https://www.e-navigation.nl/content/geographic-notice-0>;
/// both pages carry the layout, there is no separate document).
///
/// This added a three-bit version indicator (0 today) and an action to the Area Notice, widened
/// each sub-area to 96 bits, and added link bits to the point, polyline and polygon sub-areas
/// saying which shape a point continues. The USCG's message of the same name differs in its
/// version field and has no link bits: see [`uscg_geographic_notice`](super::uscg_geographic_notice).
#[derive(Debug, DekuRead, DekuWrite)]
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
#[derive(Debug, DekuRead, DekuWrite)]
#[deku(id_type = "u8", bits = 1)]
pub enum Action {
    Advisement = 0,
    Directive = 1,
}

/// SubArea definition of the EU Geographic Notice, Tables 3 to 8 of its page.
///
/// This version requires 96 bits.
#[derive(Debug, DekuRead, DekuWrite)]
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

#[derive(Debug, DekuRead, DekuWrite)]
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
    /// Latitude and longitude are signed values with a resolution of 1/10_000 minute.
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

impl TryFrom<&Position> for (f64, f64) {
    type Error = OutOfRange;

    /// The latitude and longitude in degrees.
    ///
    /// A position that is not available, latitude 91 or longitude 181 degrees, is out of range.
    fn try_from(position: &Position) -> Result<Self, Self::Error> {
        if position.latitude.abs() > 90 * 60 * 10_000 || position.longitude.abs() > 180 * 60 * 10_000 {
            return Err(OutOfRange);
        }
        Ok(((position.latitude as f64) / 60.0 / 10_000.0, (position.longitude as f64) / 60.0 / 10_000.0))
    }
}

impl Default for Position {
    /// A position that is not available: latitude 91 and longitude 181 degrees.
    fn default() -> Self {
        Self { latitude: 91 * 60 * 10_000, longitude: 181 * 60 * 10_000, precision: 4 }
    }
}

/// What a point, polyline or polygon sub-area continues: 0 a single point, polyline or polygon, or
/// the end point of one (the default); 1 the start or a further point of a polyline; 2 the start
/// or a further point of a polygon; 3 unused.
#[derive(Debug, DekuRead, DekuWrite)]
pub struct Link(#[deku(bits = "2")] pub u8);

#[derive(Debug, DekuRead, DekuWrite)]
pub struct Shape {
    pub scale: Scale,
    pub points: [Leg; 4],
    pub link: Link,
}

impl Display for GeographicNotice {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let when = (&self.start, &self.duration);
        let action: &dyn Display = &self.action;
        crate::notice(f, "EU Geographic Notice", &self.linkage, &self.description, Some(action), when, &self.subareas)
    }
}

impl Display for Action {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Action::Advisement => "advisement",
            Action::Directive => "directive",
        })
    }
}

impl crate::Piece for SubArea {
    fn shape(&self) -> Option<&'static str> {
        match self {
            SubArea::Circle { radius, .. } if radius.0 == 0 => Some("point"),
            SubArea::Circle { .. } => Some("circle"),
            SubArea::Rectangle { boundary, .. } if boundary.east == 0 && boundary.north == 0 => Some("point"),
            SubArea::Rectangle { .. } => Some("rectangle"),
            SubArea::Sector { .. } => Some("sector"),
            SubArea::Polyline(_) => Some("polyline"),
            SubArea::Polygon(_) => Some("polygon"),
            SubArea::AssociatedText(_) => None,
        }
    }

    fn text(&self) -> Option<String> {
        match self {
            SubArea::AssociatedText(text) => Some(text.to_string()),
            _ => None,
        }
    }
}

impl Display for SubArea {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            SubArea::Circle { scale, position, radius, .. } => {
                write!(f, "circle {} m at {position}", scale.metres(u32::from(radius.0)))
            }
            SubArea::Rectangle { scale, position, boundary } => {
                let (east, north) = (scale.metres(u32::from(boundary.east)), scale.metres(u32::from(boundary.north)));
                write!(f, "rectangle {east} m × {north} m at {position}, {}°", boundary.orientation)
            }
            SubArea::Sector { scale, position, boundary } => {
                let radius = scale.metres(u32::from(boundary.radius.0));
                write!(f, "sector {radius} m at {position}, {}° to {}°", boundary.left, boundary.right)
            }
            SubArea::Polyline(shape) => write!(f, "polyline, {shape}"),
            SubArea::Polygon(shape) => write!(f, "polygon, {shape}"),
            SubArea::AssociatedText(text) => write!(f, "{:?}", crate::text(text)),
        }
    }
}

impl Display for Position {
    /// In degrees to the ten-thousandth of a minute it carries, 6 places.
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match <(f64, f64)>::try_from(self) {
            Ok((latitude, longitude)) => LatLon { latitude, longitude, decimals: 6 }.fmt(f),
            Err(_) => f.write_str("position n/a"),
        }
    }
}

impl Display for Link {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(match self.0 {
            0 => "single or end",
            1 => "polyline continues",
            2 => "polygon continues",
            _ => "link 3",
        })
    }
}

impl Display for Shape {
    /// How many legs are used, at its scale.
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let legs = self.points.iter().filter(|leg| leg.angle < 720).count();
        write!(f, "{legs} legs, {}", self.scale)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use deku::{DekuContainerRead, DekuContainerWrite};

    #[test]
    fn position_default() {
        let bits = Position::default().to_bits().unwrap();
        let (_, position) = Position::from_bytes((bits.as_raw_slice(), 0)).unwrap();
        assert_eq!((position.latitude, position.longitude), (91 * 60 * 10_000, 181 * 60 * 10_000));
    }

    #[test]
    fn position_scale() {
        // The same resolution, 1/10_000 minute, is used by new() and the default.
        let position = Position::new(1.0, -1.0, 4);
        assert_eq!((position.latitude, position.longitude), (60 * 10_000, -60 * 10_000));
    }
}
