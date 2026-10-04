//  SPDX-FileCopyrightText: 2026-2026. Aeolionics, LLC
//
//  SPDX-License-Identifier: Apache-2.0

//! Application-specific messages.
pub mod area_notice;
mod area_notice_v2;
mod geographic_notice;

use ais_sixbit::SixBitString;
use ais_types::BinaryData;
use area_notice::AreaNotice;
use area_notice_v2::AreaNoticeV2;
use deku::{DekuRead, DekuWrite};
use geographic_notice::GeographicNotice;

/// Designated Area Code, the controlling authority.
#[derive(PartialOrd, PartialEq, DekuRead, DekuWrite, Debug)]
pub struct DAC(#[deku(bits = 10, endian = "big")] pub u16);

/// Function indicator, indicate the message's functionality.
#[derive(PartialOrd, PartialEq, DekuRead, DekuWrite, Debug)]
pub struct Function(#[deku(bits = 6)] pub u8);

/// Identifier for an Application-Specific Message.
#[derive(PartialOrd, PartialEq, DekuRead, DekuWrite, Debug)]
pub struct ApplicationId(pub DAC, pub Function);

/// Common type for a message version.
#[derive(Debug, DekuRead, DekuWrite)]
pub struct Version(#[deku(bits = 3)] pub u8);

/// Identifier linking together a group of messages from the same source.
#[derive(Debug, DekuRead, DekuWrite)]
pub struct Linkage(#[deku(bits = 10, endian = "big")] pub u16);

#[derive(Debug, DekuRead, DekuWrite)]
pub struct Capability(#[deku(bits = "1", pad_bits_after = "1")] pub bool);

#[repr(u8)]
#[derive(Debug, DekuRead, DekuWrite)]
#[deku(bits = 3, id_type="u8")]
pub enum ApplicationResponse{
    UnableToResponse = 0,
    ReceptionAcknowledged = 1,
    ResponseToFollow = 2,
    Inhibited = 3,
    Reserved4 = 4,
    Reserved5 = 5,
    Reserved6 = 6,
    Reserved7 = 7,
}

#[allow(non_camel_case_types)]
#[derive(Debug, DekuRead, DekuWrite)]
#[deku(id_type = "ApplicationId")]
pub enum Addressed {
    #[deku(id = "ApplicationId(DAC(1), Function(0))")]
    Text {
        #[deku(bits = "1")]
        must_acknowledge: bool,
        #[deku(bits = 11, endian = "big")]
        sequence: u16,
        text: SixBitString,
        // TODO add padding to byte boundary
    },
    #[deku(id = "ApplicationId(DAC(1), Function(2))")]
    FunctionInterrogation {
        #[deku(pad_bits_after = "64")]
        request: ApplicationId,
    },
    #[deku(id = "ApplicationId(DAC(1), Function(3))")]
    CapabilityInterrogation {
        #[deku(pad_bits_after = "70")]
        request: DAC,
    },
    #[deku(id = "ApplicationId(DAC(1), Function(4))")]
    CapabilityReply { dac: DAC, functions: [Capability; 64] },
    #[deku(id = "ApplicationId(DAC(1), Function(5))")]
    ApplicationAcknowledgement{
        requested: ApplicationId,
        #[deku(bits = 11, endian = "big")]
        sequence: u16,
        is_available: bool,
        #[deku(pad_bits_after = "49")]
        response: ApplicationResponse,
    },
    #[deku(id = "ApplicationId(DAC(1), Function(23))")]
    AreaNotice(AreaNotice),
    #[deku(id = "ApplicationId(DAC(200), Function(42))")]
    GeographicNotice(GeographicNotice),
    #[deku(id_pat = "_")]
    Unknown(ApplicationId, BinaryData),
}

#[allow(non_camel_case_types)]
#[derive(Debug, DekuRead, DekuWrite)]
#[deku(id_type = "ApplicationId")]
pub enum Broadcast {
    #[deku(id = "ApplicationId(DAC(1), Function(0))")]
    Text {
        // must_acknowledge must always be zero for broadcast so treat as padding
        #[deku(bits = 11, endian = "big", pad_bits_before = "1")]
        sequence: u16,
        text: SixBitString,
        // TODO add padding to byte boundary
    },
    #[deku(id = "ApplicationId(DAC(1), Function(22))")]
    AreaNotice(AreaNotice),
    #[deku(id = "ApplicationId(DAC(366), Function(22))")]
    AreaNoticeV2(AreaNoticeV2),
    #[deku(id = "ApplicationId(DAC(367), Function(22))")]
    GeographicNotice(GeographicNotice),
    #[deku(id = "ApplicationId(DAC(367), Function(29))")]
    LinkedText {
        #[deku(bits = 6)]
        version: u8,
        linkage: Linkage,
        text: SixBitString,
        // TODO add padding to make total message length an even number of bytes
    },
    #[deku(id_pat = "_")]
    Unknown(ApplicationId, BinaryData),
}
