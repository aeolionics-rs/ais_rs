//  SPDX-FileCopyrightText: 2026-2026. Aeolionics, LLC
//
//  SPDX-License-Identifier: Apache-2.0

use super::Linkage;
use ais_sixbit::SixBitText;
use ais_types::{Minutes, MonthDayHourMinute, OutOfRange};
use deku::{DekuRead, DekuWrite};

#[allow(non_camel_case_types)]
#[repr(u8)]
#[derive(Debug, DekuRead, DekuWrite)]
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
    radius: Radius,
    #[deku(bits = "9", endian = "big")]
    pub left: u16,
    #[deku(bits = "9", endian = "big")]
    pub right: u16,
}

/// Structure of an Area Notice message.
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

impl From<&Position> for (f64, f64) {
    /// The latitude and longitude in degrees.
    fn from(position: &Position) -> Self {
        ((position.latitude as f64) / 60.0 / 1_000.0, (position.longitude as f64) / 60.0 / 1_000.0)
    }
}

impl Default for Position {
    /// A position that is not available: latitude 91 and longitude 181 degrees.
    fn default() -> Self {
        Self {
            latitude: 91 * 60 * 1_000,
            longitude: 181 * 60 * 1_000,
            precision: 4,
        }
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

#[cfg(test)]
mod test {
    use super::*;
    use deku::{DekuContainerRead, DekuContainerWrite};

    #[test]
    fn position_degrees() {
        let position = Position::try_from((47.61, -122.4)).unwrap();
        let (latitude, longitude) = <(f64, f64)>::from(&position);
        assert_eq!(((latitude * 1e4).round(), (longitude * 1e4).round()), (476100.0, -1224000.0));
    }

    #[test]
    fn position_default() {
        let bits = Position::default().to_bits().unwrap();
        let (_, position) = Position::from_bytes((bits.as_raw_slice(), 0)).unwrap();
        assert_eq!(<(f64, f64)>::from(&position), (91.0, 181.0));
    }
}
