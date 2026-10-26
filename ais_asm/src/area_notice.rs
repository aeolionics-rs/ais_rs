//  SPDX-FileCopyrightText: 2026-2026. Aeolionics, LLC
//
//  SPDX-License-Identifier: Apache-2.0

use super::Linkage;
use ais_sixbit::SixBitText;
use ais_types::{LatLon, Minutes, MonthDayHourMinute, OutOfRange};
use core::fmt::{self, Display, Formatter};
use deku::{DekuRead, DekuWrite};

#[allow(non_camel_case_types)]
#[repr(u8)]
#[derive(Debug, Clone, Copy, DekuRead, DekuWrite)]
#[deku(id_type = "u8", bits = 7)]
pub enum Description {
    // Cautions
    MarineMammalHabitat = 0,
    MammalsInArea_ReduceSpeed = 1,
    MammalsInArea_StayClear = 2,
    MammalsInArea_ReportSightings = 3,
    ProtectedHabitat_ReduceSpeed = 4,
    ProtectedHabitat_StayClear = 5,
    ProtectedHabitat_NoFishing = 6,
    Derelicts = 7,
    TrafficCongestion = 8,
    EventOrRegatta = 9,
    DiversDown = 10,
    SwimArea = 11,
    DredgingOperations = 12,
    SurveyOperations = 13,
    UnderwaterOperations = 14,
    SeaplaneOperations = 15,
    NetsInWater = 16,
    ClusterOfFishingVessels = 17,
    FairwayClosed = 18,
    HarborClosed = 19,
    SubmergedPipelineOrCable = 20,
    UnmannedVehicleOperation = 21,
    OtherCaution = 22,
    StormFront = 23,

    // Environmental Cautions.
    HazardousSeaIce = 24,
    StormWarning = 25,
    HighWind = 26,
    HighWaves = 27,
    RestrictedVisibility = 28,
    StringCurrents = 29,
    HeavyIcing = 30,
    HazardousSubstance = 31,
    OtherEnvironmentalCaution = 32,

    // Restrictions
    FishingProhibited = 33,
    EntryApprovalRequired = 34,
    EntryProhibited = 35,
    ActiveMilitaryOPAREA = 36,
    FiringDanger = 37,
    DriftingMines = 38,
    OtherRestriction = 39,

    // Anchorage Status
    AnchorageOpen = 40,
    AnchorageClosed = 41,
    AnchoringProhibited = 42,
    DeepDraftAnchorage = 43,
    ShallowDraftAnchorage = 44,
    VesselTransferOperations = 45,
    OtherAnchorage = 46,

    // Ice Reports
    IceEdge = 47,
    NewIce = 48,
    YoungIce = 49,
    ThinIce = 50,
    MediumIce = 51,
    ThickIce = 52,
    OldIce = 53,
    UndeterminedIce = 54,
    Reserved55 = 55,

    // Security Alerts
    MARSEC1 = 56,
    MARSEC2 = 57,
    MARSEC3 = 58,

    Reserved59 = 59,
    Reserved60 = 60,
    Reserved61 = 61,
    Reserved62 = 62,
    Reserved63 = 63,

    // Distress
    VesselDisabledAndAdrift = 64,
    VesselSinking = 65,
    AbandoningShip = 66,
    VesselRequestingMedicalAssistance = 67,
    VesselFlooding = 68,
    VesselFire = 69,
    VesselGrounding = 70,
    VesselCollision = 71,
    VesselListing = 72,
    VesselUnderAssault = 73,
    PersonOverboard = 74,
    SarArea = 75,
    PollutionResponseArea = 76,
    OtherDistress = 77,

    Reserved78 = 78,
    Reserved79 = 79,

    // Instructions
    ContactVTS = 80,
    ContactPortAdministration = 81,
    DoNoProceed = 82,
    AwaitInstructionsBeforeProceeding = 83,
    ProceedAndAwaitInstructions = 84,
    ClearanceGranted = 85,
    OtherInstructions = 86,

    Reserved87 = 87,

    // Information
    PilotBoardingPosition = 88,
    IcebreakerWaitingArea = 89,
    PlaceOfRefuge = 90,
    PositionOfIcebreakers = 91,
    LocationOfResponseUnits = 92,
    VtsActiveTarget = 93,
    RogueOrSuspiciousVessel = 94,
    VesselRequestingAssistance = 95,
    OtherInformation = 96,

    // Chart Features
    SubmergedObject = 97,
    SemiSubmergedObject = 98,
    ShoalArea = 99,
    ShoalAreaDueNorth = 100,
    ShoalAreaDueEast = 101,
    ShoalAreaDueSouth = 102,
    ShoalAreaDueWest = 103,
    ChannelObstruction = 104,
    ReducedClearance = 105,
    BridgeGateLockClosed = 106,
    BridgeGateLockOpening = 107,
    BridgeGateLockOpen = 108,
    BridgeGateLockClosing = 109,
    BridgeGateLockInoperative = 110,
    OtherChartFeature = 111,

    // Reports
    ReportingIcing = 112,
    ReportingRoute = 113,
    OtherReport = 114,

    Reserved115 = 115,
    Reserved116 = 116,
    Reserved117 = 117,
    Reserved118 = 118,
    Reserved119 = 119,

    // Routes
    RecommendedRoute = 120,
    AlternativeRoute = 121,
    RecommendedRouteThruIce = 122,
    OtherRoute = 123,

    Reserved124 = 124,

    Other = 125,
    Cancellation = 126,
    Undefined = 127,
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct Scale(#[deku(bits = "2")] pub u8);

#[derive(Debug, DekuRead, DekuWrite)]
pub struct Radius(#[deku(bits = "12", endian = "big")] pub u16);

#[derive(Debug, DekuRead, DekuWrite)]
pub struct Boundary {
    #[deku(bits = "8")]
    pub east: u8,
    #[deku(bits = "8")]
    pub north: u8,
    #[deku(bits = "9", endian = "big")]
    pub orientation: u16,
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct SectorBoundary {
    pub radius: Radius,
    #[deku(bits = "9", endian = "big")]
    pub left: u16,
    #[deku(bits = "9", endian = "big")]
    pub right: u16,
}

/// Structure of an Area Notice message, IMO SN.1/Circ.289 (DAC 1, FI 22 broadcast, FI 23
/// addressed): <https://www.e-navigation.nl/content/area-notice-0> and
/// <https://www.e-navigation.nl/content/area-notice>, which carry the layout.
#[derive(Debug, DekuRead, DekuWrite)]
pub struct AreaNotice {
    pub linkage: Linkage,
    pub description: Description,
    pub start: MonthDayHourMinute,
    pub duration: Minutes,
    #[deku(read_all)]
    pub subareas: Vec<SubArea>,
}

/// Original SubArea definition.
///
/// This version requires 87 bits.
#[derive(Debug, DekuRead, DekuWrite)]
#[deku(id_type = "u8", bits = 3)]
pub enum SubArea {
    #[deku(id = 0)]
    Circle {
        scale: Scale,
        position: Position,
        #[deku(pad_bits_after = "18")]
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
    Polyline(#[deku(pad_bits_after = "2")] Shape),
    #[deku(id = 4)]
    Polygon(#[deku(pad_bits_after = "2")] Shape),
    #[deku(id = 5)]
    AssociatedText(SixBitText<14>),
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct Position {
    #[deku(bits = "25", endian = "big")]
    longitude: i32,
    #[deku(bits = "24", endian = "big")]
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
        let latitude = (latitude * 60.0 * 1_000.0) as i32;
        let longitude = (longitude * 60.0 * 1_000.0) as i32;
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
        if position.latitude.abs() > 90 * 60 * 1_000 || position.longitude.abs() > 180 * 60 * 1_000 {
            return Err(OutOfRange);
        }
        Ok(((position.latitude as f64) / 60.0 / 1_000.0, (position.longitude as f64) / 60.0 / 1_000.0))
    }
}

impl Default for Position {
    /// A position that is not available: latitude 91 and longitude 181 degrees.
    fn default() -> Self {
        Self { latitude: 91 * 60 * 1_000, longitude: 181 * 60 * 1_000, precision: 4 }
    }
}

/// A leg of a polyline or polygon, from the previous point to the next.
#[derive(Debug, DekuRead, DekuWrite)]
pub struct Leg {
    /// The true bearing to the next point in half degrees, 0 to 719, or 720 if the leg is not used.
    #[deku(bits = "10", endian = "big")]
    pub angle: u16,
    /// The distance to the next point in metres, multiplied by ten to the power of the shape's
    /// scale, or 0 if the leg is not used.
    #[deku(bits = "10", endian = "big")]
    pub distance: u16,
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct Shape {
    pub scale: Scale,
    pub points: [Leg; 4],
}

impl Display for Description {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use Description::*;
        f.write_str(match self {
            MarineMammalHabitat => "marine mammal habitat",
            MammalsInArea_ReduceSpeed => "mammals in area, reduce speed",
            MammalsInArea_StayClear => "mammals in area, stay clear",
            MammalsInArea_ReportSightings => "mammals in area, report sightings",
            ProtectedHabitat_ReduceSpeed => "protected habitat, reduce speed",
            ProtectedHabitat_StayClear => "protected habitat, stay clear",
            ProtectedHabitat_NoFishing => "protected habitat, no fishing",
            Derelicts => "derelicts",
            TrafficCongestion => "traffic congestion",
            EventOrRegatta => "event or regatta",
            DiversDown => "divers down",
            SwimArea => "swim area",
            DredgingOperations => "dredging operations",
            SurveyOperations => "survey operations",
            UnderwaterOperations => "underwater operations",
            SeaplaneOperations => "seaplane operations",
            NetsInWater => "nets in water",
            ClusterOfFishingVessels => "cluster of fishing vessels",
            FairwayClosed => "fairway closed",
            HarborClosed => "harbour closed",
            SubmergedPipelineOrCable => "submerged pipeline or cable",
            UnmannedVehicleOperation => "unmanned vehicle operation",
            OtherCaution => "other caution",
            StormFront => "storm front",
            HazardousSeaIce => "hazardous sea ice",
            StormWarning => "storm warning",
            HighWind => "high wind",
            HighWaves => "high waves",
            RestrictedVisibility => "restricted visibility",
            StringCurrents => "strong currents",
            HeavyIcing => "heavy icing",
            HazardousSubstance => "hazardous substance",
            OtherEnvironmentalCaution => "other environmental caution",
            FishingProhibited => "fishing prohibited",
            EntryApprovalRequired => "entry approval required",
            EntryProhibited => "entry prohibited",
            ActiveMilitaryOPAREA => "active military OPAREA",
            FiringDanger => "firing danger",
            DriftingMines => "drifting mines",
            OtherRestriction => "other restriction",
            AnchorageOpen => "anchorage open",
            AnchorageClosed => "anchorage closed",
            AnchoringProhibited => "anchoring prohibited",
            DeepDraftAnchorage => "deep draft anchorage",
            ShallowDraftAnchorage => "shallow draft anchorage",
            VesselTransferOperations => "vessel transfer operations",
            OtherAnchorage => "other anchorage",
            IceEdge => "ice edge",
            NewIce => "new ice",
            YoungIce => "young ice",
            ThinIce => "thin ice",
            MediumIce => "medium ice",
            ThickIce => "thick ice",
            OldIce => "old ice",
            UndeterminedIce => "undetermined ice",
            MARSEC1 => "MARSEC level 1",
            MARSEC2 => "MARSEC level 2",
            MARSEC3 => "MARSEC level 3",
            VesselDisabledAndAdrift => "vessel disabled and adrift",
            VesselSinking => "vessel sinking",
            AbandoningShip => "abandoning ship",
            VesselRequestingMedicalAssistance => "vessel requesting medical assistance",
            VesselFlooding => "vessel flooding",
            VesselFire => "vessel fire",
            VesselGrounding => "vessel grounding",
            VesselCollision => "vessel collision",
            VesselListing => "vessel listing",
            VesselUnderAssault => "vessel under assault",
            PersonOverboard => "person overboard",
            SarArea => "SAR area",
            PollutionResponseArea => "pollution response area",
            OtherDistress => "other distress",
            ContactVTS => "contact VTS",
            ContactPortAdministration => "contact port administration",
            DoNoProceed => "do not proceed",
            AwaitInstructionsBeforeProceeding => "await instructions before proceeding",
            ProceedAndAwaitInstructions => "proceed and await instructions",
            ClearanceGranted => "clearance granted",
            OtherInstructions => "other instructions",
            PilotBoardingPosition => "pilot boarding position",
            IcebreakerWaitingArea => "icebreaker waiting area",
            PlaceOfRefuge => "place of refuge",
            PositionOfIcebreakers => "position of icebreakers",
            LocationOfResponseUnits => "location of response units",
            VtsActiveTarget => "VTS active target",
            RogueOrSuspiciousVessel => "rogue or suspicious vessel",
            VesselRequestingAssistance => "vessel requesting assistance",
            OtherInformation => "other information",
            SubmergedObject => "submerged object",
            SemiSubmergedObject => "semi-submerged object",
            ShoalArea => "shoal area",
            ShoalAreaDueNorth => "shoal area due north",
            ShoalAreaDueEast => "shoal area due east",
            ShoalAreaDueSouth => "shoal area due south",
            ShoalAreaDueWest => "shoal area due west",
            ChannelObstruction => "channel obstruction",
            ReducedClearance => "reduced clearance",
            BridgeGateLockClosed => "bridge, gate or lock closed",
            BridgeGateLockOpening => "bridge, gate or lock opening",
            BridgeGateLockOpen => "bridge, gate or lock open",
            BridgeGateLockClosing => "bridge, gate or lock closing",
            BridgeGateLockInoperative => "bridge, gate or lock inoperative",
            OtherChartFeature => "other chart feature",
            ReportingIcing => "reporting icing",
            ReportingRoute => "reporting route",
            OtherReport => "other report",
            RecommendedRoute => "recommended route",
            AlternativeRoute => "alternative route",
            RecommendedRouteThruIce => "recommended route through ice",
            OtherRoute => "other route",
            Other => "other",
            Cancellation => "cancellation",
            Undefined => "description n/a",
            reserved => return write!(f, "description {}", *reserved as u8),
        })
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

impl Display for AreaNotice {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let when = (&self.start, &self.duration);
        crate::notice(f, "Area Notice", &self.linkage, &self.description, None, when, &self.subareas)
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
            SubArea::Circle { scale, position, radius } => {
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
    /// In degrees to the thousandth of a minute it carries, five places.
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match <(f64, f64)>::try_from(self) {
            Ok((latitude, longitude)) => LatLon { latitude, longitude, decimals: 5 }.fmt(f),
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

    #[test]
    fn position_degrees() {
        let position = Position::try_from((47.61, -122.4)).unwrap();
        let (latitude, longitude) = <(f64, f64)>::try_from(&position).unwrap();
        assert_eq!(((latitude * 1e4).round(), (longitude * 1e4).round()), (476100.0, -1224000.0));

        // The limits are in range.
        let position = Position::try_from((-90.0, 180.0)).unwrap();
        assert_eq!(<(f64, f64)>::try_from(&position).unwrap(), (-90.0, 180.0));
    }

    #[test]
    fn position_default() {
        let bits = Position::default().to_bits().unwrap();
        let (_, position) = Position::from_bytes((bits.as_raw_slice(), 0)).unwrap();
        assert_eq!((position.latitude, position.longitude), (91 * 60 * 1_000, 181 * 60 * 1_000));
        assert!(<(f64, f64)>::try_from(&position).is_err());

        // Either coordinate being not available makes the position not available.
        let position = Position { latitude: 0, longitude: 181 * 60 * 1_000, precision: 4 };
        assert!(<(f64, f64)>::try_from(&position).is_err());
        let position = Position { latitude: 91 * 60 * 1_000, longitude: 0, precision: 4 };
        assert!(<(f64, f64)>::try_from(&position).is_err());
    }
}
