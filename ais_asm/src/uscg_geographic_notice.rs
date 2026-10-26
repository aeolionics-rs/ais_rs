//  SPDX-FileCopyrightText: 2026-2026. Aeolionics, LLC
//
//  SPDX-License-Identifier: Apache-2.0

//! The USCG Geographic Notice. Each message defines its own sub-area types, as its specification
//! lays them out; only the notice description, the code table the Geographic Notices share, comes
//! from the EU message's module.

use super::geographic_notice::Description;
use super::{Linkage, MessageVersion};
use ais_sixbit::SixBitText;
use ais_types::{LatLon, Minutes, MonthDayHourMinute, OutOfRange};
use core::fmt::{self, Display, Formatter};
use deku::{DekuRead, DekuWrite};

/// Structure of the USCG Geographic Notice message (DAC 367, FI 22, broadcast in message 8 or
/// addressed in message 6), release 2 of 23 March 2015:
/// <https://www.e-navigation.nl/content/geographic-notice>, specification
/// <https://www.e-navigation.nl/sites/default/files/asm_files/GN%20Release%202-23MAR15.pdf>.
///
/// The successor to the USCG Area Notice (DAC 366): a six-bit message version (set to 2) and an
/// action added, each sub-area widened to 96 bits. The EU inland message of the same name,
/// [`geographic_notice`](super::geographic_notice), has a three-bit version and link bits where
/// this one has spare bits.
#[derive(Debug, DekuRead, DekuWrite)]
pub struct GeographicNotice {
    pub version: MessageVersion,
    pub linkage: Linkage,
    pub description: Description,
    pub start: MonthDayHourMinute,
    pub duration: Minutes,
    #[deku(pad_bits_after = "2")]
    pub action: Action,
    #[deku(read_all)]
    pub subareas: Vec<SubArea>,
}

/// Whether the notice advises or directs.
#[repr(u8)]
#[derive(Debug, DekuRead, DekuWrite)]
#[deku(id_type = "u8", bits = 1)]
pub enum Action {
    Advisement = 0,
    Directive = 1,
}

/// SubArea definition of the USCG Geographic Notice, Tables 5 to 10 of its specification.
///
/// Each sub-area is 96 bits.
#[derive(Debug, DekuRead, DekuWrite)]
#[deku(id_type = "u8", bits = 3)]
pub enum SubArea {
    /// Table 5: a circle, or a point when the radius is nought, which a polygon's legs follow.
    #[deku(id = 0)]
    Circle {
        scale: Scale,
        position: Position,
        #[deku(pad_bits_after = "21")]
        radius: Radius,
    },
    /// Table 6: a rectangle from its south-west corner, or a point when both dimensions are
    /// nought, which a polyline's legs follow.
    #[deku(id = 1)]
    Rectangle {
        scale: Scale,
        position: Position,
        #[deku(pad_bits_after = "8")]
        boundary: Boundary,
    },
    /// Table 7: a sector of a circle, from its left boundary clockwise to its right.
    #[deku(id = 2)]
    Sector {
        scale: Scale,
        position: Position,
        #[deku(pad_bits_after = "3")]
        boundary: SectorBoundary,
    },
    /// Table 8: up to four legs of a line from the point before.
    #[deku(id = 3)]
    Polyline(#[deku(pad_bits_after = "7")] Shape),
    /// Table 9: up to four legs of a polygon from the point before, closed back to it.
    #[deku(id = 4)]
    Polygon(#[deku(pad_bits_after = "7")] Shape),
    /// Table 10: fifteen six-bit characters, the pieces joined in order.
    #[deku(id = 5)]
    AssociatedText(#[deku(pad_bits_after = "3")] SixBitText<15>),
}

/// A sub-area's scale: its dimensions in metres are multiplied by ten to this power, 0 to 3.
#[derive(Debug, DekuRead, DekuWrite)]
pub struct Scale(#[deku(bits = "2")] pub u8);

/// A radius in metres, 0 a point, to 4,095, times the scale.
#[derive(Debug, DekuRead, DekuWrite)]
pub struct Radius(#[deku(bits = "12", endian = "big")] pub u16);

/// A rectangle's dimensions east and north of its corner in metres times the scale, 0 to 255,
/// and its rotation clockwise about the corner in degrees, 0 to 359.
#[derive(Debug, DekuRead, DekuWrite)]
pub struct Boundary {
    #[deku(bits = "8")]
    pub east: u8,
    #[deku(bits = "8")]
    pub north: u8,
    #[deku(bits = "9", endian = "big")]
    pub orientation: u16,
}

/// A sector's radius and the bearings of its left and right boundaries from true north, 0 to 359.
#[derive(Debug, DekuRead, DekuWrite)]
pub struct SectorBoundary {
    pub radius: Radius,
    #[deku(bits = "9", endian = "big")]
    pub left: u16,
    #[deku(bits = "9", endian = "big")]
    pub right: u16,
}

/// A position to a ten-thousandth of a minute, with the precision its sender means it to.
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
    /// Latitude and longitude are signed values with a resolution of 1/10_000 minute; the
    /// precision is the number of decimal places meant, 0 to 4.
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

/// A leg of a polyline or polygon, from the previous point to the next.
#[derive(Debug, DekuRead, DekuWrite)]
pub struct Leg {
    /// The true bearing to the next point in half degrees, 0 to 719, or 720 if the leg is not used.
    #[deku(bits = "10", endian = "big")]
    pub angle: u16,
    /// The distance to the next point in metres times the scale, 1 to 2,047, or 0 if the leg is
    /// not used.
    #[deku(bits = "11", endian = "big")]
    pub distance: u16,
}

/// The legs of a polyline or polygon sub-area, at one scale.
#[derive(Debug, DekuRead, DekuWrite)]
pub struct Shape {
    pub scale: Scale,
    pub points: [Leg; 4],
}

impl Display for GeographicNotice {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let when = (&self.start, &self.duration);
        let action: &dyn Display = &self.action;
        // A test message says so; the version in force is the specification's business.
        let name = if self.version.0 == 0 { "Geographic Notice (test)" } else { "Geographic Notice" };
        crate::notice(
            f,
            name,
            &self.linkage,
            (&self.description, self.description == Description::Cancellation),
            Some(action),
            when,
            &self.subareas,
        )
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

impl Display for Scale {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "scale {}", self.0)
    }
}

impl Scale {
    /// Metres for so many of the scale's units.
    pub fn metres(&self, units: u32) -> u32 {
        units * 10u32.pow(u32::from(self.0))
    }
}

impl Display for Radius {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "radius {}", self.0)
    }
}

impl Display for Boundary {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} east × {} north, {}°", self.east, self.north, self.orientation)
    }
}

impl Display for SectorBoundary {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}° to {}°", self.radius, self.left, self.right)
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

impl Display for Leg {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.angle {
            720.. => f.write_str("unused"),
            angle => write!(f, "{:.1}° {}", f64::from(angle) / 2.0, self.distance),
        }
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

    /// The head is 64 bits and each sub-area 96, as the specification's tables total them, and
    /// the version is the first six bits.
    #[test]
    fn sizes() {
        let notice = GeographicNotice {
            version: MessageVersion(2),
            linkage: Linkage(1),
            description: Description::MarineMammalHabitat,
            start: MonthDayHourMinute { month: 10, day: 25, hour: 12, minute: 0 },
            duration: Minutes(60),
            action: Action::Directive,
            subareas: vec![
                SubArea::Circle { scale: Scale(0), position: Position::new(51.5, -0.1, 4), radius: Radius(500) },
                SubArea::Rectangle {
                    scale: Scale(0),
                    position: Position::new(51.5, -0.1, 4),
                    boundary: Boundary { east: 10, north: 20, orientation: 30 },
                },
                SubArea::Sector {
                    scale: Scale(0),
                    position: Position::new(51.5, -0.1, 4),
                    boundary: SectorBoundary { radius: Radius(100), left: 10, right: 20 },
                },
                SubArea::Polygon(Shape {
                    scale: Scale(0),
                    points: std::array::from_fn(|_| Leg { angle: 720, distance: 0 }),
                }),
                SubArea::AssociatedText("FIFTEEN LETTERS".parse().unwrap()),
            ],
        };
        assert_eq!(
            notice.to_string(),
            "Geographic Notice 1: marine mammal habitat, directive, 10-25 12:00 UTC for 60 min; \
             circle, rectangle, sector, polygon; \"FIFTEEN LETTERS\""
        );
        assert_eq!(notice.subareas[0].to_string(), "circle 500 m at 51.500000°N 0.100000°W");
        assert_eq!(notice.subareas[2].to_string(), "sector 100 m at 51.500000°N 0.100000°W, 10° to 20°");
        let bits = notice.to_bits().unwrap();
        assert_eq!(bits.len(), 64 + 5 * 96);
        assert_eq!(bits.as_raw_slice()[0] >> 2, 2, "the version is the first six bits");
        let (_, again) = GeographicNotice::from_bytes((bits.as_raw_slice(), 0)).unwrap();
        assert_eq!(again.subareas.len(), 5);
        assert!(matches!(again.action, Action::Directive));
    }

    #[test]
    fn position_default() {
        let bits = Position::default().to_bits().unwrap();
        let (_, position) = Position::from_bytes((bits.as_raw_slice(), 0)).unwrap();
        assert_eq!((position.latitude, position.longitude), (91 * 60 * 10_000, 181 * 60 * 10_000));
    }
}
