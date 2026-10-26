//  SPDX-FileCopyrightText: 2026-2026. Aeolionics, LLC
//
//  SPDX-License-Identifier: Apache-2.0

//! Common types.

use ais_sixbit::SixBitText;
use ais_types::{LatLon, OutOfRange};
use chrono::{DateTime, Datelike, Timelike, Utc};
use core::fmt::{Display, Formatter};
use deku::{DekuRead, DekuWrite, deku_derive};

/// Indicates how many times a message has been repeated.
#[repr(u8)]
#[derive(Debug, Copy, Clone, Default, DekuRead, DekuWrite)]
#[deku(id_type = "u8", bits = "2")]
pub enum RepeatCount {
    /// Original message (default).
    #[default]
    Original = 0,
    /// First retransmission.
    Once = 1,
    /// Second retransmission.
    Twice = 2,
    /// Third and final repeat; do not repeat any more.
    Final = 3,
}

/// Maritime Mobile Service Identity.
///
/// Written as text it is always nine digits, its leading zeros kept, so that a coast station's
/// `00MIDxxxx` or a group's `0MIDxxxxx` reads as what it is.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, DekuRead, DekuWrite)]
pub struct MMSI(#[deku(bits = "30", endian = "big")] u32);
impl MMSI {
    /// Create a new MMS identifier.
    ///
    /// # Errors
    ///
    /// `OutOfRange` if the value is not a valid MMSI, more than nine digits.
    pub fn new(mmsi: u32) -> Result<Self, OutOfRange> {
        match mmsi {
            ..=999_999_999 => Ok(MMSI(mmsi)),
            _ => Err(OutOfRange),
        }
    }
}

impl From<MMSI> for u32 {
    fn from(mmsi: MMSI) -> Self {
        mmsi.0
    }
}

impl Display for MMSI {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:09}", self.0)
    }
}

impl core::str::FromStr for MMSI {
    type Err = OutOfRange;

    /// An MMSI from its text: nine digits, no more and no fewer.
    fn from_str(text: &str) -> Result<Self, OutOfRange> {
        if text.len() != 9 || !text.bytes().all(|b| b.is_ascii_digit()) {
            return Err(OutOfRange);
        }
        MMSI::new(text.parse().map_err(|_| OutOfRange)?)
    }
}

/// Version of the AIS standard being used.
#[repr(u8)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, DekuRead, DekuWrite)]
#[deku(id_type = "u8", bits = "2")]
pub enum AisVersion {
    M1371_1 = 0,
    M1371_3 = 1,
    M1371_5 = 2,
    M1371_6 = 3,
}

/// Information about the equipment and its manufacturer.
#[derive(Debug, Copy, Clone, DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct Manufacturer {
    /// Manufacturer code assigned by NMEA/SAE.
    #[deku(bits = 18)]
    pub id: u32,
    /// Equipment model identifier.
    #[deku(bits = 4)]
    pub model: u32,
    /// Equipment serial number.
    #[deku(bits = 20)]
    pub serial: u32,
}

/// A vessel's IMO number.
#[deku_derive(DekuRead, DekuWrite)]
#[derive(Debug)]
#[deku(endian = "big")]
pub struct ImoNumber(#[deku(bits = "30")] pub u32);

/// The type of ship.
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, DekuRead, DekuWrite)]
#[deku(id_type = "u8")]
pub enum ShipType {
    #[deku(id = "0")]
    NotAvailable,
    #[deku(id = "1")]
    Research,
    #[deku(id = "2")]
    Training,
    #[deku(id = "3")]
    Government,
    #[deku(id = "4")]
    IceBreaker,
    #[deku(id = "5")]
    BuoyTender,
    #[deku(id = "6")]
    CableLayer,
    #[deku(id = "7")]
    PipeLayer,
    #[deku(id = "9")]
    SpecialPurpose,
    #[deku(id = "11")]
    FPSO,
    #[deku(id = "12")]
    FishFactory,
    #[deku(id = "13")]
    FishFarmSupport,
    #[deku(id = "14")]
    OffshoreSupport,
    #[deku(id = "17")]
    Construction,
    #[deku(id = "18")]
    CrewBoat,
    #[deku(id = "19")]
    SupportVessel,
    #[deku(id = "20")]
    WingInGround,
    #[deku(id = "21")]
    Wig_X,
    #[deku(id = "22")]
    Wig_Y,
    #[deku(id = "23")]
    Wig_Z,
    #[deku(id = "24")]
    Wig_OS,
    #[deku(id = "29")]
    Wig_Other,
    #[deku(id = "30")]
    Fishing,
    #[deku(id = "31")]
    Towing,
    #[deku(id = "32")]
    LongTow,
    #[deku(id = "33")]
    Dredger,
    #[deku(id = "34")]
    Diving,
    #[deku(id = "35")]
    Warship,
    #[deku(id = "36")]
    Sailing,
    #[deku(id = "37")]
    Pleasure,
    #[deku(id = "38")]
    Trawler,
    #[deku(id = "39")]
    Patrol,
    #[deku(id = "40")]
    HighSpeed,
    #[deku(id = "41")]
    HighSpeed_X,
    #[deku(id = "42")]
    HighSpeed_Y,
    #[deku(id = "43")]
    HighSpeed_Z,
    #[deku(id = "44")]
    HighSpeed_OS,
    #[deku(id = "45")]
    HighSpeed_Passenger,
    #[deku(id = "46")]
    HighSpeed_RoRo,
    #[deku(id = "49")]
    HighSpeed_Other,
    #[deku(id = "50")]
    Pilot,
    #[deku(id = "51")]
    SearchRescue,
    #[deku(id = "52")]
    Tug,
    #[deku(id = "53")]
    Tender,
    #[deku(id = "54")]
    Responder,
    #[deku(id = "55")]
    LawEnforcement,
    #[deku(id = "58")]
    Medical,
    #[deku(id = "59")]
    NotAtWar,
    #[deku(id = "60")]
    Passenger,
    #[deku(id = "61")]
    Passenger_X,
    #[deku(id = "62")]
    Passenger_Y,
    #[deku(id = "63")]
    Passenger_Z,
    #[deku(id = "64")]
    Passenger_OS,
    #[deku(id = "65")]
    CruiseShip,
    #[deku(id = "66")]
    Ferry,
    #[deku(id = "67")]
    Excursion,
    #[deku(id = "69")]
    Passenger_Other,
    #[deku(id = "70")]
    Cargo,
    #[deku(id = "71")]
    Cargo_X,
    #[deku(id = "72")]
    Cargo_Y,
    #[deku(id = "73")]
    Cargo_Z,
    #[deku(id = "74")]
    Cargo_OS,
    #[deku(id = "75")]
    BulkCarrier,
    #[deku(id = "76")]
    ContainerShip,
    #[deku(id = "77")]
    RoRoCarrier,
    #[deku(id = "78")]
    LandingCraft,
    #[deku(id = "79")]
    Cargo_Other,
    #[deku(id = "80")]
    Tanker,
    #[deku(id = "81")]
    Tanker_X,
    #[deku(id = "82")]
    Tanker_Y,
    #[deku(id = "83")]
    Tanker_Z,
    #[deku(id = "84")]
    Tanker_OS,
    #[deku(id = "85")]
    Tanker_NonHazardous,
    #[deku(id = "86")]
    TugAndTank,
    #[deku(id = "89")]
    Tanker_Other,
    #[deku(id = "90")]
    Other,
    #[deku(id = "91")]
    Other_X,
    #[deku(id = "92")]
    Other_Y,
    #[deku(id = "93")]
    Other_Z,
    #[deku(id = "94")]
    Other_OS,
    #[deku(id = "99")]
    Other_NoMoreInformation,
    #[deku(id_pat = "_")]
    Unknown(u8),
}

/// Ship's navigational status.
#[repr(u8)]
#[derive(Debug, Copy, Clone, Default, DekuRead, DekuWrite)]
#[deku(id_type = "u8", bits = "4")]
pub enum NavigationalStatus {
    /// Vessel is under way under power.
    UnderWay = 0,
    /// Vessel is at anchor.
    AtAnchor = 1,
    /// Vessel is not under command.
    NotUnderCommand = 2,
    /// Vessel is restricted in her ability to manoeuvre.
    RestrictedManoeuvrability = 3,
    /// Vessel is constrained by her draught.
    ConstrainedByDraught = 4,
    /// Vessel is moored.
    Moored = 5,
    /// Vessel is aground.
    Aground = 6,
    /// Vessel engaged in fishing.
    Fishing = 7,
    /// Vessel under way under sailing only.
    Sailing = 8,
    Reserved9 = 9,
    Reserved10 = 10,
    /// Power-driven vessel towing astern.
    TowingAstern = 11,
    /// Power drive vessel pushing ahead or towing alongside.
    Pushing = 12,
    Reserved13 = 13,
    /// Active AIS-SART, MOB-AIS or EPIRB-AIS
    ActiveSART = 14,
    /// Undefined (default), or SART under test.
    #[default]
    Undefined = 15,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, DekuRead, DekuWrite)]
#[deku(id_type = "u8", bits = "1")]
pub enum SatelliteTxControl {
    Stop = 0,
    Send = 1,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, DekuRead, DekuWrite)]
#[deku(id_type = "u8", bits = "1")]
pub enum AssignedMode {
    Autonomous = 0,
    Assigned = 1,
}

#[allow(non_camel_case_types)]
#[repr(u8)]
#[derive(Debug, DekuRead, DekuWrite)]
#[deku(id_type = "u8", bits = "1")]
pub enum AltitudeSensor {
    GNSS = 0,
    Barometric = 1,
}

#[allow(non_camel_case_types)]
#[repr(u8)]
#[derive(Debug, DekuRead, DekuWrite)]
#[deku(id_type = "u8", bits = "1")]
pub enum CommunicationStateType {
    SOTDMA = 0,
    ITDMA = 1,
}

#[deku_derive(DekuRead, DekuWrite)]
#[derive(Debug)]
pub struct CommunicationState {
    #[deku(bits = 2)]
    pub sync: u8,
    #[deku(bits = 3)]
    pub slot_time_out: u8,
    #[deku(bits = 14, endian = "big")]
    pub sub_message: u16,
}

#[repr(u8)]
#[derive(Debug, DekuRead, DekuWrite)]
#[deku(id_type = "u8", bits = "1")]
pub enum DteReady {
    Available = 0,
    NotAvailable = 1,
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct OverallDimension {
    #[deku(bits = "9", endian = "big")]
    pub a: u16,
    #[deku(bits = "9", endian = "big")]
    pub b: u16,
    #[deku(bits = "6")]
    pub c: u8,
    #[deku(bits = "6")]
    pub d: u8,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, DekuRead, DekuWrite)]
#[deku(id_type = "u8", bits = "4")]
pub enum PositioningDevice {
    NotAvailable = 0,
    GPS = 1,
    GLONASS = 2,
    GNSS = 3,
    Loran = 4,
    Chayka = 5,
    INS = 6,
    Manual = 7,
    Galileo = 8,
    BDS = 9,
    Reserved10 = 10,
    Reserved11 = 11,
    PNT = 12,
    Inertial = 13,
    Terrestrial = 14,
    InternalGNSS = 15,
}

/// The type of aid-to-navigation (ITU-R M.1371-6, Annex A7-3.19.1, Table 72).
///
/// Fixed AtoN are values 5 to 19, floating AtoN are values 20 to 31.
#[repr(u8)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, DekuRead, DekuWrite)]
#[deku(id_type = "u8", bits = "5")]
pub enum AtonType {
    /// Default, type of AtoN not specified.
    #[default]
    NotSpecified = 0,
    /// Reference point.
    ReferencePoint = 1,
    /// RACON.
    Racon = 2,
    /// Fixed structure off shore, such as oil platforms, wind farms and rigs.
    FixedStructure = 3,
    /// Emergency wreck marking buoy.
    EmergencyWreckMarkingBuoy = 4,
    /// Light, without sectors.
    Light = 5,
    /// Light, with sectors.
    SectorLight = 6,
    /// Leading light front.
    LeadingLightFront = 7,
    /// Leading light rear.
    LeadingLightRear = 8,
    /// Beacon, cardinal N.
    BeaconCardinalNorth = 9,
    /// Beacon, cardinal E.
    BeaconCardinalEast = 10,
    /// Beacon, cardinal S.
    BeaconCardinalSouth = 11,
    /// Beacon, cardinal W.
    BeaconCardinalWest = 12,
    /// Beacon, port hand.
    BeaconPortHand = 13,
    /// Beacon, starboard hand.
    BeaconStarboardHand = 14,
    /// Beacon, preferred channel port hand.
    BeaconPreferredChannelPortHand = 15,
    /// Beacon, preferred channel starboard hand.
    BeaconPreferredChannelStarboardHand = 16,
    /// Beacon, isolated danger.
    BeaconIsolatedDanger = 17,
    /// Beacon, safe water.
    BeaconSafeWater = 18,
    /// Beacon, special mark.
    BeaconSpecialMark = 19,
    /// Cardinal mark N.
    CardinalNorth = 20,
    /// Cardinal mark E.
    CardinalEast = 21,
    /// Cardinal mark S.
    CardinalSouth = 22,
    /// Cardinal mark W.
    CardinalWest = 23,
    /// Port hand mark.
    PortHand = 24,
    /// Starboard hand mark.
    StarboardHand = 25,
    /// Preferred channel port hand.
    PreferredChannelPortHand = 26,
    /// Preferred channel starboard hand.
    PreferredChannelStarboardHand = 27,
    /// Isolated danger.
    IsolatedDanger = 28,
    /// Safe water.
    SafeWater = 29,
    /// Special mark.
    SpecialMark = 30,
    /// Light vessel, LANBY or rigs.
    LightVessel = 31,
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct UtcDateTime {
    #[deku(bits = 14, endian = "big")]
    pub year: u16,
    #[deku(bits = 4)]
    pub month: u8,
    #[deku(bits = 5)]
    pub day: u8,
    #[deku(bits = 5)]
    pub hour: u8,
    #[deku(bits = 6)]
    pub minute: u8,
    #[deku(bits = 6)]
    pub second: u8,
}

impl From<DateTime<Utc>> for UtcDateTime {
    /// A year outside the fourteen bits AIS gives it, before the common era say, wraps.
    #[expect(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        reason = "chrono's month, day, hour, minute and second are small; the year as documented"
    )]
    fn from(value: DateTime<Utc>) -> Self {
        Self {
            year: value.year() as u16,
            month: value.month() as u8,
            day: value.day() as u8,
            hour: value.hour() as u8,
            minute: value.minute() as u8,
            second: value.second() as u8,
        }
    }
}

impl Default for UtcDateTime {
    fn default() -> Self {
        Self { year: 0, month: 0, day: 0, hour: 24, minute: 60, second: 60 }
    }
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct QuietTime(#[deku(bits = 4)] pub u32);

#[derive(Debug, Clone, Copy, Default, DekuRead, DekuWrite)]
#[deku(id_type = "u8", bits = "6")]
pub enum Timestamp {
    #[deku(id_pat = "0..60")]
    Second(u8),
    #[default]
    #[deku(id = "60")]
    Unavailable,
    #[deku(id = "61")]
    Manual,
    #[deku(id = "62")]
    Estimated,
    #[deku(id = "63")]
    Inoperative,
}

impl Timestamp {
    #[must_use]
    #[expect(clippy::cast_possible_truncation, reason = "a second is 0 to 59")]
    pub fn now() -> Self {
        Self::Second(Utc::now().second() as u8)
    }
}

impl From<Timestamp> for u8 {
    fn from(value: Timestamp) -> Self {
        match value {
            Timestamp::Second(second) => second,
            Timestamp::Unavailable => 60,
            Timestamp::Manual => 61,
            Timestamp::Estimated => 62,
            Timestamp::Inoperative => 63,
        }
    }
}

pub type Name = SixBitText<20>;

pub type CallSign = SixBitText<7>;
#[derive(Debug, DekuRead, DekuWrite)]
pub struct RateOfTurn(#[deku(bits = 8)] pub i8);

impl Default for RateOfTurn {
    fn default() -> Self {
        Self(-128)
    }
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct SpeedOverGround(#[deku(bits = 10, endian = "big")] pub u16);

impl Default for SpeedOverGround {
    fn default() -> Self {
        SpeedOverGround(1023)
    }
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct CourseOverGround(#[deku(bits = 12, endian = "big")] pub u16);

impl Default for CourseOverGround {
    fn default() -> Self {
        CourseOverGround(3600)
    }
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct Heading(#[deku(bits = 9, endian = "big")] pub u16);

impl Default for Heading {
    fn default() -> Self {
        Self(511)
    }
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct Altitude(#[deku(bits = 12, endian = "big")] pub u16);

/// Receiver autonomous integrity monitoring (RAIM) indicator.
///
/// See M-1371 Table 48.
#[repr(u8)]
#[derive(Debug, Copy, Clone, Default, DekuRead, DekuWrite)]
#[deku(id_type = "u8", bits = "1")]
pub enum RaimFlag {
    /// RAIM is not in use.
    #[default]
    NotInUse = 0,
    /// RAIM is in use.
    InUse = 1,
}

/// Position accuracy based on M-1371 Table 48.
#[repr(u8)]
#[derive(Debug, Copy, Clone, Default, DekuRead, DekuWrite)]
#[deku(id_type = "u8", bits = "1")]
pub enum PositionAccuracy {
    /// Accuracy > 10m (default)
    #[default]
    Low = 0,
    /// Accuracy < 10m
    High = 1,
}

/// Station position, stored at a resolution of `1/10_000` minute.
#[derive(Debug, Copy, Clone, DekuRead, DekuWrite)]
pub struct StationPosition {
    pub accuracy: PositionAccuracy,
    #[deku(bits = 28, endian = "big")]
    pub longitude: i32,
    #[deku(bits = 27, endian = "big")]
    pub latitude: i32,
}
impl StationPosition {
    /// Construct from raw components.
    ///
    /// Latitude and longitude are signed values with a resolution of `1/10_000` minute.
    ///
    /// # Safety
    ///
    /// The caller must ensure the values are within range.
    #[must_use]
    pub unsafe fn new_raw(latitude: i32, longitude: i32, accuracy: PositionAccuracy) -> Self {
        Self { accuracy, longitude, latitude }
    }

    /// Construct a position from latitude and longitude (degrees).
    ///
    /// # Panics
    ///
    /// If the latitude or longitude is out of range.
    #[must_use]
    #[expect(clippy::cast_possible_truncation, reason = "in range, the units fit an i32")]
    pub fn new(latitude: f64, longitude: f64, accuracy: PositionAccuracy) -> Self {
        assert!((-90.0..=90.0).contains(&latitude));
        assert!((-180.0..=180.0).contains(&longitude));
        let latitude = (latitude * 60.0 * 10_000.0) as i32;
        let longitude = (longitude * 60.0 * 10_000.0) as i32;
        // SAFETY: Bounds have been checked.
        unsafe { Self::new_raw(latitude, longitude, accuracy) }
    }
}

impl TryFrom<(f64, f64)> for StationPosition {
    type Error = OutOfRange;

    fn try_from((latitude, longitude): (f64, f64)) -> Result<Self, Self::Error> {
        if latitude < -90.0 || latitude > 90.0 || longitude < -180.0 || longitude > 180.0 {
            return Err(OutOfRange);
        }
        Ok(Self::new(latitude, longitude, PositionAccuracy::default()))
    }
}

impl TryFrom<StationPosition> for (f64, f64) {
    type Error = OutOfRange;

    /// The latitude and longitude in degrees.
    ///
    /// A position that is not available, latitude 91 or longitude 181 degrees, is out of range.
    fn try_from(value: StationPosition) -> Result<Self, Self::Error> {
        if value.latitude.abs() > 90 * 60 * 10_000 || value.longitude.abs() > 180 * 60 * 10_000 {
            return Err(OutOfRange);
        }
        Ok((f64::from(value.latitude) / 60.0 / 10_000.0, f64::from(value.longitude) / 60.0 / 10_000.0))
    }
}

impl Default for StationPosition {
    fn default() -> Self {
        Self {
            accuracy: PositionAccuracy::default(),
            longitude: 0x0679_1AC0, // Constant from spec = 181 * 60 * 10_000
            latitude: 0x0341_2140,  // Constant from spec = 91 * 60 * 10_000
        }
    }
}

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct GnssPosition {
    #[deku(bits = 18)]
    pub longitude: i32,
    #[deku(bits = 17)]
    pub latitude: i32,
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct GnssArea {
    pub upper_right: GnssPosition,
    pub lower_left: GnssPosition,
}

/// Indicates if the vessel is performing a special manoeuvre.
#[repr(u8)]
#[derive(Debug, Default, DekuRead, DekuWrite)]
#[deku(id_type = "u8", bits = "2")]
pub enum SpecialManoeuvre {
    /// Information is not available (default).
    #[default]
    NotAvailable = 0,
    /// Vessel is not engaged in a special manoeuvre.
    NotEngaged = 1,
    /// Vessel is engaged in a special manoeuvre (e.g. a passing arrangement)
    Engaged = 2,
    /// Reserved for regional use.
    Regional = 3,
}

/// Transmitter output power.
#[repr(u8)]
#[derive(Debug, Default, DekuRead, DekuWrite)]
#[deku(id_type = "u8", bits = "1")]
pub enum TransmitPower {
    /// Transmitting in high power mode (default).
    #[default]
    High = 0,
    /// Transmitting in low power mode.
    Low = 1,
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct Sequence(#[deku(bits = 2)] pub u8);

#[repr(u8)]
#[derive(Debug, DekuRead, DekuWrite)]
#[deku(id_type = "u8", bits = "2")]
pub enum VdesCapability {
    AisOnly = 0,
    Vdes = 1,
    VdesTerrestrial = 2,
    VdesSatellite = 3,
}

#[repr(u8)]
#[derive(Debug, DekuRead, DekuWrite)]
#[deku(id_type = "u8", bits = "1")]
pub enum PositionLatency {
    Low = 0,
    High = 1,
}

#[allow(non_camel_case_types)]
#[derive(Debug, DekuRead, DekuWrite)]
#[deku(id_type = "u8", bits = "4")]
pub enum StationSelection {
    #[deku(id = "0")]
    AllMobile,
    #[deku(id = "1")]
    OnlyClassA,
    #[deku(id = "2")]
    OnlyClassB,
    #[deku(id = "3")]
    OnlyAirborne,
    #[deku(id = "4")]
    OnlyClassB_SO,
    #[deku(id = "5")]
    OnlyClassB_CS,
    #[deku(id = "6")]
    AllWaterways,
    #[deku(id = "10")]
    AllInCoverageArea,
    #[deku(id_pat = "_")]
    Other(u8),
}

#[repr(u8)]
#[allow(non_camel_case_types)]
#[derive(Debug, DekuRead, DekuWrite)]
#[deku(id_type = "u8", bits = "2")]
pub enum TxRxMode {
    TxAB_RxAB = 0,
    TxA_RxAB = 1,
    TxB_RxAB = 2,
    Reserved = 3,
}

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(id_type = "u8", bits = "4")]
pub enum ReportingInterval {
    #[deku(id = "0")]
    Autonomous,
    #[deku(id = "1")]
    TenMinutes,
    #[deku(id = "2")]
    SixMinutes,
    #[deku(id = "3")]
    ThreeMinutes,
    #[deku(id = "4")]
    OneMinute,
    #[deku(id = "5")]
    ThirtySeconds,
    #[deku(id = "6")]
    FifteenSeconds,
    #[deku(id = "7")]
    TenSeconds,
    #[deku(id = "8")]
    FiveSeconds,
    #[deku(id = "9")]
    NextShorter,
    #[deku(id = "10")]
    NextLonger,
    #[deku(id = "11")]
    TwoSeconds,
    #[deku(id_pat = "_")]
    Other(u8),
}

/// Writes `n/a` for a value that is not available.
const NA: &str = "n/a";

impl Display for RepeatCount {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str(match self {
            RepeatCount::Original => "original",
            RepeatCount::Once => "repeated once",
            RepeatCount::Twice => "repeated twice",
            RepeatCount::Final => "repeated, final",
        })
    }
}

impl Display for AisVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str(match self {
            AisVersion::M1371_1 => "M.1371-1",
            AisVersion::M1371_3 => "M.1371-3",
            AisVersion::M1371_5 => "M.1371-5",
            AisVersion::M1371_6 => "M.1371-6",
        })
    }
}

impl Display for Manufacturer {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "manufacturer {} model {} serial {}", self.id, self.model, self.serial)
    }
}

impl Display for ImoNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self.0 {
            0 => write!(f, "IMO {NA}"),
            number => write!(f, "IMO {number}"),
        }
    }
}

impl Display for ShipType {
    /// What the ship is, and for a hazardous cargo its category, `tanker, category X`.
    #[expect(clippy::too_many_lines, reason = "a code table, one arm a code")]
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        use ShipType::{
            BulkCarrier, BuoyTender, CableLayer, Cargo, Cargo_OS, Cargo_Other, Cargo_X, Cargo_Y, Cargo_Z, Construction,
            ContainerShip, CrewBoat, CruiseShip, Diving, Dredger, Excursion, FPSO, Ferry, FishFactory, FishFarmSupport,
            Fishing, Government, HighSpeed, HighSpeed_OS, HighSpeed_Other, HighSpeed_Passenger, HighSpeed_RoRo,
            HighSpeed_X, HighSpeed_Y, HighSpeed_Z, IceBreaker, LandingCraft, LawEnforcement, LongTow, Medical,
            NotAtWar, NotAvailable, OffshoreSupport, Other, Other_NoMoreInformation, Other_OS, Other_X, Other_Y,
            Other_Z, Passenger, Passenger_OS, Passenger_Other, Passenger_X, Passenger_Y, Passenger_Z, Patrol, Pilot,
            PipeLayer, Pleasure, Research, Responder, RoRoCarrier, Sailing, SearchRescue, SpecialPurpose,
            SupportVessel, Tanker, Tanker_NonHazardous, Tanker_OS, Tanker_Other, Tanker_X, Tanker_Y, Tanker_Z, Tender,
            Towing, Training, Trawler, Tug, TugAndTank, Unknown, Warship, Wig_OS, Wig_Other, Wig_X, Wig_Y, Wig_Z,
            WingInGround,
        };
        let (kind, category) = match self {
            NotAvailable => ("type n/a", None),
            Research => ("research", None),
            Training => ("training", None),
            Government => ("government", None),
            IceBreaker => ("ice breaker", None),
            BuoyTender => ("buoy tender", None),
            CableLayer => ("cable layer", None),
            PipeLayer => ("pipe layer", None),
            SpecialPurpose => ("special purpose", None),
            FPSO => ("FPSO", None),
            FishFactory => ("fish factory", None),
            FishFarmSupport => ("fish farm support", None),
            OffshoreSupport => ("offshore support", None),
            Construction => ("construction", None),
            CrewBoat => ("crew boat", None),
            SupportVessel => ("support vessel", None),
            WingInGround => ("wing in ground", None),
            Wig_X => ("wing in ground", Some("X")),
            Wig_Y => ("wing in ground", Some("Y")),
            Wig_Z => ("wing in ground", Some("Z")),
            Wig_OS => ("wing in ground", Some("OS")),
            Wig_Other => ("wing in ground, other", None),
            Fishing => ("fishing", None),
            Towing => ("towing", None),
            LongTow => ("towing, long tow", None),
            Dredger => ("dredger", None),
            Diving => ("diving", None),
            Warship => ("military", None),
            Sailing => ("sailing", None),
            Pleasure => ("pleasure craft", None),
            Trawler => ("trawler", None),
            Patrol => ("patrol", None),
            HighSpeed => ("high speed craft", None),
            HighSpeed_X => ("high speed craft", Some("X")),
            HighSpeed_Y => ("high speed craft", Some("Y")),
            HighSpeed_Z => ("high speed craft", Some("Z")),
            HighSpeed_OS => ("high speed craft", Some("OS")),
            HighSpeed_Passenger => ("high speed craft, passenger", None),
            HighSpeed_RoRo => ("high speed craft, ro-ro", None),
            HighSpeed_Other => ("high speed craft, other", None),
            Pilot => ("pilot", None),
            SearchRescue => ("search and rescue", None),
            Tug => ("tug", None),
            Tender => ("port tender", None),
            Responder => ("anti-pollution", None),
            LawEnforcement => ("law enforcement", None),
            Medical => ("medical transport", None),
            NotAtWar => ("not party to conflict", None),
            Passenger => ("passenger", None),
            Passenger_X => ("passenger", Some("X")),
            Passenger_Y => ("passenger", Some("Y")),
            Passenger_Z => ("passenger", Some("Z")),
            Passenger_OS => ("passenger", Some("OS")),
            CruiseShip => ("cruise ship", None),
            Ferry => ("ferry", None),
            Excursion => ("excursion", None),
            Passenger_Other => ("passenger, other", None),
            Cargo => ("cargo", None),
            Cargo_X => ("cargo", Some("X")),
            Cargo_Y => ("cargo", Some("Y")),
            Cargo_Z => ("cargo", Some("Z")),
            Cargo_OS => ("cargo", Some("OS")),
            BulkCarrier => ("bulk carrier", None),
            ContainerShip => ("container ship", None),
            RoRoCarrier => ("ro-ro carrier", None),
            LandingCraft => ("landing craft", None),
            Cargo_Other => ("cargo, other", None),
            Tanker => ("tanker", None),
            Tanker_X => ("tanker", Some("X")),
            Tanker_Y => ("tanker", Some("Y")),
            Tanker_Z => ("tanker", Some("Z")),
            Tanker_OS => ("tanker", Some("OS")),
            Tanker_NonHazardous => ("tanker, not hazardous", None),
            TugAndTank => ("tug and tank barge", None),
            Tanker_Other => ("tanker, other", None),
            Other => ("other", None),
            Other_X => ("other", Some("X")),
            Other_Y => ("other", Some("Y")),
            Other_Z => ("other", Some("Z")),
            Other_OS => ("other", Some("OS")),
            Other_NoMoreInformation => ("other, no more information", None),
            Unknown(code) => return write!(f, "type {code}"),
        };
        f.write_str(kind)?;
        match category {
            Some(category) => write!(f, ", category {category}"),
            None => Ok(()),
        }
    }
}

impl Display for NavigationalStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        use NavigationalStatus::{
            ActiveSART, Aground, AtAnchor, ConstrainedByDraught, Fishing, Moored, NotUnderCommand, Pushing, Reserved9,
            Reserved10, Reserved13, RestrictedManoeuvrability, Sailing, TowingAstern, Undefined, UnderWay,
        };
        f.write_str(match self {
            UnderWay => "under way using engine",
            AtAnchor => "at anchor",
            NotUnderCommand => "not under command",
            RestrictedManoeuvrability => "restricted manoeuvrability",
            ConstrainedByDraught => "constrained by draught",
            Moored => "moored",
            Aground => "aground",
            Fishing => "engaged in fishing",
            Sailing => "under way sailing",
            Reserved9 => "status 9",
            Reserved10 => "status 10",
            TowingAstern => "towing astern",
            Pushing => "pushing ahead or towing alongside",
            Reserved13 => "status 13",
            ActiveSART => "AIS-SART active",
            Undefined => "status n/a",
        })
    }
}

impl Display for SatelliteTxControl {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str(match self {
            SatelliteTxControl::Stop => "stop",
            SatelliteTxControl::Send => "send",
        })
    }
}

impl Display for AssignedMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str(match self {
            AssignedMode::Autonomous => "autonomous",
            AssignedMode::Assigned => "assigned",
        })
    }
}

impl Display for AltitudeSensor {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str(match self {
            AltitudeSensor::GNSS => "GNSS",
            AltitudeSensor::Barometric => "barometric",
        })
    }
}

impl Display for CommunicationStateType {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str(match self {
            CommunicationStateType::SOTDMA => "SOTDMA",
            CommunicationStateType::ITDMA => "ITDMA",
        })
    }
}

impl Display for CommunicationState {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "sync {}, time-out {}, sub-message {}", self.sync, self.slot_time_out, self.sub_message)
    }
}

impl Display for DteReady {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str(match self {
            DteReady::Available => "DTE ready",
            DteReady::NotAvailable => "DTE not ready",
        })
    }
}

impl Display for OverallDimension {
    /// Length and beam, `15 m × 5 m`, from the distances to the reference point.
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let length = self.a + self.b;
        let beam = u16::from(self.c) + u16::from(self.d);
        match (length, beam) {
            (0, 0) => write!(f, "dimensions {NA}"),
            (0, beam) => write!(f, "length {NA} × {beam} m"),
            (length, 0) => write!(f, "{length} m × beam {NA}"),
            (length, beam) => write!(f, "{length} m × {beam} m"),
        }
    }
}

impl Display for PositioningDevice {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        use PositioningDevice::{
            BDS, Chayka, GLONASS, GNSS, GPS, Galileo, INS, Inertial, InternalGNSS, Loran, Manual, NotAvailable, PNT,
            Reserved10, Reserved11, Terrestrial,
        };
        f.write_str(match self {
            NotAvailable => "device n/a",
            GPS => "GPS",
            GLONASS => "GLONASS",
            GNSS => "GNSS",
            Loran => "Loran-C",
            Chayka => "Chayka",
            INS => "INS",
            Manual => "surveyed",
            Galileo => "Galileo",
            BDS => "BDS",
            Reserved10 => "device 10",
            Reserved11 => "device 11",
            PNT => "PNT",
            Inertial => "inertial",
            Terrestrial => "terrestrial",
            InternalGNSS => "internal GNSS",
        })
    }
}

impl Display for AtonType {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        use AtonType::{
            BeaconCardinalEast, BeaconCardinalNorth, BeaconCardinalSouth, BeaconCardinalWest, BeaconIsolatedDanger,
            BeaconPortHand, BeaconPreferredChannelPortHand, BeaconPreferredChannelStarboardHand, BeaconSafeWater,
            BeaconSpecialMark, BeaconStarboardHand, CardinalEast, CardinalNorth, CardinalSouth, CardinalWest,
            EmergencyWreckMarkingBuoy, FixedStructure, IsolatedDanger, LeadingLightFront, LeadingLightRear, Light,
            LightVessel, NotSpecified, PortHand, PreferredChannelPortHand, PreferredChannelStarboardHand, Racon,
            ReferencePoint, SafeWater, SectorLight, SpecialMark, StarboardHand,
        };
        f.write_str(match self {
            NotSpecified => "AtoN type n/a",
            ReferencePoint => "reference point",
            Racon => "RACON",
            FixedStructure => "fixed structure off shore",
            EmergencyWreckMarkingBuoy => "emergency wreck marking buoy",
            Light => "light",
            SectorLight => "sector light",
            LeadingLightFront => "leading light front",
            LeadingLightRear => "leading light rear",
            BeaconCardinalNorth => "beacon, cardinal N",
            BeaconCardinalEast => "beacon, cardinal E",
            BeaconCardinalSouth => "beacon, cardinal S",
            BeaconCardinalWest => "beacon, cardinal W",
            BeaconPortHand => "beacon, port hand",
            BeaconStarboardHand => "beacon, starboard hand",
            BeaconPreferredChannelPortHand => "beacon, preferred channel port hand",
            BeaconPreferredChannelStarboardHand => "beacon, preferred channel starboard hand",
            BeaconIsolatedDanger => "beacon, isolated danger",
            BeaconSafeWater => "beacon, safe water",
            BeaconSpecialMark => "beacon, special mark",
            CardinalNorth => "cardinal mark N",
            CardinalEast => "cardinal mark E",
            CardinalSouth => "cardinal mark S",
            CardinalWest => "cardinal mark W",
            PortHand => "port hand mark",
            StarboardHand => "starboard hand mark",
            PreferredChannelPortHand => "preferred channel port hand",
            PreferredChannelStarboardHand => "preferred channel starboard hand",
            IsolatedDanger => "isolated danger",
            SafeWater => "safe water",
            SpecialMark => "special mark",
            LightVessel => "light vessel, LANBY or rig",
        })
    }
}

impl Display for UtcDateTime {
    /// `2026-10-25 09:30:15 UTC`, the date or the time `n/a` when not available.
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        if self.year == 0 || !(1..=12).contains(&self.month) || !(1..=31).contains(&self.day) {
            write!(f, "date {NA}")?;
        } else {
            write!(f, "{:04}-{:02}-{:02}", self.year, self.month, self.day)?;
        }
        if self.hour > 23 || self.minute > 59 || self.second > 59 {
            write!(f, " time {NA}")
        } else {
            write!(f, " {:02}:{:02}:{:02} UTC", self.hour, self.minute, self.second)
        }
    }
}

impl Display for QuietTime {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self.0 {
            0 => f.write_str("no quiet time"),
            minutes => write!(f, "quiet {minutes} min"),
        }
    }
}

impl Display for Timestamp {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Timestamp::Second(second) => write!(f, "second {second}"),
            Timestamp::Unavailable => write!(f, "time stamp {NA}"),
            Timestamp::Manual => f.write_str("manual input"),
            Timestamp::Estimated => f.write_str("dead reckoning"),
            Timestamp::Inoperative => f.write_str("positioning inoperative"),
        }
    }
}

impl Display for RateOfTurn {
    /// In degrees a minute to starboard or port, from the indicator's square root scale.
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self.0 {
            -128 => write!(f, "rate of turn {NA}"),
            0 => f.write_str("not turning"),
            127 => f.write_str("turning starboard over 5°/30 s"),
            -127 => f.write_str("turning port over 5°/30 s"),
            indicator => {
                let rate = (f64::from(indicator) / 4.733).powi(2);
                let side = if indicator > 0 { "starboard" } else { "port" };
                write!(f, "turning {side} {rate:.0}°/min")
            }
        }
    }
}

impl Display for SpeedOverGround {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self.0 {
            1023 => write!(f, "SOG {NA}"),
            1022 => f.write_str("SOG ≥ 102.2 kn"),
            tenths => write!(f, "SOG {:.1} kn", f64::from(tenths) / 10.0),
        }
    }
}

impl Display for CourseOverGround {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self.0 {
            3600.. => write!(f, "COG {NA}"),
            tenths => write!(f, "COG {:.1}°", f64::from(tenths) / 10.0),
        }
    }
}

impl Display for Heading {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self.0 {
            360.. => write!(f, "heading {NA}"),
            degrees => write!(f, "heading {degrees}°"),
        }
    }
}

impl Display for Altitude {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self.0 {
            4095 => write!(f, "altitude {NA}"),
            4094 => f.write_str("altitude ≥ 4094 m"),
            metres => write!(f, "altitude {metres} m"),
        }
    }
}

impl Display for RaimFlag {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str(match self {
            RaimFlag::NotInUse => "RAIM not in use",
            RaimFlag::InUse => "RAIM in use",
        })
    }
}

impl Display for PositionAccuracy {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str(match self {
            PositionAccuracy::Low => "accuracy over 10 m",
            PositionAccuracy::High => "accuracy within 10 m",
        })
    }
}

impl Display for StationPosition {
    /// In degrees to the ten-thousandth of a minute it carries, six places.
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match <(f64, f64)>::try_from(*self) {
            Ok((latitude, longitude)) => LatLon { latitude, longitude, decimals: 6 }.fmt(f),
            Err(_) => write!(f, "position {NA}"),
        }
    }
}

impl GnssPosition {
    /// The latitude and longitude in degrees, from tenths of a minute; `None` when not available,
    /// latitude 91 or longitude 181 degrees.
    #[must_use]
    pub fn degrees(&self) -> Option<(f64, f64)> {
        if self.latitude.abs() > 90 * 600 || self.longitude.abs() > 180 * 600 {
            return None;
        }
        Some((f64::from(self.latitude) / 600.0, f64::from(self.longitude) / 600.0))
    }
}

impl Display for GnssPosition {
    /// In degrees to the tenth of a minute it carries, three places.
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self.degrees() {
            Some((latitude, longitude)) => LatLon { latitude, longitude, decimals: 3 }.fmt(f),
            None => write!(f, "position {NA}"),
        }
    }
}

impl Display for GnssArea {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} to {}", self.lower_left, self.upper_right)
    }
}

impl Display for SpecialManoeuvre {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str(match self {
            SpecialManoeuvre::NotAvailable => "special manoeuvre n/a",
            SpecialManoeuvre::NotEngaged => "no special manoeuvre",
            SpecialManoeuvre::Engaged => "special manoeuvre",
            SpecialManoeuvre::Regional => "special manoeuvre, regional",
        })
    }
}

impl Display for TransmitPower {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str(match self {
            TransmitPower::High => "high power",
            TransmitPower::Low => "low power",
        })
    }
}

impl Display for Sequence {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "#{}", self.0)
    }
}

impl Display for VdesCapability {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str(match self {
            VdesCapability::AisOnly => "AIS only",
            VdesCapability::Vdes => "VDES",
            VdesCapability::VdesTerrestrial => "VDES terrestrial",
            VdesCapability::VdesSatellite => "VDES satellite",
        })
    }
}

impl Display for PositionLatency {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str(match self {
            PositionLatency::Low => "low latency",
            PositionLatency::High => "high latency",
        })
    }
}

impl Display for StationSelection {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            StationSelection::AllMobile => f.write_str("all mobile stations"),
            StationSelection::OnlyClassA => f.write_str("Class A stations"),
            StationSelection::OnlyClassB => f.write_str("Class B stations"),
            StationSelection::OnlyAirborne => f.write_str("SAR aircraft"),
            StationSelection::OnlyClassB_SO => f.write_str("Class B SOTDMA stations"),
            StationSelection::OnlyClassB_CS => f.write_str("Class B CS stations"),
            StationSelection::AllWaterways => f.write_str("inland waterway stations"),
            StationSelection::AllInCoverageArea => f.write_str("all stations in coverage"),
            StationSelection::Other(code) => write!(f, "station type {code}"),
        }
    }
}

impl Display for TxRxMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str(match self {
            TxRxMode::TxAB_RxAB => "Tx A/B Rx A/B",
            TxRxMode::TxA_RxAB => "Tx A Rx A/B",
            TxRxMode::TxB_RxAB => "Tx B Rx A/B",
            TxRxMode::Reserved => "Tx/Rx mode 3",
        })
    }
}

impl Display for ReportingInterval {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            ReportingInterval::Autonomous => f.write_str("autonomous"),
            ReportingInterval::TenMinutes => f.write_str("every 10 min"),
            ReportingInterval::SixMinutes => f.write_str("every 6 min"),
            ReportingInterval::ThreeMinutes => f.write_str("every 3 min"),
            ReportingInterval::OneMinute => f.write_str("every 1 min"),
            ReportingInterval::ThirtySeconds => f.write_str("every 30 s"),
            ReportingInterval::FifteenSeconds => f.write_str("every 15 s"),
            ReportingInterval::TenSeconds => f.write_str("every 10 s"),
            ReportingInterval::FiveSeconds => f.write_str("every 5 s"),
            ReportingInterval::NextShorter => f.write_str("next shorter"),
            ReportingInterval::NextLonger => f.write_str("next longer"),
            ReportingInterval::TwoSeconds => f.write_str("every 2 s"),
            ReportingInterval::Other(code) => write!(f, "interval {code}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_mmsi_is_written_as_nine_digits() {
        let coast = MMSI::new(2_320_001).unwrap();
        assert_eq!(coast.to_string(), "002320001");
        assert_eq!("002320001".parse::<MMSI>().ok(), Some(coast));
        assert_eq!(MMSI::new(366_123_456).unwrap().to_string(), "366123456");
        assert!("2320001".parse::<MMSI>().is_err(), "nine digits");
        assert!("0023200010".parse::<MMSI>().is_err(), "not ten");
        assert!("+02320001".parse::<MMSI>().is_err());
    }
}
