pub mod asms;
mod types;
pub mod six_bit_ascii;

use six_bit_ascii::{SixBitAscii, SixBitText};
use asms::ApplicationMessage;
use deku::prelude::*;
pub use types::*;

#[derive(DekuRead, DekuWrite)]
#[deku(id_type = "u8", bits = "6")]
pub enum Message {
    #[deku(id = 1)]
    ScheduledPositionReport(PositionReport),
    #[deku(id = 2)]
    AssignedPositionReport(PositionReport),
    #[deku(id = 3)]
    SpecialPositionReport(PositionReport),
    #[deku(id = 4)]
    BaseStationReport(BaseStationReport),
    #[deku(id = 5)]
    StaticData(ShipStaticData),
    #[deku(id = 6)]
    BinaryAddressedMessage(AddressedApplicationMessage),
    #[deku(id = 7)]
    BinaryAcknowledgement(Acknowledgement),
    #[deku(id = 8)]
    BinaryBroadcastMessage(BroadcastApplicationMessage),
    #[deku(id = 9)]
    SarPositionReport(AircraftPositionReport),
    #[deku(id = 10)]
    UtcDateInquiry(UtcInquiry),
    #[deku(id = 11)]
    UtcDateResponse(UtcResponse),
    #[deku(id = 12)]
    AddressedSafetyMessage(AddressedSafetyMessage),
    #[deku(id = 13)]
    SafetyAcknowledgement(Acknowledgement),
    #[deku(id = 14)]
    BroadcastSafetyMessage(BroadcastSafetyMessage),
    #[deku(id = 15)]
    Interrogation(Interrogation),
    #[deku(id = 16)]
    AssignmentModeCommand(AssignedModeCommand),
    #[deku(id = 17)]
    GnssBroadcastMessage(GnssBroadcast),
    #[deku(id = 18)]
    StandardClassBReport(StandardClassBReport),
    #[deku(id = 19)]
    ExtendedClassBReport(ExtendedClassBReport),
    #[deku(id = 20)]
    DataLinkManagement(DataLinkManagement),
    #[deku(id = 21)]
    AtonReport(AtonReport),
    #[deku(id = 22)]
    ChannelManagement(ChannelManagement),
    #[deku(id = 23)]
    GroupAssignment(GroupAssignment),
    #[deku(id = 24)]
    StaticDataReport(StaticData),
    #[deku(id = 25)]
    SingleSlotBinary(SingleSlotBinary),
    #[deku(id = 26)]
    MultiSlotBinary(MultiSlotBinary),
    #[deku(id = 27)]
    AisSatellite(AisSatellitePositionReport),
    #[deku(id = 28)]
    SingleSlotAtonReport(SingleSlotAtonReport),
}

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
    message: ApplicationMessage,
}

#[derive(DekuRead, DekuWrite)]
pub struct AcknowledgmentDestination{
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
    repeat_count: RepeatCount,
    #[deku(pad_bits_after = "2")]
    source: MMSI,
    message: ApplicationMessage,
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
    text: SixBitText<{936/6}>
}

#[derive(DekuRead, DekuWrite)]
pub struct BroadcastSafetyMessage {
    repeat: RepeatCount,
    #[deku(pad_bits_after = "2")]
    source: MMSI,
    text: SixBitText<{968/6}>
}

#[derive(DekuRead, DekuWrite)]
pub struct InterrogationDestination{
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
pub struct Assignment{
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
    data: Vec<u8> // TODO structure for DGNSS data
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
pub struct Reservation{
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
    PartB{
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
    name : Name,
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
    data: Vec<u8>,  // Includes CommunicationState as a trailer ...
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::asms::area_notice::*;
    use crate::asms::*;
    use chrono::Utc;
    use std::str::FromStr;

    #[test]
    fn type1() {
        let msg = Message::ScheduledPositionReport(PositionReport {
            repeat: RepeatCount::default(),
            source: MMSI::new(0).unwrap(),
            status: NavigationalStatus::default(),
            rate_of_turn: RateOfTurn::default(),
            speed_over_ground: SpeedOverGround::default(),
            position: StationPosition::default(),
            course_over_ground: CourseOverGround::default(),
            heading: Heading::default(),
            timestamp: Timestamp(0),
            manoeuvre: SpecialManoeuvre::default(),
            transmit_power: TransmitPower::default(),
            raim: RaimFlag::NotInUse,
            state: CommunicationState {
                sync: 0b11,
                slot_time_out: 0b111,
                sub_message: 0x3fff,
            },
        });
        let result = msg.to_bits().unwrap();
        assert_eq!(result.len(), 168);
        println!("{:b}", result);
    }

    #[test]
    fn geographic_notice() {
        let asm = GeographicNotice {
            version: Version(0),
            linkage: Linkage(0),
            description: Description::MarineMammalHabitat,
            start: Utc::now().into(),
            duration: Minutes(10),
            action: Action::Advisement,
            subareas: vec![SubArea::AssociatedText(Text::from_str("DEADBEEF").unwrap())],
        };
        let msg = Message::BinaryBroadcastMessage(BroadcastApplicationMessage {
            repeat_count: RepeatCount::Original,
            source: MMSI::new(999999999).unwrap(),
            message: ApplicationMessage::GeographicNotice_EU(asm)
        });
        let result = msg.to_bits().unwrap();
        assert_eq!(result.len(), 216);
        println!("{:b}", result);


    }
}
