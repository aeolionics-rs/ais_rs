//  SPDX-FileCopyrightText: 2026-2026. Aeolionics, LLC
//
//  SPDX-License-Identifier: Apache-2.0

//! Common types.

use ais_types::OutOfRange;
use chrono::{DateTime, Datelike, Timelike, Utc};
use deku::{DekuRead, DekuWrite, deku_derive};
use ais_sixbit::SixBitText;

/// Indicates how many times a message has been repeated.
#[repr(u8)]
#[deku_derive(DekuRead, DekuWrite)]
#[deku(id_type = "u8", bits = "2")]
pub enum RepeatCount {
    /// Original message (default).
    Original = 0,
    /// First retransmission.
    Once = 1,
    /// Second retransmission.
    Twice = 2,
    /// Third and final repeat; do not repeat any more.
    Final = 3,
}
impl Default for RepeatCount {
    fn default() -> Self {
        Self::Original
    }
}

/// Maritime Mobile Service Identity.
#[deku_derive(DekuRead, DekuWrite)]
pub struct MMSI(#[deku(bits = "30", endian = "big")] u32);
impl MMSI {
    /// Create a new MMS identifier.
    ///
    /// Returns `OutOfRange` if the value is not a valid MMSI.
    pub fn new(mmsi: u32) -> Result<Self, OutOfRange> {
        match mmsi {
            ..=999_999_999 => Ok(MMSI(mmsi)),
            _ => Err(OutOfRange),
        }
    }
}

/// Version of the AIS standard being used.
#[repr(u8)]
#[non_exhaustive]
#[derive(DekuRead, DekuWrite)]
#[deku(id_type = "u8", bits = "2")]
pub enum AisVersion {
    M1371_1 = 0,
    M1371_3 = 1,
    M1371_5 = 2,
    M1371_6 = 3,
}

/// Information about the equipment and its manufacturer.
#[derive(DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct Manufacturer {
    /// Manufacturer code assigned by NMEA/SAE.
    #[deku(bits = 18)]
    id: u32,
    /// Equipment model identifier.
    #[deku(bits = 4)]
    model: u32,
    /// Equipment serial number.
    #[deku(bits = 20)]
    serial: u32,
}

/// A vessel's IMO number.
#[deku_derive(DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct ImoNumber(#[deku(bits = "30")] pub u32);

/// The type of ship.
#[allow(non_camel_case_types)]
#[derive(DekuRead, DekuWrite)]
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
#[derive(DekuRead, DekuWrite)]
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
    Undefined = 15,
}
impl Default for NavigationalStatus {
    fn default() -> Self {
        NavigationalStatus::Undefined
    }
}

#[repr(u8)]
#[derive(DekuRead, DekuWrite)]
#[deku(id_type = "u8", bits = "1")]
pub enum SatelliteTxControl {
    Stop = 0,
    Send = 1,
}

#[repr(u8)]
#[derive(DekuRead, DekuWrite)]
#[deku(id_type = "u8", bits = "1")]
pub enum AssignedMode {
    Autonomous = 0,
    Assigned = 1,
}

#[allow(non_camel_case_types)]
#[repr(u8)]
#[derive(DekuRead, DekuWrite)]
#[deku(id_type = "u8", bits = "1")]
pub enum AltitudeSensor {
    GNSS = 0,
    Barometric = 1,
}

#[allow(non_camel_case_types)]
#[repr(u8)]
#[derive(DekuRead, DekuWrite)]
#[deku(id_type = "u8", bits = "1")]
pub enum CommunicationStateType {
    SOTDMA = 0,
    ITDMA = 1,
}

#[deku_derive(DekuRead, DekuWrite)]
pub struct CommunicationState {
    #[deku(bits = 2)]
    pub sync: u8,
    #[deku(bits = 3)]
    pub slot_time_out: u8,
    #[deku(bits = 14, endian = "big")]
    pub sub_message: u16,
}

#[repr(u8)]
#[derive(DekuRead, DekuWrite)]
#[deku(id_type = "u8", bits = "1")]
pub enum DteReady {
    Available = 0,
    NotAvailable = 1,
}

#[derive(DekuRead, DekuWrite)]
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
#[derive(DekuRead, DekuWrite)]
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

#[derive(DekuRead, DekuWrite)]
pub struct UtcDateTime {
    #[deku(bits = 14)]
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
        Self {
            year: 0,
            month: 0,
            day: 0,
            hour: 24,
            minute: 60,
            second: 60,
        }
    }
}

#[derive(DekuRead, DekuWrite)]
pub struct QuietTime(#[deku(bits = 4)] pub u32);

#[derive(DekuRead, DekuWrite)]
#[deku(id_type = "u8", bits = "6")]
pub enum Timestamp {
    #[deku(id_pat = "0..60")]
    Second(u8),
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
    pub fn now() -> Self {
        // SAFETY: second() returns number in range 0..59
        Self::Second(Utc::now().second() as u8)
    }
}

impl Default for Timestamp {
    fn default() -> Self {
        Self::Unavailable
    }
}

pub type Name = SixBitText<20>;

pub type CallSign = SixBitText<7>;
#[derive(DekuRead, DekuWrite)]
pub struct RateOfTurn(#[deku(bits = 8)] pub i8);

impl Default for RateOfTurn {
    fn default() -> Self {
        Self(-128)
    }
}

#[derive(DekuRead, DekuWrite)]
pub struct SpeedOverGround(#[deku(bits = 10, endian = "big")] pub u16);

impl Default for SpeedOverGround {
    fn default() -> Self {
        SpeedOverGround(1023)
    }
}

#[derive(DekuRead, DekuWrite)]
pub struct CourseOverGround(#[deku(bits = 12, endian = "big")] pub u16);

impl Default for CourseOverGround {
    fn default() -> Self {
        CourseOverGround(3600)
    }
}

#[derive(DekuRead, DekuWrite)]
pub struct Heading(#[deku(bits = 9, endian = "big")] pub u16);

impl Default for Heading {
    fn default() -> Self {
        Self(511)
    }
}

#[derive(DekuRead, DekuWrite)]
pub struct Altitude(#[deku(bits = 12, endian = "big")] pub u16);

/// Receiver autonomous integrity monitoring (RAIM) indicator.
///
/// See M-1371 Table 48.
#[repr(u8)]
#[derive(DekuRead, DekuWrite)]
#[deku(id_type = "u8", bits = "1")]
pub enum RaimFlag {
    /// RAIM is not in use.
    NotInUse = 0,
    /// RAIM is in use.
    InUse = 1,
}
impl Default for RaimFlag {
    fn default() -> Self {
        RaimFlag::NotInUse
    }
}

/// Position accuracy based on M-1371 Table 48.
#[repr(u8)]
#[derive(DekuRead, DekuWrite)]
#[deku(id_type = "u8", bits = "1")]
pub enum PositionAccuracy {
    /// Accuracy > 10m (default)
    Low = 0,
    /// Accuracy < 10m
    High = 1,
}

impl Default for PositionAccuracy {
    fn default() -> Self {
        PositionAccuracy::Low
    }
}

/// Station position, stored at a resolution of 1/10_000 minute.
#[derive(DekuRead, DekuWrite)]
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
    /// Latitude and longitude are signed values with a resolution of 1/10_000 minute.
    /// SAFETY: caller must ensure values are within range
    pub unsafe fn new_raw(latitude: i32, longitude: i32, accuracy: PositionAccuracy) -> Self {
        Self { latitude, longitude, accuracy }
    }

    /// Construct a position from latitude and longitude (degrees).
    ///
    /// Function asserts values are in range.
    pub fn new(latitude: f64, longitude: f64, accuracy: PositionAccuracy) -> Self {
        assert!(latitude >= -90.0 && latitude <= 90.0);
        assert!(longitude >= -180.0 && longitude <= 180.0);
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
        Ok(Self::new(latitude, longitude, Default::default()))
    }
}

impl Default for StationPosition {
    fn default() -> Self {
        Self {
            accuracy: PositionAccuracy::default(),
            longitude: 0x6791AC0, // Constant from spec = 181 * 60 * 10_000
            latitude: 0x3412140,  // Constant from spec = 91 * 60 * 10_000
        }
    }
}

#[derive(DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct GnssPosition {
    #[deku(bits = 18)]
    pub longitude: i32,
    #[deku(bits = 17)]
    pub latitude: i32,
}

#[derive(DekuRead, DekuWrite)]
pub struct GnssArea {
    upper_right: GnssPosition,
    lower_left: GnssPosition,
}

/// Indicates if the vessel is performing a special manoeuvre.
#[repr(u8)]
#[derive(DekuRead, DekuWrite)]
#[deku(id_type = "u8", bits = "2")]
pub enum SpecialManoeuvre {
    /// Information is not available (default).
    NotAvailable = 0,
    /// Vessel is not engaged in a special manoeuvre.
    NotEngaged = 1,
    /// Vessel is engaged in a special manoeuvre (e.g. a passing arrangement)
    Engaged = 2,
    /// Reserved for regional use.
    Regional = 3,
}

impl Default for SpecialManoeuvre {
    fn default() -> Self {
        SpecialManoeuvre::NotAvailable
    }
}

/// Transmitter output power.
#[repr(u8)]
#[derive(DekuRead, DekuWrite)]
#[deku(id_type = "u8", bits = "1")]
pub enum TransmitPower {
    /// Transmitting in high power mode (default).
    High = 0,
    /// Transmitting in low power mode.
    Low = 1,
}

impl Default for TransmitPower {
    fn default() -> Self {
        TransmitPower::High
    }
}

#[derive(DekuRead, DekuWrite)]
pub struct Sequence(#[deku(bits = 2)] pub u8);

#[repr(u8)]
#[derive(DekuRead, DekuWrite)]
#[deku(id_type = "u8", bits = "2")]
pub enum VdesCapability {
    AisOnly = 0,
    Vdes = 1,
    VdesTerrestrial = 2,
    VdesSatellite = 3,
}

#[repr(u8)]
#[derive(DekuRead, DekuWrite)]
#[deku(id_type = "u8", bits = "1")]
pub enum PositionLatency {
    Low = 0,
    High = 1,
}

#[allow(non_camel_case_types)]
#[derive(DekuRead, DekuWrite)]
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
#[derive(DekuRead, DekuWrite)]
#[deku(id_type = "u8", bits = "2")]
pub enum TxRxMode {
    TxAB_RxAB = 0,
    TxA_RxAB = 1,
    TxB_RxAB = 2,
    Reserved = 3,
}

#[derive(DekuRead, DekuWrite)]
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

