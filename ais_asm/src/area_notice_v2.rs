//  SPDX-FileCopyrightText: 2026-2026. Aeolionics, LLC
//
//  SPDX-License-Identifier: Apache-2.0

use super::Linkage;
pub use super::area_notice::{Boundary, Description, Radius, Scale, SectorBoundary};
use ais_sixbit::SixBitText;
use ais_types::{LatLon, Minutes, MonthDayHourMinute, OutOfRange};
use core::fmt::{self, Display, Formatter};
use deku::{DekuRead, DekuWrite};

/// Structure of an Area Notice message, version 2: the USCG's Area Notice (DAC 366, FI 22),
/// version 2.3, deprecated in favour of the Geographic Notice:
/// <https://www.e-navigation.nl/content/area-notice-1>, specification
/// <https://www.e-navigation.nl/sites/default/files/asm_files/AN%20Version%2023.pdf>.
///
/// This modified `SubArea` position to increase precision
/// but removed the indicator from Rectangle and Sector.
#[derive(Debug, DekuRead, DekuWrite)]
pub struct AreaNoticeV2 {
    pub linkage: Linkage,
    pub description: Description,
    pub start: MonthDayHourMinute,
    pub duration: Minutes,
    #[deku(read_all)]
    pub subareas: Vec<SubArea>,
}

/// `SubArea` definition, version 2.
///
/// This version requires 90 bits.
#[derive(Debug, DekuRead, DekuWrite)]
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

#[derive(Debug, DekuRead, DekuWrite)]
pub struct Position {
    #[deku(bits = "28", endian = "big")]
    longitude: i32,
    #[deku(bits = "27", endian = "big")]
    latitude: i32,
}

impl Position {
    /// Construct from raw components.
    ///
    /// Latitude and longitude are signed values with a resolution of `1/10_000` minute.
    ///
    /// # Safety
    ///
    /// The caller must ensure the values are within range.
    #[must_use]
    pub unsafe fn new_raw(latitude: i32, longitude: i32) -> Self {
        Self { longitude, latitude }
    }

    /// Construct a position from floating point latitude and longitude (degrees).
    ///
    /// # Panics
    ///
    /// If the latitude or longitude is out of range.
    #[must_use]
    #[expect(clippy::cast_possible_truncation, reason = "in range, the units fit an i32")]
    pub fn new(latitude: f64, longitude: f64) -> Self {
        assert!((-90.0..=90.0).contains(&latitude));
        assert!((-180.0..=180.0).contains(&longitude));
        let latitude = (latitude * 60.0 * 10_000.0).round() as i32;
        let longitude = (longitude * 60.0 * 10_000.0).round() as i32;
        // SAFETY: Bounds have been checked.
        unsafe { Self::new_raw(latitude, longitude) }
    }
}

impl TryFrom<(f64, f64)> for Position {
    type Error = OutOfRange;

    fn try_from((latitude, longitude): (f64, f64)) -> Result<Self, Self::Error> {
        if latitude < -90.0 || latitude > 90.0 || longitude < -180.0 || longitude > 180.0 {
            return Err(OutOfRange);
        }
        Ok(Self::new(latitude, longitude))
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
        Ok((f64::from(position.latitude) / 60.0 / 10_000.0, f64::from(position.longitude) / 60.0 / 10_000.0))
    }
}

impl Default for Position {
    /// A position that is not available: latitude 91 and longitude 181 degrees.
    fn default() -> Self {
        Self { latitude: 91 * 60 * 10_000, longitude: 181 * 60 * 10_000 }
    }
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct Leg {
    #[deku(bits = "10", endian = "big")]
    pub angle: u16,
    #[deku(bits = "11", endian = "big")]
    pub distance: u16,
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct Shape {
    pub scale: Scale,
    pub points: [Leg; 4],
}

impl Display for AreaNoticeV2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let when = (&self.start, &self.duration);
        crate::notice(
            f,
            "USCG Area Notice",
            &self.linkage,
            (&self.description, self.description == Description::Cancellation),
            None,
            when,
            &self.subareas,
        )
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

    /// Degrees go to the nearest ten-thousandth of a minute: 0.57 degrees is 341,999.99... of
    /// them in floating point, which truncating made one short.
    #[test]
    fn position_rounds() {
        let position = Position::new(0.57, -0.41);
        assert_eq!((position.latitude, position.longitude), (342_000, -246_000));
    }

    #[test]
    fn position_default() {
        let bits = Position::default().to_bits().unwrap();
        let (_, position) = Position::from_bytes((bits.as_raw_slice(), 0)).unwrap();
        assert_eq!((position.latitude, position.longitude), (91 * 60 * 10_000, 181 * 60 * 10_000));
    }
}
