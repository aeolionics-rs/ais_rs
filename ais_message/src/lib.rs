//  SPDX-FileCopyrightText: 2026-2026. Aeolionics, LLC
//
//  SPDX-License-Identifier: Apache-2.0

//! A crate for working with AIS messages as defined by
//! [ITU-R M.1371-6 (02/2026)](https://www.itu.int/rec/recommendation.asp?lang=en&parent=R-REC-M.1371-6-202602-I).
//!
//! [Deku](https://crates.io/crates/deku) is used for (de-)serialization.
pub mod message;
pub mod types;

use std::fmt::{Binary, Formatter, Write};
use deku::prelude::*;
use message::*;

/// An AIS message.
#[non_exhaustive]
#[derive(DekuRead, DekuWrite)]
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
            CommunicationState {
                sync: 0b11,
                slot_time_out: 0b111,
                sub_message: 0x3fff,
            },
        ));
        let result = msg.to_bits().unwrap();
        assert_eq!(result.len(), 168);
    }

    /// Decode an armored AIVDM payload.
    fn decode(payload: &str) -> Message {
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
        bits.set_uninitialized(false);
        Message::from_bytes((bits.as_raw_slice(), 0)).unwrap().1
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
        let (latitude, longitude) = report.position.into();
        assert_eq!(((latitude * 1e4).round(), (longitude * 1e4).round()), (406845.0, -740721.0));
        assert_eq!(report.heading.0, 511);
    }

    #[test]
    fn type19_position() {
        let Message::ExtendedClassBReport(report) = decode("C5N3SRgPEnJGEBT>NhWAwwo862PaLELTBJ:V00000000S0D:R220") else { panic!() };
        assert_eq!(u32::from(report.source), 367059850);
        let (latitude, longitude) = report.position.into();
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
        let message::StaticDataPart::PartB { ship_type: ShipType::Passenger, call_sign, dimension, .. } = report.data else { panic!("not passenger part B") };
        assert_eq!(call_sign.to_string(), "TC6163");
        assert_eq!((dimension.a, dimension.b, dimension.c, dimension.d), (0, 15, 0, 5));
    }
}
