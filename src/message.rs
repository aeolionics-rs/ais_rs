//  SPDX-FileCopyrightText: 2026-2026. Aeolionics, LLC
//
//  SPDX-License-Identifier: Apache-2.0

//! Message definitions.

use deku::{DekuRead, DekuWrite};
use crate::asm;
use crate::types::sixbit::{SixBitAscii, SixBitText};
use crate::types::{AisVersion, Altitude, AltitudeSensor, AssignedMode, CallSign, CommunicationState, CommunicationStateType, CourseOverGround, DteReady, GnssArea, GnssPosition, Heading, ImoNumber, Manufacturer, MonthDayHourMinute, Name, NavigationalStatus, OverallDimension, PositionAccuracy, PositionLatency, PositioningDevice, QuietTime, RaimFlag, RateOfTurn, RepeatCount, ReportingInterval, SatelliteTxControl, Sequence, ShipType, SpecialManoeuvre, SpeedOverGround, StationPosition, StationSelection, Timestamp, TransmitPower, TxRxMode, UtcDateTime, VdesCapability, MMSI};

#[derive(DekuRead, DekuWrite)]
pub struct PositionReport {
    pub repeat: RepeatCount,
    pub source: MMSI,
    pub status: NavigationalStatus,
    pub rate_of_turn: RateOfTurn,
    pub speed_over_ground: SpeedOverGround,
    pub position: StationPosition,
    pub course_over_ground: CourseOverGround,
    pub heading: Heading,
    pub timestamp: Timestamp,
    pub manoeuvre: SpecialManoeuvre,
    #[deku(pad_bits_before = "2")]
    pub transmit_power: TransmitPower,
    pub raim: RaimFlag,
    pub state: CommunicationState,
}

#[derive(DekuRead, DekuWrite)]
pub struct AircraftPositionReport {
    repeat: RepeatCount,
    source: MMSI,
    altitude: Altitude,
    speed_over_ground: SpeedOverGround,
    position: StationPosition,
    course_over_ground: CourseOverGround,
    timestamp: Timestamp,
    #[deku(pad_bits_after = "7")]
    sensor: AltitudeSensor,
    #[deku(pad_bits_after = "3")]
    dte_ready: DteReady,
    assigned_mode: AssignedMode,
    raim: RaimFlag,
    state_flag: CommunicationStateType,
    state: CommunicationState,
}

#[derive(DekuRead, DekuWrite)]
pub struct BaseStationReport {
    repeat: RepeatCount,
    source: MMSI,
    utc_datetime: UtcDateTime,
    position: StationPosition,
    positioning_device: PositioningDevice,
    satellite_control: SatelliteTxControl,
    #[deku(pad_bits_before = "9")]
    raim: RaimFlag,
    state: CommunicationState,
}

#[derive(DekuRead, DekuWrite)]
pub struct ShipStaticData {
    repeat: RepeatCount,
    source: MMSI,
    ais_version: AisVersion,
    imo_number: ImoNumber,
    call_sign: CallSign,
    name: Name,
    type_of_ship: ShipType,
    dimension: OverallDimension,
    positioning_device: PositioningDevice,
    eta: MonthDayHourMinute,
    max_draught: u8,
    destination: Name,
    #[deku(pad_bits_after = "1")]
    dte: DteReady,
}

#[derive(DekuRead, DekuWrite)]
pub struct AddressedApplicationMessage {
    repeat_count: RepeatCount,
    source: MMSI,
    #[deku(bits = 2)]
    sequence: u8,
    destination: MMSI,
    #[deku(bits = 1, pad_bits_after = "1")]
    retransmitted: bool,
    message: asm::AddressedApplicationMessage,
}

#[derive(DekuRead, DekuWrite)]
pub struct AcknowledgmentDestination {
    destination: MMSI,
    sequence: Sequence,
}

#[derive(DekuRead, DekuWrite)]
pub struct Acknowledgement {
    repeat: RepeatCount,
    #[deku(pad_bits_after = "2")]
    source: MMSI,
    #[deku(read_all)]
    destinations: Vec<AcknowledgmentDestination>,
}

#[derive(DekuRead, DekuWrite)]
pub struct BroadcastApplicationMessage {
    pub repeat_count: RepeatCount,
    #[deku(pad_bits_after = "2")]
    pub source: MMSI,
    pub message: asm::BroadcastApplicationMessage,
}

#[derive(DekuRead, DekuWrite)]
pub struct UtcInquiry {
    repeat: RepeatCount,
    #[deku(pad_bits_after = "2")]
    source: MMSI,
    #[deku(pad_bits_after = "2")]
    destination: MMSI,
}

pub type UtcResponse = BaseStationReport;

#[derive(DekuRead, DekuWrite)]
pub struct AddressedSafetyMessage {
    repeat: RepeatCount,
    #[deku(pad_bits_after = "2")]
    source: MMSI,
    sequence: Sequence,
    #[deku(pad_bits_after = "2")]
    destination: MMSI,
    #[deku(bits = 1, pad_bits_after = "1")]
    retransmit: bool,
    text: SixBitText<{ 936 / 6 }>,
}

#[derive(DekuRead, DekuWrite)]
pub struct BroadcastSafetyMessage {
    repeat: RepeatCount,
    #[deku(pad_bits_after = "2")]
    source: MMSI,
    text: SixBitText<{ 968 / 6 }>,
}

#[derive(DekuRead, DekuWrite)]
pub struct InterrogationDestination {
    destination: MMSI,
    #[deku(bits = 6)]
    message_id: u8,
    #[deku(bits = 12, endian = "big", pad_bits_after = "2")]
    slot_offset: u16,
}

#[derive(DekuRead, DekuWrite)]
pub struct Interrogation {
    repeat: RepeatCount,
    #[deku(pad_bits_after = "2")]
    source: MMSI,
    #[deku(read_all)]
    destinations: Vec<InterrogationDestination>,
}

#[derive(DekuRead, DekuWrite)]
pub struct Assignment {
    destination: MMSI,
    #[deku(bits = 12, endian = "big")]
    offset: u16,
    #[deku(bits = 10, endian = "big")]
    increment: u16,
}

#[derive(DekuRead, DekuWrite)]
pub struct AssignedModeCommand {
    repeat: RepeatCount,
    #[deku(pad_bits_after = "2")]
    source: MMSI,
    a: Assignment,
    b: Option<Assignment>, // TODO may need to deal with padding if b is None
}

#[derive(DekuRead, DekuWrite)]
pub struct GnssBroadcast {
    repeat: RepeatCount,
    #[deku(pad_bits_after = "2")]
    source: MMSI,
    #[deku(pad_bits_after = "5")]
    position: GnssPosition,
    #[deku(read_all)]
    data: Vec<u8>, // TODO structure for DGNSS data
}

#[derive(DekuRead, DekuWrite)]
pub struct StandardClassBReport {
    repeat: RepeatCount,
    #[deku(pad_bits_after = "2")]
    source: MMSI,
    speed_over_ground: SpeedOverGround,
    position: StationPosition,
    course_over_ground: CourseOverGround,
    heading: Heading,
    timestamp: Timestamp,
    #[deku(pad_bits_after = "1")]
    transmit_power: TransmitPower,
    #[deku(bits = 1)]
    unit_type: u8,
    #[deku(bits = 1)]
    has_display: u8,
    #[deku(bits = 1)]
    has_dsc: u8,
    #[deku(bits = 1)]
    whole_band: u8,
    #[deku(bits = 1)]
    frequency_management: u8,
    assigned_mode: AssignedMode,
    raim: RaimFlag,
    state_flag: CommunicationStateType,
    state: CommunicationState, // TODO this is always a constant for ClassB-CS
}

#[derive(DekuRead, DekuWrite)]
pub struct ExtendedClassBReport {
    repeat: RepeatCount,
    #[deku(pad_bits_after = "2")]
    source: MMSI,
    speed_over_ground: SpeedOverGround,
    position: StationPosition,
    course_over_ground: CourseOverGround,
    heading: Heading,
    timestamp: Timestamp,
    #[deku(pad_bits_before = "4")]
    name: Name,
    ship_type: ShipType,
    overall_dimension: OverallDimension,
    positioning_device: PositioningDevice,
    raim: RaimFlag,
    dte_ready: DteReady,
    #[deku(pad_bits_after = "4")]
    assigned_mode: AssignedMode,
}

#[derive(DekuRead, DekuWrite)]
pub struct Reservation {
    #[deku(bits = 12, endian = "big")]
    offset: u16,
    #[deku(bits = 4)]
    count: u8,
    #[deku(bits = 3)]
    timeout: u8,
    #[deku(bits = 11, endian = "big")]
    increment: u16,
}

#[derive(DekuRead, DekuWrite)]
pub struct DataLinkManagement {
    repeat: RepeatCount,
    #[deku(pad_bits_after = "2")]
    source: MMSI,
    #[deku(read_all)]
    reservations: Vec<Reservation>,
}

#[derive(DekuRead, DekuWrite)]
pub struct AtonReport {
    repeat: RepeatCount,
    #[deku(pad_bits_after = "2")]
    source: MMSI,
    #[deku(bits = 5)]
    aton_type: u8,
    name: Name,
    position: StationPosition,
    overall_dimension: OverallDimension,
    positioning_device: PositioningDevice,
    timestamp: Timestamp,
    #[deku(bits = 1)]
    off_position: bool,
    status: u8,
    raim: RaimFlag,
    #[deku(bits = 1)]
    virtual_aton: bool,
    #[deku(pad_bits_after = "1")]
    assigned_mode: AssignedMode,
    #[deku(read_all)]
    name_extension: Vec<SixBitAscii>, // TODO may need padding
}

#[derive(DekuRead, DekuWrite)]
pub struct ChannelManagement {
    repeat: RepeatCount,
    #[deku(pad_bits_after = "2")]
    source: MMSI,
    #[deku(bits = 12, endian = "big")]
    channel_a: u16,
    #[deku(bits = 12, endian = "big")]
    channel_b: u16,
    #[deku(bits = 4)]
    txrx_mode: u8,
    power: TransmitPower,
    affects: GnssArea,
    #[deku(bits = 1)]
    addressed: bool,
    #[deku(bits = 1)]
    bandwidth_a: u8,
    #[deku(bits = 1)]
    bandwidth_b: u8,
    #[deku(bits = 3, pad_bits_after = "23")]
    transitional_zone: u8,
}

#[derive(DekuRead, DekuWrite)]
pub struct GroupSelection {
    area: GnssArea,
    stations: StationSelection,
    ships: ShipType,
}

#[derive(DekuRead, DekuWrite)]
pub struct GroupAssignment {
    repeat: RepeatCount,
    #[deku(pad_bits_after = "2")]
    source: MMSI,
    #[deku(pad_bits_after = "22")]
    affects: GroupSelection,
    txrx_mode: TxRxMode,
    interval: ReportingInterval,
    #[deku(pad_bits_after = "6")]
    quiet_time: QuietTime,
}

#[derive(DekuRead, DekuWrite)]
#[deku(id_type = "u8", bits = "2")]
pub enum StaticDataPart {
    #[deku(id = "0")]
    PartA(Name),
    #[deku(id = "1")]
    PartB {
        ship_type: ShipType,
        manufacturer: Manufacturer,
        call_sign: CallSign,
        dimension: OverallDimension,
        positioning_device: PositioningDevice,
        vdes: VdesCapability,
    },
}

#[derive(DekuRead, DekuWrite)]
pub struct StaticData {
    repeat: RepeatCount,
    #[deku(pad_bits_after = "2")]
    source: MMSI,
    name: Name,
}

#[derive(DekuRead, DekuWrite)]
pub struct SingleSlotBinary {
    repeat: RepeatCount,
    source: MMSI,
    #[deku(bits = 1)]
    addressed: bool,
    #[deku(bits = 1)]
    structured: bool,
    #[deku(read_all)]
    data: Vec<u8>,
}

#[derive(DekuRead, DekuWrite)]
pub struct MultiSlotBinary {
    repeat: RepeatCount,
    source: MMSI,
    #[deku(bits = 1)]
    addressed: bool,
    #[deku(bits = 1)]
    structured: bool,
    #[deku(read_all)]
    data: Vec<u8>, // Includes CommunicationState as a trailer ...
}

#[derive(DekuRead, DekuWrite)]
pub struct AisSatellitePositionReport {
    repeat: RepeatCount,
    source: MMSI,
    position_accuracy: PositionAccuracy,
    raim_flag: RaimFlag,
    navigational_status: NavigationalStatus,
    position: GnssPosition,
    #[deku(bits = 6)]
    speed_over_ground: u8,
    #[deku(bits = 9)]
    course_over_ground: u16,
    #[deku(pad_bits_after = "1")]
    position_latency: PositionLatency,
}

#[derive(DekuRead, DekuWrite)]
pub struct SingleSlotAtonReport {
    repeat: RepeatCount,
    source: MMSI,
    timestamp: Timestamp,
    #[deku(bits = 28, endian = "big")]
    longitude: i32,
    #[deku(bits = 27, endian = "big")]
    latitude: i32,
    #[deku(bits = 2)]
    restricted_use: u8,
    #[deku(bits = 3)]
    station_type: u8,
    #[deku(bits = 7)]
    aton_type: u8,
    #[deku(bits = 17, endian = "big")]
    mrn: u32,
    #[deku(bits = 4)]
    dimensions_type: u8,
    #[deku(bits = 9, endian = "big")]
    dimensions_a: u16,
    #[deku(bits = 11, endian = "big")]
    dimensions_b: u16,
    #[deku(bits = 1)]
    dimensions_ext: u8,
    #[deku(bits = 1)]
    charted: bool,
    #[deku(bits = 4)]
    on_station: u8,
    #[deku(bits = 8)]
    status: u8,
    #[deku(bits = 1, pad_bits_before = "1")]
    authenticated: bool,
}