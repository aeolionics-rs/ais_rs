//  SPDX-FileCopyrightText: 2026-2026. Aeolionics, LLC
//
//  SPDX-License-Identifier: Apache-2.0

use super::area_notice_v2::{Boundary, Leg, Radius, Scale, SectorBoundary};
use super::{Linkage, Version};
use ais_sixbit::SixBitText;
use ais_types::{LatLon, Minutes, MonthDayHourMinute, OutOfRange};
use core::fmt::{self, Display, Formatter};
use deku::{DekuRead, DekuWrite};

/// What a Geographic Notice is about: the table the EU inland (DAC 200, FI 42) and USCG (DAC 367,
/// FI 22) messages share, code for code. It widens the Area Notice's: codes the Area Notice
/// reserves gain meanings (ice reports, an "other" in each category), and a few move, fishing
/// prohibited from 32 to 33, the sunken vessel into 97 with the submerged object.
#[allow(non_camel_case_types)]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, DekuRead, DekuWrite)]
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
    StrongCurrents = 29,
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
    DoNotProceed = 82,
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
    RecommendedRouteThroughIce = 122,
    OtherRoute = 123,

    Reserved124 = 124,

    Other = 125,
    Cancellation = 126,
    Undefined = 127,
}

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

/// `SubArea` definition of the EU Geographic Notice, Tables 3 to 8 of its page.
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
    /// Latitude and longitude are signed values with a resolution of `1/10_000` minute.
    ///
    /// # Safety
    ///
    /// The caller must ensure the values are within range.
    #[must_use]
    pub unsafe fn new_raw(latitude: i32, longitude: i32, precision: u8) -> Self {
        Self { longitude, latitude, precision }
    }

    /// Construct a position from latitude and longitude (degrees).
    ///
    /// # Panics
    ///
    /// If the latitude or longitude is out of range.
    #[must_use]
    #[expect(clippy::cast_possible_truncation, reason = "in range, the units fit an i32")]
    pub fn new(latitude: f64, longitude: f64, precision: u8) -> Self {
        assert!((-90.0..=90.0).contains(&latitude));
        assert!((-180.0..=180.0).contains(&longitude));
        let latitude = (latitude * 60.0 * 10_000.0).round() as i32;
        let longitude = (longitude * 60.0 * 10_000.0).round() as i32;
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
        Ok((f64::from(position.latitude) / 60.0 / 10_000.0, f64::from(position.longitude) / 60.0 / 10_000.0))
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
        crate::notice(
            f,
            "EU Geographic Notice",
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

impl Display for Description {
    #[expect(clippy::too_many_lines, reason = "a code table, one arm a code")]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use Description::{
            AbandoningShip, ActiveMilitaryOPAREA, AlternativeRoute, AnchorageClosed, AnchorageOpen,
            AnchoringProhibited, AwaitInstructionsBeforeProceeding, BridgeGateLockClosed, BridgeGateLockClosing,
            BridgeGateLockInoperative, BridgeGateLockOpen, BridgeGateLockOpening, Cancellation, ChannelObstruction,
            ClearanceGranted, ClusterOfFishingVessels, ContactPortAdministration, ContactVTS, DeepDraftAnchorage,
            Derelicts, DiversDown, DoNotProceed, DredgingOperations, DriftingMines, EntryApprovalRequired,
            EntryProhibited, EventOrRegatta, FairwayClosed, FiringDanger, FishingProhibited, HarborClosed,
            HazardousSeaIce, HazardousSubstance, HeavyIcing, HighWaves, HighWind, IceEdge, IcebreakerWaitingArea,
            LocationOfResponseUnits, MARSEC1, MARSEC2, MARSEC3, MammalsInArea_ReduceSpeed,
            MammalsInArea_ReportSightings, MammalsInArea_StayClear, MarineMammalHabitat, MediumIce, NetsInWater,
            NewIce, OldIce, Other, OtherAnchorage, OtherCaution, OtherChartFeature, OtherDistress,
            OtherEnvironmentalCaution, OtherInformation, OtherInstructions, OtherReport, OtherRestriction, OtherRoute,
            PersonOverboard, PilotBoardingPosition, PlaceOfRefuge, PollutionResponseArea, PositionOfIcebreakers,
            ProceedAndAwaitInstructions, ProtectedHabitat_NoFishing, ProtectedHabitat_ReduceSpeed,
            ProtectedHabitat_StayClear, RecommendedRoute, RecommendedRouteThroughIce, ReducedClearance, ReportingIcing,
            ReportingRoute, RestrictedVisibility, RogueOrSuspiciousVessel, SarArea, SeaplaneOperations,
            SemiSubmergedObject, ShallowDraftAnchorage, ShoalArea, ShoalAreaDueEast, ShoalAreaDueNorth,
            ShoalAreaDueSouth, ShoalAreaDueWest, StormFront, StormWarning, StrongCurrents, SubmergedObject,
            SubmergedPipelineOrCable, SurveyOperations, SwimArea, ThickIce, ThinIce, TrafficCongestion, Undefined,
            UnderwaterOperations, UndeterminedIce, UnmannedVehicleOperation, VesselCollision, VesselDisabledAndAdrift,
            VesselFire, VesselFlooding, VesselGrounding, VesselListing, VesselRequestingAssistance,
            VesselRequestingMedicalAssistance, VesselSinking, VesselTransferOperations, VesselUnderAssault,
            VtsActiveTarget, YoungIce,
        };
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
            StrongCurrents => "strong currents",
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
            DoNotProceed => "do not proceed",
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
            SubmergedObject => "submerged object or sunken vessel",
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
            RecommendedRouteThroughIce => "recommended route through ice",
            OtherRoute => "other route",
            Other => "other",
            Cancellation => "cancellation",
            Undefined => "description n/a",
            reserved => return write!(f, "description {}", *reserved as u8),
        })
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
        let position = Position::new(0.57, -0.41, 4);
        assert_eq!((position.latitude, position.longitude), (342_000, -246_000));
    }

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
