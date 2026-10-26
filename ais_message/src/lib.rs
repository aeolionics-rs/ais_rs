//  SPDX-FileCopyrightText: 2026-2026. Aeolionics, LLC
//
//  SPDX-License-Identifier: Apache-2.0

//! A crate for working with AIS messages as defined by
//! [ITU-R M.1371-6 (02/2026)](https://www.itu.int/rec/recommendation.asp?lang=en&parent=R-REC-M.1371-6-202602-I).
//!
//! [Deku](https://crates.io/crates/deku) is used for (de-)serialization.
pub mod message;
pub mod types;

use deku::bitvec::{BitSlice, BitVec, Msb0};
use deku::prelude::*;
use message::*;
use std::fmt::{Binary, Display, Formatter, Write};

/// An AIS message.
#[non_exhaustive]
#[derive(Debug, DekuRead, DekuWrite)]
#[deku(id_type = "u8", bits = "6")]
pub enum Message {
    /// Message 1: Scheduled Position Report from a Class A AIS station.
    #[deku(id = 1)]
    ScheduledPositionReport(PositionReport),
    /// Message 2: Position Report from a Class A AIS station, sent on a assigned schedule.
    #[deku(id = 2)]
    AssignedPositionReport(PositionReport),
    /// Message 3: Special position Report from a Class A AIS station, sent in response to an interrogation.
    #[deku(id = 3)]
    SpecialPositionReport(PositionReport),
    /// Message 4: UTC, date and current slot number of base station.
    #[deku(id = 4)]
    BaseStationReport(BaseStationReport),
    /// Message 5: Scheduled static and voyage related vessel data report from a Class A AIS station.
    #[deku(id = 5)]
    StaticData(ShipStaticData),
    /// Message 6: An application-specific message address to a specific station.
    #[deku(id = 6)]
    AddressedBinaryMessage(AddressedApplicationMessage),
    /// Message 7: Acknowledgement of an application-specific message.
    #[deku(id = 7)]
    BinaryAcknowledgement(Acknowledgement),
    /// Message 8: An application-specific message broadcast to all stations.
    #[deku(id = 8)]
    BroadcastBinaryMessage(BroadcastApplicationMessage),
    /// Message 9: Position report for airborne stations involved in SAR operations.
    #[deku(id = 9)]
    SarPositionReport(AircraftPositionReport),
    /// Message 10: Request UTC and date.
    #[deku(id = 10)]
    UtcDateInquiry(UtcInquiry),
    /// Message 11: Current UTC and date.
    #[deku(id = 11)]
    UtcDateResponse(UtcResponse),
    /// Message 12: A safety-related message address to a specific station.
    #[deku(id = 12)]
    AddressedSafetyMessage(AddressedSafetyMessage),
    /// Message 13: Acknowledgement of aa safety-related message.
    #[deku(id = 13)]
    SafetyAcknowledgement(Acknowledgement),
    /// Message 14: A safety-related message broadcast to all stations.
    #[deku(id = 14)]
    BroadcastSafetyMessage(BroadcastSafetyMessage),
    /// Message 15: Request for a specific message type.
    #[deku(id = 15)]
    Interrogation(Interrogation),
    /// Message 16: Assignment of a specific report behaviour by competent authority using a Base station.
    #[deku(id = 16)]
    AssignmentModeCommand(AssignedModeCommand),
    /// Message 17: DGNSS corrections provided by a base station.
    #[deku(id = 17)]
    GnssBroadcastMessage(GnssBroadcast),
    /// Message 18: Standard Position Report from a Class B AIS station.
    #[deku(id = 18)]
    StandardClassBReport(StandardClassBReport),
    /// Message 19: Extended Position Report from a Class B AIS station (deprecated).
    #[deku(id = 19)]
    ExtendedClassBReport(ExtendedClassBReport),
    /// Message 20: Reserve slots for Base station(s).
    #[deku(id = 20)]
    DataLinkManagement(DataLinkManagement),
    /// Message 21: Position and status report for aids-to-navigation.
    #[deku(id = 21)]
    AtonReport(AtonReport),
    /// Message 22: Management of channels and transceiver modes by a Base station.
    #[deku(id = 22)]
    ChannelManagement(ChannelManagement),
    /// Message 23: Assignment of a specific report behaviour by competent authority using a Base
    /// station to a specific group of mobiles.
    #[deku(id = 23)]
    GroupAssignment(GroupAssignment),
    /// Message 24: Additional data assigned to an MMSI:
    /// - Part A: Name
    /// - Part B: Static Data
    #[deku(id = 24)]
    StaticDataReport(StaticData),
    /// Message 25: An application-specific message sent in a single slot.
    #[deku(id = 25)]
    SingleSlotBinary(SingleSlotBinary),
    /// Message 25: An application-specific message sent in multiple slots.
    #[deku(id = 26)]
    MultiSlotBinary(MultiSlotBinary),
    /// Message 27: A position report sent via satellite by stations outside base station coverage.
    #[deku(id = 27)]
    AisSatellite(AisSatellitePositionReport),
    /// Message 28: Position and status report for aids-to-navigation sent in a single slot.
    #[deku(id = 28)]
    SingleSlotAtonReport(SingleSlotAtonReport),
}

impl Message {
    /// Decode a message from its bits.
    ///
    /// Unlike [`DekuContainerRead::from_bytes`], the message need not be a whole number of bytes:
    /// a variable-length field at the end of the message, such as text or binary data, is read up
    /// to the last bit rather than to the end of the last byte. Any bits following a fixed-length
    /// message are ignored.
    pub fn from_bits(bits: &BitSlice<u8, Msb0>) -> Result<Self, DekuError> {
        // Whole bytes can be decoded in place.
        if let Some((None, bytes, None)) = bits.domain().region() {
            let (_, message) = Message::from_bytes((bytes, 0))?;
            return Ok(message);
        }

        // Decoding reads whole bytes, so pad before the message so that it ends on a byte boundary
        // and start decoding after the padding.
        let padding = (8 - bits.len() % 8) % 8;
        let mut padded = BitVec::<u8, Msb0>::repeat(false, padding);
        padded.extend_from_bitslice(bits);
        let (_, message) = Message::from_bytes((padded.as_raw_slice(), padding))?;
        Ok(message)
    }
}

impl Display for Message {
    /// What kind of message, then the message on one line.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Message::ScheduledPositionReport(m) => write!(f, "Position report: {m}"),
            Message::AssignedPositionReport(m) => write!(f, "Position report, assigned: {m}"),
            Message::SpecialPositionReport(m) => write!(f, "Position report, interrogated: {m}"),
            Message::BaseStationReport(m) => write!(f, "Base station report: {m}"),
            Message::StaticData(m) => write!(f, "Static and voyage data: {m}"),
            Message::AddressedBinaryMessage(m) => write!(f, "Addressed binary message: {m}"),
            Message::BinaryAcknowledgement(m) => write!(f, "Binary acknowledgement: {m}"),
            Message::BroadcastBinaryMessage(m) => write!(f, "Binary broadcast message: {m}"),
            Message::SarPositionReport(m) => write!(f, "SAR aircraft position report: {m}"),
            Message::UtcDateInquiry(m) => write!(f, "UTC and date inquiry: {m}"),
            Message::UtcDateResponse(m) => write!(f, "UTC and date response: {m}"),
            Message::AddressedSafetyMessage(m) => write!(f, "Addressed safety message: {m}"),
            Message::SafetyAcknowledgement(m) => write!(f, "Safety acknowledgement: {m}"),
            Message::BroadcastSafetyMessage(m) => write!(f, "Safety broadcast message: {m}"),
            Message::Interrogation(m) => write!(f, "Interrogation: {m}"),
            Message::AssignmentModeCommand(m) => write!(f, "Assigned mode command: {m}"),
            Message::GnssBroadcastMessage(m) => write!(f, "DGNSS broadcast: {m}"),
            Message::StandardClassBReport(m) => write!(f, "Class B position report: {m}"),
            Message::ExtendedClassBReport(m) => write!(f, "Class B extended position report: {m}"),
            Message::DataLinkManagement(m) => write!(f, "Data link management: {m}"),
            Message::AtonReport(m) => write!(f, "Aid to navigation report: {m}"),
            Message::ChannelManagement(m) => write!(f, "Channel management: {m}"),
            Message::GroupAssignment(m) => write!(f, "Group assignment: {m}"),
            Message::StaticDataReport(m) => write!(f, "Static data report: {m}"),
            Message::SingleSlotBinary(m) => write!(f, "Single slot binary message: {m}"),
            Message::MultiSlotBinary(m) => write!(f, "Multiple slot binary message: {m}"),
            Message::AisSatellite(m) => write!(f, "Long-range position report: {m}"),
            Message::SingleSlotAtonReport(m) => write!(f, "Single slot AtoN report: {m}"),
        }
    }
}

impl Binary for Message {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Ok(bits) = self.to_bits() {
            for bit in bits {
                f.write_char(if bit { '1' } else { '0' })?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use types::*;

    #[test]
    fn type1() {
        let msg = Message::ScheduledPositionReport(PositionReport::new(
            MMSI::new(0).unwrap(),
            CommunicationState { sync: 0b11, slot_time_out: 0b111, sub_message: 0x3fff },
        ));
        let result = msg.to_bits().unwrap();
        assert_eq!(result.len(), 168);
    }

    /// Decode an armored AIVDM payload, discarding any fill bits.
    fn decode_with_fill(payload: &str, fill_bits: usize) -> Message {
        let mut bits = deku::bitvec::BitVec::<u8, deku::bitvec::Msb0>::new();
        for ch in payload.bytes() {
            let value = match ch - 48 {
                value @ 0..40 => value,
                value => value - 8,
            };
            for i in (0..6).rev() {
                bits.push(value >> i & 1 == 1);
            }
        }
        bits.truncate(bits.len() - fill_bits);
        Message::from_bits(&bits).unwrap()
    }

    /// Decode an armored AIVDM payload.
    fn decode(payload: &str) -> Message {
        decode_with_fill(payload, 0)
    }

    /// Decode a single-sentence AIVDM/AIVDO sentence, discarding any fill bits.
    fn decode_sentence(sentence: &str) -> Message {
        let fields: Vec<&str> = sentence.split(',').collect();
        let payload = fields[5];
        let fill_bits = fields[6].split('*').next().unwrap().parse().unwrap();
        decode_with_fill(payload, fill_bits)
    }

    #[test]
    fn type21_aton() {
        let Message::AtonReport(report) =
            decode_sentence("!AIVDM,1,1,,A,E=?eK;hab0b4WW5RR`2a0000000C@OBPvT340000002P10,4*53")
        else {
            panic!()
        };
        assert_eq!(u32::from(report.source), 888888111);
        assert_eq!(report.aton_type, AtonType::ReferencePoint);
        assert_eq!(report.name.to_string(), "STATIONKEEPER");
        let (latitude, longitude): (f64, f64) = report.position.try_into().unwrap();
        assert_eq!(((latitude * 1e4).round(), (longitude * 1e4).round()), (-50218.0, 910902.0));
        let dimension = report.overall_dimension;
        assert_eq!((dimension.a, dimension.b, dimension.c, dimension.d), (0, 0, 0, 0));
        assert!(matches!(report.timestamp, Timestamp::Second(5)));
        assert!(!report.off_position);
        assert!(report.virtual_aton);
        assert_eq!(report.name_extension.len(), 0);
    }

    #[test]
    fn station_position_not_available() {
        assert!(<(f64, f64)>::try_from(StationPosition::default()).is_err());
        // SAFETY: test values only, never encoded.
        let no_longitude = unsafe { StationPosition::new_raw(0, 181 * 60 * 10_000, PositionAccuracy::Low) };
        assert!(<(f64, f64)>::try_from(no_longitude).is_err());
        let no_latitude = unsafe { StationPosition::new_raw(91 * 60 * 10_000, 0, PositionAccuracy::Low) };
        assert!(<(f64, f64)>::try_from(no_latitude).is_err());

        // The limits are in range.
        let position = StationPosition::new(-90.0, 180.0, PositionAccuracy::Low);
        assert_eq!(<(f64, f64)>::try_from(position).unwrap(), (-90.0, 180.0));
    }

    #[test]
    fn type4_year() {
        let Message::BaseStationReport(report) = decode("403OviQuMGCqWrRO9>E6fE700@GO") else { panic!() };
        let utc = report.utc_datetime;
        assert_eq!((utc.year, utc.month, utc.day), (2007, 5, 14));
        assert_eq!((utc.hour, utc.minute, utc.second), (19, 57, 39));
    }

    #[test]
    fn type18_position() {
        let Message::StandardClassBReport(report) = decode("B52K>;h00Fc>jpUlNV@ikwpUoP06") else { panic!() };
        assert_eq!(u32::from(report.source), 338087471);
        let (latitude, longitude): (f64, f64) = report.position.try_into().unwrap();
        assert_eq!(((latitude * 1e4).round(), (longitude * 1e4).round()), (406845.0, -740721.0));
        assert_eq!(report.heading.0, 511);
    }

    #[test]
    fn type19_position() {
        let Message::ExtendedClassBReport(report) = decode("C5N3SRgPEnJGEBT>NhWAwwo862PaLELTBJ:V00000000S0D:R220")
        else {
            panic!()
        };
        assert_eq!(u32::from(report.source), 367059850);
        let (latitude, longitude): (f64, f64) = report.position.try_into().unwrap();
        assert_eq!(((latitude * 1e4).round(), (longitude * 1e4).round()), (295437.0, -888104.0));
        assert_eq!(report.name.to_string(), "CAPT.J.RIMES");
    }

    #[test]
    fn type24_parts() {
        let Message::StaticDataReport(report) = decode("H42O55i18tMET00000000000000") else { panic!() };
        assert_eq!(u32::from(report.source), 271041815);
        let message::StaticDataPart::PartA(name) = report.data else { panic!("not part A") };
        assert_eq!(name.to_string(), "PROGUY");

        let Message::StaticDataReport(report) = decode("H42O55lti4hhhilD3nink000?050") else { panic!() };
        assert_eq!(u32::from(report.source), 271041815);
        let message::StaticDataPart::PartB { ship_type: ShipType::Passenger, call_sign, dimension, .. } = report.data
        else {
            panic!("not passenger part B")
        };
        assert_eq!(call_sign.to_string(), "TC6163");
        assert_eq!((dimension.a, dimension.b, dimension.c, dimension.d), (0, 15, 0, 5));
    }

    #[test]
    fn from_bits_text() {
        // 70 bits, so the text would be followed by 2 bits of padding if decoded from bytes.
        let msg = Message::BroadcastSafetyMessage(BroadcastSafetyMessage {
            repeat: RepeatCount::Original,
            source: MMSI::new(366999999).unwrap(),
            text: "HELLO".parse().unwrap(),
        });
        let bits = msg.to_bits().unwrap();
        assert_eq!(bits.len(), 70);
        let Message::BroadcastSafetyMessage(decoded) = Message::from_bits(&bits).unwrap() else { panic!() };
        assert_eq!(u32::from(decoded.source), 366999999);
        assert_eq!(decoded.text.to_string(), "HELLO");
    }

    #[test]
    fn from_bits_aton_name_extension() {
        // A type 21 AtoN report with a 1 character name extension is 278 bits.
        let Message::AtonReport(mut report) =
            decode_sentence("!AIVDM,1,1,,A,E=?eK;hab0b4WW5RR`2a0000000C@OBPvT340000002P10,4*53")
        else {
            panic!()
        };
        report.name_extension = "X".parse().unwrap();
        let mut bits = Message::AtonReport(report).to_bits().unwrap();
        assert_eq!(bits.len(), 278);
        let Message::AtonReport(decoded) = Message::from_bits(&bits).unwrap() else { panic!() };
        assert_eq!(decoded.name.to_string(), "STATIONKEEPER");
        assert_eq!(decoded.name_extension.to_string(), "X");

        // The spare bits that pad the name extension to a byte boundary are ignored.
        bits.extend([false, false]);
        let Message::AtonReport(decoded) = Message::from_bits(&bits).unwrap() else { panic!() };
        assert_eq!(decoded.name_extension.to_string(), "X");
    }

    #[test]
    fn from_bits_binary_data() {
        // A type 25 message with 12 bits of data is 52 bits.
        let data = deku::bitvec::bitvec![u8, deku::bitvec::Msb0; 1, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0, 1];
        let msg = Message::SingleSlotBinary(message::SingleSlotBinary {
            repeat: RepeatCount::Original,
            source: MMSI::new(123456789).unwrap(),
            addressed: false,
            structured: false,
            data: data.clone().into(),
        });
        let bits = msg.to_bits().unwrap();
        assert_eq!(bits.len(), 52);
        let Message::SingleSlotBinary(decoded) = Message::from_bits(&bits).unwrap() else { panic!() };
        assert_eq!(decoded.data.0, data);

        // A type 8 message for an unknown application with 7 bits of data is 63 bits.
        use ais_asm::{ApplicationId, Broadcast, DAC, Function};
        let data = &data[..7];
        let msg = Message::BroadcastBinaryMessage(message::BroadcastApplicationMessage {
            repeat: RepeatCount::Original,
            source: MMSI::new(123456789).unwrap(),
            message: Broadcast::Unknown(ApplicationId(DAC(1), Function(63)), data.to_bitvec().into()),
        });
        let bits = msg.to_bits().unwrap();
        assert_eq!(bits.len(), 63);
        let Message::BroadcastBinaryMessage(decoded) = Message::from_bits(&bits).unwrap() else { panic!() };
        let Broadcast::Unknown(_, decoded) = decoded.message else { panic!() };
        assert_eq!(*decoded, *data);
    }

    #[test]
    fn type27_satellite() {
        let Message::AisSatellite(report) = decode("K5DfMB9FLsM?P00d") else { panic!() };
        assert_eq!(u32::from(report.source), 357277000);
        assert_eq!(report.course_over_ground, 11);
    }

    #[test]
    fn from_bits_unaligned() {
        // Whole bytes that do not start on a byte boundary are decoded the same as aligned ones.
        let msg = Message::ScheduledPositionReport(PositionReport::new(
            MMSI::new(123456789).unwrap(),
            CommunicationState { sync: 0, slot_time_out: 0, sub_message: 0 },
        ));
        let mut bits = deku::bitvec::BitVec::<u8, deku::bitvec::Msb0>::repeat(true, 3);
        bits.extend_from_bitslice(&msg.to_bits().unwrap());
        assert_eq!(bits[3..].len(), 168);
        let Message::ScheduledPositionReport(decoded) = Message::from_bits(&bits[3..]).unwrap() else { panic!() };
        assert_eq!(u32::from(decoded.source), 123456789);
    }

    #[test]
    fn from_bits_trailing_bits() {
        // Bits after a fixed-length message are ignored.
        let msg = Message::ScheduledPositionReport(PositionReport::new(
            MMSI::new(123456789).unwrap(),
            CommunicationState { sync: 0, slot_time_out: 0, sub_message: 0 },
        ));
        let mut bits = msg.to_bits().unwrap();
        bits.extend([true, true]);
        let Message::ScheduledPositionReport(decoded) = Message::from_bits(&bits).unwrap() else { panic!() };
        assert_eq!(u32::from(decoded.source), 123456789);
    }

    /// Each message on one line: who sent it, then what matters of what it says.
    #[test]
    fn display() {
        let shown = |payload: &str, fill: usize| decode_with_fill(payload, fill).to_string();
        assert_eq!(
            shown("13u?etPv2;0n:dDPwUM1U1Cb069D", 0),
            "Position report: 265547250, under way using engine, 57.660353°N 11.832977°E, SOG 13.9 kn, COG 40.4°, \
             heading 41°"
        );
        assert_eq!(
            shown("403OviQuMGCqWrRO9>E6fE700@GO", 0),
            "Base station report: 003669702, 2007-05-14 19:57:39 UTC, 36.883767°N 76.352362°W"
        );
        assert_eq!(
            shown("55?MbV02;H;s<HtKR20EHE:0@T4@Dn2222222216L961O5Gf0NSQEp6ClRp888888888880", 2),
            "Static and voyage data: 351759000, \"EVER DIADEM\", 3FOF8, IMO 9134270, cargo, 295 m × 32 m, \
             draught 12.2 m, to \"NEW YORK\", ETA 05-15 14:00 UTC"
        );
        assert_eq!(
            shown("B52K>;h00Fc>jpUlNV@ikwpUoP06", 0),
            "Class B position report: 338087471, 40.684540°N 74.072132°W, SOG 0.1 kn, COG 79.6°"
        );
        assert_eq!(
            shown("C5N3SRgPEnJGEBT>NhWAwwo862PaLELTBJ:V00000000S0D:R220", 0),
            "Class B extended position report: 367059850, \"CAPT.J.RIMES\", cargo, 29.543695°N 88.810392°W, \
             SOG 8.7 kn, COG 335.9°"
        );
        assert_eq!(shown("H42O55i18tMET00000000000000", 0), "Static data report: 271041815, \"PROGUY\"");
        assert_eq!(
            shown("H42O55lti4hhhilD3nink000?050", 0),
            "Static data report: 271041815, passenger, TC6163, 15 m × 5 m"
        );
        assert_eq!(
            shown("E=?eK;hab0b4WW5RR`2a0000000C@OBPvT340000002P10", 4),
            "Aid to navigation report: 888888111, \"STATIONKEEPER\", reference point, 5.021813°S 91.090187°E, virtual"
        );
        assert_eq!(
            shown("K5DfMB9FLsM?P00d", 0),
            "Long-range position report: 357277000, moored, 37.653°S 176.182°E, SOG 0 kn, COG 11°"
        );
        assert_eq!(
            shown("91b55wi;hbOS@OdQAC062Ch2089h", 0),
            "SAR aircraft position report: 111232511, 58.144000°N 6.278843°W, altitude 303 m, SOG 42 kn, COG 154.5°"
        );

        // What isn't available is left out, but a position says so.
        let mmsi = |n| MMSI::new(n).unwrap();
        let state = CommunicationState { sync: 0, slot_time_out: 0, sub_message: 0 };
        assert_eq!(
            Message::ScheduledPositionReport(PositionReport::new(mmsi(235_000_001), state)).to_string(),
            "Position report: 235000001, position n/a"
        );
        let shown = |message: Message| message.to_string();
        assert_eq!(
            shown(Message::UtcDateInquiry(UtcInquiry {
                repeat: RepeatCount::Original,
                source: mmsi(235_000_001),
                destination: mmsi(2_320_001),
            })),
            "UTC and date inquiry: 235000001 asks 002320001 for UTC and date"
        );
        assert_eq!(
            shown(Message::BroadcastSafetyMessage(BroadcastSafetyMessage {
                repeat: RepeatCount::Original,
                source: mmsi(2_320_001),
                text: "GALE WARNING@@".parse().unwrap(),
            })),
            "Safety broadcast message: 002320001, \"GALE WARNING\""
        );
        assert_eq!(
            shown(Message::BinaryAcknowledgement(Acknowledgement {
                repeat: RepeatCount::Original,
                source: mmsi(235_000_001),
                destinations: vec![AcknowledgmentDestination { destination: mmsi(2_320_001), sequence: Sequence(2) }],
            })),
            "Binary acknowledgement: 235000001 acknowledges 002320001 #2"
        );
        assert_eq!(
            shown(Message::Interrogation(Interrogation {
                repeat: RepeatCount::Original,
                source: mmsi(2_320_001),
                destinations: vec![InterrogationDestination {
                    destination: mmsi(235_000_001),
                    message_id: 5,
                    slot_offset: 0,
                }],
            })),
            "Interrogation: 002320001 asks 235000001 for message 5"
        );
        use ais_asm::{Broadcast, Linkage};
        assert_eq!(
            shown(Message::BroadcastBinaryMessage(BroadcastApplicationMessage {
                repeat: RepeatCount::Original,
                source: mmsi(2_320_001),
                message: Broadcast::LinkedText {
                    version: 1,
                    linkage: Linkage(1),
                    text: "WHALES SIGHTED".parse().unwrap()
                },
            })),
            "Binary broadcast message: 002320001, Linked Text 1, \"WHALES SIGHTED\""
        );
        assert_eq!(
            shown(Message::SingleSlotBinary(message::SingleSlotBinary {
                repeat: RepeatCount::Original,
                source: mmsi(123_456_789),
                addressed: false,
                structured: true,
                data: vec![0u8; 4].into(),
            })),
            "Single slot binary message: 123456789, 32 bits, structured"
        );
    }

    #[test]
    fn field_types_display_themselves() {
        assert_eq!(ShipType::Tanker_Y.to_string(), "tanker, category Y");
        assert_eq!(ShipType::Unknown(25).to_string(), "type 25");
        assert_eq!(NavigationalStatus::Sailing.to_string(), "under way sailing");
        assert_eq!(SpeedOverGround(1022).to_string(), "SOG ≥ 102.2 kn");
        assert_eq!(Heading(511).to_string(), "heading n/a");
        assert_eq!(RateOfTurn(-128).to_string(), "rate of turn n/a");
        assert_eq!(RateOfTurn(20).to_string(), "turning starboard 18°/min");
        assert_eq!(StationPosition::default().to_string(), "position n/a");
        assert_eq!(OverallDimension { a: 0, b: 0, c: 0, d: 0 }.to_string(), "dimensions n/a");
        let utc = UtcDateTime { year: 0, month: 0, day: 0, hour: 9, minute: 30, second: 15 };
        assert_eq!(utc.to_string(), "date n/a 09:30:15 UTC");
        assert_eq!(UtcDateTime::default().to_string(), "date n/a time n/a");
        assert_eq!(Timestamp::Estimated.to_string(), "dead reckoning");
    }

    #[test]
    fn debug() {
        let debug = format!("{:?}", decode("H42O55i18tMET00000000000000"));
        assert!(debug.starts_with("StaticDataReport(StaticData {"), "{debug}");
        assert!(debug.contains("PartA(\"PROGUY@@@@@@@@@@@@@@\")"), "{debug}");
    }
}
