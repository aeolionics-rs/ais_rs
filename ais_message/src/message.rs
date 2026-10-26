//  SPDX-FileCopyrightText: 2026-2026. Aeolionics, LLC
//
//  SPDX-License-Identifier: Apache-2.0

//! AIS message definitions.

use crate::types::{
    AisVersion, Altitude, AltitudeSensor, AssignedMode, AtonType, CallSign, CommunicationState, CommunicationStateType,
    CourseOverGround, DteReady, GnssArea, GnssPosition, Heading, ImoNumber, MMSI, Manufacturer, Name,
    NavigationalStatus, OverallDimension, PositionAccuracy, PositionLatency, PositioningDevice, QuietTime, RaimFlag,
    RateOfTurn, RepeatCount, ReportingInterval, SatelliteTxControl, Sequence, ShipType, SpecialManoeuvre,
    SpeedOverGround, StationPosition, StationSelection, Timestamp, TransmitPower, TxRxMode, UtcDateTime,
    VdesCapability,
};
use ais_asm::{Addressed, Broadcast};
use ais_sixbit::SixBitString;
use ais_types::{BinaryData, LatLon, MonthDayHourMinute};
use core::fmt::{self, Display, Formatter};
use deku::{DekuRead, DekuWrite};

/// Message 1,2,3: a position report sent by a mobile, Class A station.
///
/// Messages 1, 2 and 3 all use the same data structure.
/// The message type is used to indicate the transmission window that was used
/// (autonomous, assigned, or by request, respectively).
#[derive(Debug, DekuRead, DekuWrite)]
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

impl PositionReport {
    /// Create a new position report with minimal information.
    /// All other fields are set to the default values defined by the specification.
    pub fn new(source: MMSI, state: CommunicationState) -> Self {
        PositionReport {
            repeat: Default::default(),
            source,
            status: Default::default(),
            rate_of_turn: Default::default(),
            speed_over_ground: Default::default(),
            position: Default::default(),
            course_over_ground: Default::default(),
            heading: Default::default(),
            timestamp: Default::default(),
            manoeuvre: Default::default(),
            transmit_power: Default::default(),
            raim: Default::default(),
            state,
        }
    }
}

/// Message 9: Position report for airborne stations involved in SAR operations.
#[derive(Debug, DekuRead, DekuWrite)]
pub struct AircraftPositionReport {
    pub repeat: RepeatCount,
    pub source: MMSI,
    pub altitude: Altitude,
    pub speed_over_ground: SpeedOverGround,
    pub position: StationPosition,
    pub course_over_ground: CourseOverGround,
    pub timestamp: Timestamp,
    #[deku(pad_bits_after = "7")]
    pub sensor: AltitudeSensor,
    #[deku(pad_bits_after = "3")]
    pub dte_ready: DteReady,
    pub assigned_mode: AssignedMode,
    pub raim: RaimFlag,
    pub state_flag: CommunicationStateType,
    pub state: CommunicationState,
}

/// Message 4: UTC, date and current slot number of base station.

#[derive(Debug, DekuRead, DekuWrite)]
pub struct BaseStationReport {
    pub repeat: RepeatCount,
    pub source: MMSI,
    pub utc_datetime: UtcDateTime,
    pub position: StationPosition,
    pub positioning_device: PositioningDevice,
    pub satellite_control: SatelliteTxControl,
    #[deku(pad_bits_before = "9")]
    pub raim: RaimFlag,
    pub state: CommunicationState,
}

/// Message 5: Scheduled static and voyage related vessel data report from a Class A AIS station.
#[derive(Debug, DekuRead, DekuWrite)]
pub struct ShipStaticData {
    pub repeat: RepeatCount,
    pub source: MMSI,
    pub ais_version: AisVersion,
    pub imo_number: ImoNumber,
    pub call_sign: CallSign,
    pub name: Name,
    pub type_of_ship: ShipType,
    pub dimension: OverallDimension,
    pub positioning_device: PositioningDevice,
    pub eta: MonthDayHourMinute,
    pub max_draught: u8,
    pub destination: Name,
    #[deku(pad_bits_after = "1")]
    pub dte: DteReady,
}

/// Message 6: An application-specific message address to a specific station.

#[derive(Debug, DekuRead, DekuWrite)]
pub struct AddressedApplicationMessage {
    pub repeat: RepeatCount,
    pub source: MMSI,
    #[deku(bits = 2)]
    pub sequence: u8,
    pub destination: MMSI,
    #[deku(bits = 1, pad_bits_after = "1")]
    pub retransmitted: bool,
    pub message: Addressed,
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct AcknowledgmentDestination {
    pub destination: MMSI,
    pub sequence: Sequence,
}

/// Message 7, 13: Acknowledgement of an application-specific or safety message.
#[derive(Debug, DekuRead, DekuWrite)]
pub struct Acknowledgement {
    pub repeat: RepeatCount,
    #[deku(pad_bits_after = "2")]
    pub source: MMSI,
    #[deku(read_all)]
    pub destinations: Vec<AcknowledgmentDestination>,
}

/// Message 8: An application-specific message broadcast to all stations.
#[derive(Debug, DekuRead, DekuWrite)]
pub struct BroadcastApplicationMessage {
    pub repeat: RepeatCount,
    #[deku(pad_bits_after = "2")]
    pub source: MMSI,
    pub message: Broadcast,
}

/// Message 10: Request UTC and date.
#[derive(Debug, DekuRead, DekuWrite)]

pub struct UtcInquiry {
    pub repeat: RepeatCount,
    #[deku(pad_bits_after = "2")]
    pub source: MMSI,
    #[deku(pad_bits_after = "2")]
    pub destination: MMSI,
}

/// Message 11: Current UTC and date.
///
/// This uses the same structure as Message 4.
pub type UtcResponse = BaseStationReport;

/// Message 12: A safety-related message address to a specific station.
#[derive(Debug, DekuRead, DekuWrite)]
pub struct AddressedSafetyMessage {
    pub repeat: RepeatCount,
    #[deku(pad_bits_after = "2")]
    pub source: MMSI,
    pub sequence: Sequence,
    #[deku(pad_bits_after = "2")]
    pub destination: MMSI,
    #[deku(bits = 1, pad_bits_after = "1")]
    pub retransmit: bool,
    pub text: SixBitString,
}

#[derive(Debug, DekuRead, DekuWrite)]
/// Message 14: A safety-related message broadcast to all stations.
pub struct BroadcastSafetyMessage {
    pub repeat: RepeatCount,
    #[deku(pad_bits_after = "2")]
    pub source: MMSI,
    pub text: SixBitString,
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct InterrogationDestination {
    pub destination: MMSI,
    #[deku(bits = 6)]
    pub message_id: u8,
    #[deku(bits = 12, endian = "big", pad_bits_after = "2")]
    pub slot_offset: u16,
}

/// Message 15: Request for a specific message type.
#[derive(Debug, DekuRead, DekuWrite)]
pub struct Interrogation {
    pub repeat: RepeatCount,
    #[deku(pad_bits_after = "2")]
    pub source: MMSI,
    #[deku(read_all)]
    pub destinations: Vec<InterrogationDestination>,
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct Assignment {
    pub destination: MMSI,
    #[deku(bits = 12, endian = "big")]
    pub offset: u16,
    #[deku(bits = 10, endian = "big")]
    pub increment: u16,
}

#[derive(Debug, DekuRead, DekuWrite)]
/// Message 16: Assignment of a specific report behaviour by competent authority using a Base station.
pub struct AssignedModeCommand {
    pub repeat: RepeatCount,
    #[deku(pad_bits_after = "2")]
    pub source: MMSI,
    pub a: Assignment,
    pub b: Option<Assignment>, // TODO may need to deal with padding if b is None
}

/// Message 17: DGNSS corrections provided by a base station.
#[derive(Debug, DekuRead, DekuWrite)]
pub struct GnssBroadcast {
    pub repeat: RepeatCount,
    #[deku(pad_bits_after = "2")]
    pub source: MMSI,
    #[deku(pad_bits_after = "5")]
    pub position: GnssPosition,
    pub data: BinaryData, // TODO structure for DGNSS data
}

/// Message 18: Standard Position Report from a Class B AIS station.
#[derive(Debug, DekuRead, DekuWrite)]
pub struct StandardClassBReport {
    pub repeat: RepeatCount,
    #[deku(pad_bits_after = "8")]
    pub source: MMSI,
    pub speed_over_ground: SpeedOverGround,
    pub position: StationPosition,
    pub course_over_ground: CourseOverGround,
    pub heading: Heading,
    pub timestamp: Timestamp,
    #[deku(pad_bits_after = "1")]
    pub transmit_power: TransmitPower,
    #[deku(bits = 1)]
    pub unit_type: u8,
    #[deku(bits = 1)]
    pub has_display: u8,
    #[deku(bits = 1)]
    pub has_dsc: u8,
    #[deku(bits = 1)]
    pub whole_band: u8,
    #[deku(bits = 1)]
    pub frequency_management: u8,
    pub assigned_mode: AssignedMode,
    pub raim: RaimFlag,
    pub state_flag: CommunicationStateType,
    pub state: CommunicationState, // TODO this is always a constant for ClassB-CS
}

/// Message 19: Extended Position Report from a Class B AIS station (deprecated).
#[derive(Debug, DekuRead, DekuWrite)]
pub struct ExtendedClassBReport {
    pub repeat: RepeatCount,
    #[deku(pad_bits_after = "8")]
    pub source: MMSI,
    pub speed_over_ground: SpeedOverGround,
    pub position: StationPosition,
    pub course_over_ground: CourseOverGround,
    pub heading: Heading,
    pub timestamp: Timestamp,
    #[deku(pad_bits_before = "4")]
    pub name: Name,
    pub ship_type: ShipType,
    pub overall_dimension: OverallDimension,
    pub positioning_device: PositioningDevice,
    pub raim: RaimFlag,
    pub dte_ready: DteReady,
    #[deku(pad_bits_after = "4")]
    pub assigned_mode: AssignedMode,
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct Reservation {
    #[deku(bits = 12, endian = "big")]
    pub offset: u16,
    #[deku(bits = 4)]
    pub count: u8,
    #[deku(bits = 3)]
    pub timeout: u8,
    #[deku(bits = 11, endian = "big")]
    pub increment: u16,
}

/// Message 20: Reserve slots for Base station(s).
#[derive(Debug, DekuRead, DekuWrite)]
pub struct DataLinkManagement {
    pub repeat: RepeatCount,
    #[deku(pad_bits_after = "2")]
    pub source: MMSI,
    #[deku(read_all)]
    pub reservations: Vec<Reservation>,
}

/// Message 21: Position and status report for aids-to-navigation.
#[derive(Debug, DekuRead, DekuWrite)]
pub struct AtonReport {
    pub repeat: RepeatCount,
    pub source: MMSI,
    pub aton_type: AtonType,
    pub name: Name,
    pub position: StationPosition,
    pub overall_dimension: OverallDimension,
    pub positioning_device: PositioningDevice,
    pub timestamp: Timestamp,
    #[deku(bits = 1)]
    pub off_position: bool,
    pub status: u8,
    pub raim: RaimFlag,
    #[deku(bits = 1)]
    pub virtual_aton: bool,
    #[deku(pad_bits_after = "1")]
    pub assigned_mode: AssignedMode,
    pub name_extension: SixBitString, // TODO may need padding
}

/// Message 22: Management of channels and transceiver modes by a Base station.
#[derive(Debug, DekuRead, DekuWrite)]
pub struct ChannelManagement {
    pub repeat: RepeatCount,
    #[deku(pad_bits_after = "2")]
    pub source: MMSI,
    #[deku(bits = 12, endian = "big")]
    pub channel_a: u16,
    #[deku(bits = 12, endian = "big")]
    pub channel_b: u16,
    #[deku(bits = 4)]
    pub txrx_mode: u8,
    pub power: TransmitPower,
    pub affects: GnssArea,
    #[deku(bits = 1)]
    pub addressed: bool,
    #[deku(bits = 1)]
    pub bandwidth_a: u8,
    #[deku(bits = 1)]
    pub bandwidth_b: u8,
    #[deku(bits = 3, pad_bits_after = "23")]
    pub transitional_zone: u8,
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct GroupSelection {
    pub area: GnssArea,
    pub stations: StationSelection,
    pub ships: ShipType,
}

/// Message 23: Assignment of a specific report behaviour by competent authority using a Base
/// station to a specific group of mobiles.
#[derive(Debug, DekuRead, DekuWrite)]
pub struct GroupAssignment {
    pub repeat: RepeatCount,
    #[deku(pad_bits_after = "2")]
    pub source: MMSI,
    #[deku(pad_bits_after = "22")]
    pub affects: GroupSelection,
    pub txrx_mode: TxRxMode,
    pub interval: ReportingInterval,
    #[deku(pad_bits_after = "6")]
    pub quiet_time: QuietTime,
}

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(id_type = "u8", bits = "2")]
pub enum StaticDataPart {
    /// - Part A: Vessel Name
    #[deku(id = "0")]
    PartA(Name),
    /// - Part B: Vessel Static Data
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

/// Message 24: Additional data assigned to an MMSI:
#[derive(Debug, DekuRead, DekuWrite)]
pub struct StaticData {
    pub repeat: RepeatCount,
    pub source: MMSI,
    pub data: StaticDataPart,
}

/// Message 25: An application-specific message sent in a single slot.
#[derive(Debug, DekuRead, DekuWrite)]
pub struct SingleSlotBinary {
    pub repeat: RepeatCount,
    pub source: MMSI,
    #[deku(bits = 1)]
    pub addressed: bool,
    #[deku(bits = 1)]
    pub structured: bool,
    pub data: BinaryData,
}

/// Message 26: An application-specific message sent in multiple slots.
#[derive(Debug, DekuRead, DekuWrite)]
pub struct MultiSlotBinary {
    pub repeat: RepeatCount,
    pub source: MMSI,
    #[deku(bits = 1)]
    pub addressed: bool,
    #[deku(bits = 1)]
    pub structured: bool,
    pub data: BinaryData, // Includes CommunicationState as a trailer ...
}

/// Message 27: A position report sent via satellite by stations outside base station coverage.
#[derive(Debug, DekuRead, DekuWrite)]
pub struct AisSatellitePositionReport {
    pub repeat: RepeatCount,
    pub source: MMSI,
    pub position_accuracy: PositionAccuracy,
    pub raim_flag: RaimFlag,
    pub navigational_status: NavigationalStatus,
    pub position: GnssPosition,
    #[deku(bits = 6)]
    pub speed_over_ground: u8,
    #[deku(bits = 9, endian = "big")]
    pub course_over_ground: u16,
    #[deku(pad_bits_after = "1")]
    pub position_latency: PositionLatency,
}

/// Message 28: Position and status report for aids-to-navigation sent in a single slot.
#[derive(Debug, DekuRead, DekuWrite)]
pub struct SingleSlotAtonReport {
    pub repeat: RepeatCount,
    pub source: MMSI,
    pub timestamp: Timestamp,
    #[deku(bits = 28, endian = "big")]
    pub longitude: i32,
    #[deku(bits = 27, endian = "big")]
    pub latitude: i32,
    #[deku(bits = 2)]
    pub restricted_use: u8,
    #[deku(bits = 3)]
    pub station_type: u8,
    #[deku(bits = 7)]
    pub aton_type: u8,
    #[deku(bits = 17, endian = "big")]
    pub mrn: u32,
    #[deku(bits = 4)]
    pub dimensions_type: u8,
    #[deku(bits = 9, endian = "big")]
    pub dimensions_a: u16,
    #[deku(bits = 11, endian = "big")]
    pub dimensions_b: u16,
    #[deku(bits = 1)]
    pub dimensions_ext: u8,
    #[deku(bits = 1)]
    pub charted: bool,
    #[deku(bits = 4)]
    pub on_station: u8,
    #[deku(bits = 8)]
    pub status: u8,
    #[deku(bits = 1, pad_bits_before = "1")]
    pub authenticated: bool,
}

// Each message displays as one line: the station that sent it, then what matters of what it says;
// what is not available is left out, and the radio's own fields are for `Debug`.

/// Text as it reads, without the '@' and spaces that pad it.
fn text(text: &impl Display) -> String {
    text.to_string().trim_end_matches(['@', ' ']).to_owned()
}

/// Writes `, {what}` when `available`.
fn and(f: &mut Formatter<'_>, available: bool, what: &impl Display) -> fmt::Result {
    if available { write!(f, ", {what}") } else { Ok(()) }
}

/// The motion a report gives, each part only when available.
fn motion(f: &mut Formatter<'_>, sog: &SpeedOverGround, cog: &CourseOverGround, heading: &Heading) -> fmt::Result {
    and(f, sog.0 != 1023, sog)?;
    and(f, cog.0 < 3600, cog)?;
    and(f, heading.0 < 360, heading)
}

impl Display for PositionReport {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.source)?;
        and(f, !matches!(self.status, NavigationalStatus::Undefined), &self.status)?;
        write!(f, ", {}", self.position)?;
        motion(f, &self.speed_over_ground, &self.course_over_ground, &self.heading)
    }
}

impl Display for AircraftPositionReport {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}", self.source, self.position)?;
        and(f, self.altitude.0 != 4095, &self.altitude)?;
        // An aircraft's speed is in whole knots.
        match self.speed_over_ground.0 {
            1023 => {}
            1022 => f.write_str(", SOG ≥ 1022 kn")?,
            knots => write!(f, ", SOG {knots} kn")?,
        }
        and(f, self.course_over_ground.0 < 3600, &self.course_over_ground)
    }
}

impl Display for BaseStationReport {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}, {}", self.source, self.utc_datetime, self.position)
    }
}

impl Display for ShipStaticData {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.source)?;
        let name = text(&self.name);
        and(f, !name.is_empty(), &format_args!("{name:?}"))?;
        let call_sign = text(&self.call_sign);
        and(f, !call_sign.is_empty(), &call_sign)?;
        and(f, self.imo_number.0 != 0, &self.imo_number)?;
        write!(f, ", {}", self.type_of_ship)?;
        and(f, self.dimension.a + self.dimension.b != 0, &self.dimension)?;
        and(f, self.max_draught != 0, &format_args!("draught {:.1} m", f64::from(self.max_draught) / 10.0))?;
        let destination = text(&self.destination);
        and(f, !destination.is_empty(), &format_args!("to {destination:?}"))?;
        and(f, self.eta.month != 0, &format_args!("ETA {}", self.eta))
    }
}

impl Display for AddressedApplicationMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} to {}, {}", self.source, self.destination, self.message)
    }
}

impl Display for Acknowledgement {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} acknowledges", self.source)?;
        for (i, acknowledged) in self.destinations.iter().enumerate() {
            let separator = if i == 0 { " " } else { ", " };
            write!(f, "{separator}{} {}", acknowledged.destination, acknowledged.sequence)?;
        }
        Ok(())
    }
}

impl Display for BroadcastApplicationMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}", self.source, self.message)
    }
}

impl Display for UtcInquiry {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} asks {} for UTC and date", self.source, self.destination)
    }
}

impl Display for AddressedSafetyMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} to {}, {:?}", self.source, self.destination, text(&self.text))
    }
}

impl Display for BroadcastSafetyMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {:?}", self.source, text(&self.text))
    }
}

impl Display for Interrogation {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} asks", self.source)?;
        for (i, asked) in self.destinations.iter().enumerate() {
            let separator = if i == 0 { " " } else { ", " };
            write!(f, "{separator}{} for message {}", asked.destination, asked.message_id)?;
        }
        Ok(())
    }
}

impl Display for Assignment {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} offset {} increment {}", self.destination, self.offset, self.increment)
    }
}

impl Display for AssignedModeCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} assigns {}", self.source, self.a)?;
        match &self.b {
            Some(b) => write!(f, ", {b}"),
            None => Ok(()),
        }
    }
}

impl Display for GnssBroadcast {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}, DGNSS corrections from {}, {}", self.source, self.position, self.data)
    }
}

impl Display for StandardClassBReport {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}", self.source, self.position)?;
        motion(f, &self.speed_over_ground, &self.course_over_ground, &self.heading)
    }
}

impl Display for ExtendedClassBReport {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.source)?;
        let name = text(&self.name);
        and(f, !name.is_empty(), &format_args!("{name:?}"))?;
        write!(f, ", {}, {}", self.ship_type, self.position)?;
        motion(f, &self.speed_over_ground, &self.course_over_ground, &self.heading)
    }
}

impl Display for Reservation {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} slots at offset {} every {}", self.count, self.offset, self.increment)
    }
}

impl Display for DataLinkManagement {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} reserves", self.source)?;
        for (i, reservation) in self.reservations.iter().enumerate() {
            let separator = if i == 0 { " " } else { ", " };
            write!(f, "{separator}{reservation}")?;
        }
        Ok(())
    }
}

impl Display for AtonReport {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let name = format!("{}{}", self.name, self.name_extension);
        write!(f, "{}, {:?}, {}, {}", self.source, text(&name), self.aton_type, self.position)?;
        and(f, self.virtual_aton, &"virtual")?;
        and(f, self.off_position, &"off position")
    }
}

impl Display for ChannelManagement {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}, channels {} and {}, {}", self.source, self.channel_a, self.channel_b, self.power)?;
        // Addressed, the area's bits name the two stations instead.
        if self.addressed { f.write_str(", addressed") } else { write!(f, ", {}", self.affects) }
    }
}

impl Display for GroupSelection {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.stations)?;
        and(f, !matches!(self.ships, ShipType::NotAvailable), &self.ships)?;
        write!(f, " in {}", self.area)
    }
}

impl Display for GroupAssignment {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} assigns {}: report {}, {}", self.source, self.affects, self.interval, self.txrx_mode)?;
        and(f, self.quiet_time.0 != 0, &self.quiet_time)
    }
}

impl Display for StaticData {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.source)?;
        match &self.data {
            StaticDataPart::PartA(name) => write!(f, ", {:?}", text(name)),
            StaticDataPart::PartB { ship_type, call_sign, dimension, .. } => {
                write!(f, ", {ship_type}")?;
                let call_sign = text(call_sign);
                and(f, !call_sign.is_empty(), &call_sign)?;
                and(f, dimension.a + dimension.b != 0, dimension)
            }
        }
    }
}

impl Display for SingleSlotBinary {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}", self.source, self.data)?;
        and(f, self.addressed, &"addressed")?;
        and(f, self.structured, &"structured")
    }
}

impl Display for MultiSlotBinary {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}", self.source, self.data)?;
        and(f, self.addressed, &"addressed")?;
        and(f, self.structured, &"structured")
    }
}

impl Display for AisSatellitePositionReport {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.source)?;
        and(f, !matches!(self.navigational_status, NavigationalStatus::Undefined), &self.navigational_status)?;
        write!(f, ", {}", self.position)?;
        // Whole knots and degrees, 63 and 511 not available.
        and(f, self.speed_over_ground != 63, &format_args!("SOG {} kn", self.speed_over_ground))?;
        and(f, self.course_over_ground != 511, &format_args!("COG {}°", self.course_over_ground))
    }
}

impl Display for SingleSlotAtonReport {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}, AtoN type {}", self.source, self.aton_type)?;
        // A ten-thousandth of a minute, as a station's position.
        if self.latitude.abs() > 90 * 600_000 || self.longitude.abs() > 180 * 600_000 {
            f.write_str(", position n/a")
        } else {
            let (latitude, longitude) = (f64::from(self.latitude) / 600_000.0, f64::from(self.longitude) / 600_000.0);
            write!(f, ", {}", LatLon { latitude, longitude, decimals: 6 })
        }
    }
}
