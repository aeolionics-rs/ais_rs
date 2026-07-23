//  SPDX-FileCopyrightText: 2026-2026. Aeolionics, LLC
//
//  SPDX-License-Identifier: Apache-2.0

use super::Linkage;
use crate::types::sixbit::SixBitText;
use crate::types::{Minutes, MonthDayHourMinute};
use deku::{DekuRead, DekuWrite};

#[allow(non_camel_case_types)]
#[repr(u8)]
#[derive(DekuRead, DekuWrite)]
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

#[derive(DekuRead, DekuWrite)]
pub struct Scale(#[deku(bits = "2")] pub u8);

#[derive(DekuRead, DekuWrite)]
pub struct Radius(#[deku(bits = "12", endian = "big")] pub u16);

#[derive(DekuRead, DekuWrite)]
pub struct Boundary {
    #[deku(bits = "8")]
    pub east: u8,
    #[deku(bits = "8")]
    pub north: u8,
    #[deku(bits = "9", endian = "big")]
    pub orientation: u16,
}

#[derive(DekuRead, DekuWrite)]
pub struct SectorBoundary {
    radius: Radius,
    #[deku(bits = "9", endian = "big")]
    pub left: u16,
    #[deku(bits = "9", endian = "big")]
    pub right: u16,
}

/// Structure of an Area Notice message.
#[derive(DekuRead, DekuWrite)]
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
#[derive(DekuRead, DekuWrite)]
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

#[derive(DekuRead, DekuWrite)]
pub struct Position {
    #[deku(bits = "25", endian = "big")]
    pub longitude: i32,
    #[deku(bits = "24", endian = "big")]
    pub latitude: i32,
    #[deku(bits = "3")]
    pub precision: u8,
}

#[derive(DekuRead, DekuWrite)]
pub struct Leg {
    #[deku(bits = "10", endian = "big")]
    pub angle: u16,
    #[deku(bits = "10", endian = "big")]
    pub distance: u16,
}

#[derive(DekuRead, DekuWrite)]
pub struct Shape {
    pub scale: Scale,
    pub points: [Leg; 4],
}
