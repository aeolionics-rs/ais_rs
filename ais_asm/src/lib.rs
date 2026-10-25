//  SPDX-FileCopyrightText: 2026-2026. Aeolionics, LLC
//
//  SPDX-License-Identifier: Apache-2.0

//! Application-specific messages.
//!
//! The messages are those registered in the IALA ASM collection kept by the Netherlands at
//! <https://www.e-navigation.nl/asm>, which links each one's specification; each type here names
//! its entry.
pub mod area_notice;
pub mod area_notice_v2;
pub mod geographic_notice;
pub mod uscg_geographic_notice;

use ais_sixbit::SixBitString;
use ais_types::BinaryData;
use area_notice::AreaNotice;
use area_notice_v2::AreaNoticeV2;
use deku::{DekuRead, DekuWrite};
use geographic_notice::GeographicNotice;
use uscg_geographic_notice::GeographicNotice as UscgGeographicNotice;

/// Designated Area Code, the controlling authority.
#[derive(PartialOrd, PartialEq, DekuRead, DekuWrite, Debug)]
pub struct DAC(#[deku(bits = 10, endian = "big")] pub u16);

/// Function indicator, indicate the message's functionality.
#[derive(PartialOrd, PartialEq, DekuRead, DekuWrite, Debug)]
pub struct Function(#[deku(bits = 6)] pub u8);

/// Identifier for an Application-Specific Message.
#[derive(PartialOrd, PartialEq, DekuRead, DekuWrite, Debug)]
pub struct ApplicationId(pub DAC, pub Function);

/// The version indicator of the EU inland messages: three bits, 0 today, the rest reserved.
#[derive(Debug, DekuRead, DekuWrite)]
pub struct Version(#[deku(bits = 3)] pub u8);

/// The message version of the USCG messages: six bits, 0 a test message, 1 upwards a version
/// (the Geographic Notice limits it to 15, the Linked Text allows 63).
#[derive(Debug, DekuRead, DekuWrite)]
pub struct MessageVersion(#[deku(bits = 6)] pub u8);

/// Identifier linking together a group of messages from the same source.
#[derive(Debug, DekuRead, DekuWrite)]
pub struct Linkage(#[deku(bits = 10, endian = "big")] pub u16);

#[derive(Debug, DekuRead, DekuWrite)]
pub struct Capability(#[deku(bits = "1", pad_bits_after = "1")] pub bool);

#[repr(u8)]
#[derive(Debug, DekuRead, DekuWrite)]
#[deku(bits = 3, id_type = "u8")]
pub enum ApplicationResponse {
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
    /// Text using 6-bit ASCII, IMO SN.1/Circ.289:
    /// <https://www.e-navigation.nl/content/text-using-6-bit-ascii>.
    #[deku(id = "ApplicationId(DAC(1), Function(0))")]
    Text {
        #[deku(bits = "1")]
        must_acknowledge: bool,
        #[deku(bits = 11, endian = "big")]
        sequence: u16,
        text: SixBitString,
        // TODO add padding to byte boundary
    },
    /// Interrogation for specified FMs within the IAI branch, IMO SN.1/Circ.289:
    /// <https://www.e-navigation.nl/content/interrogation-specified-fms-within-iai-branch>.
    #[deku(id = "ApplicationId(DAC(1), Function(2))")]
    FunctionInterrogation {
        #[deku(pad_bits_after = "64")]
        request: ApplicationId,
    },
    /// Capability interrogation, IMO SN.1/Circ.289:
    /// <https://www.e-navigation.nl/content/capability-interrogation-0>.
    #[deku(id = "ApplicationId(DAC(1), Function(3))")]
    CapabilityInterrogation {
        #[deku(pad_bits_after = "70")]
        request: DAC,
    },
    /// Capability reply, IMO SN.1/Circ.289: <https://www.e-navigation.nl/content/capability-reply-0>.
    #[deku(id = "ApplicationId(DAC(1), Function(4))")]
    CapabilityReply { dac: DAC, functions: [Capability; 64] },
    /// Application acknowledgement to an addressed binary message, IMO SN.1/Circ.289:
    /// <https://www.e-navigation.nl/content/application-acknowledgement-addressed-binary-message>.
    #[deku(id = "ApplicationId(DAC(1), Function(5))")]
    ApplicationAcknowledgement {
        requested: ApplicationId,
        #[deku(bits = 11, endian = "big")]
        sequence: u16,
        is_available: bool,
        #[deku(pad_bits_after = "49")]
        response: ApplicationResponse,
    },
    /// Area notice, addressed, IMO SN.1/Circ.289: <https://www.e-navigation.nl/content/area-notice>.
    #[deku(id = "ApplicationId(DAC(1), Function(23))")]
    AreaNotice(AreaNotice),
    /// Geographic Notice, EU inland, in force since 18 October 2021:
    /// <https://www.e-navigation.nl/content/geographic-notice-1>.
    #[deku(id = "ApplicationId(DAC(200), Function(42))")]
    EuGeographicNotice(GeographicNotice),
    /// Geographic Notice, USCG, addressed (the same DAC and FI as the broadcast, in message 6):
    /// <https://www.e-navigation.nl/content/geographic-notice>.
    #[deku(id = "ApplicationId(DAC(367), Function(22))")]
    UscgGeographicNotice(UscgGeographicNotice),
    #[deku(id_pat = "_")]
    Unknown(ApplicationId, BinaryData),
}

#[allow(non_camel_case_types)]
#[derive(Debug, DekuRead, DekuWrite)]
#[deku(id_type = "ApplicationId")]
pub enum Broadcast {
    /// Text using 6-bit ASCII, IMO SN.1/Circ.289:
    /// <https://www.e-navigation.nl/content/text-using-6-bit-ascii-0>.
    #[deku(id = "ApplicationId(DAC(1), Function(0))")]
    Text {
        // must_acknowledge must always be zero for broadcast so treat as padding
        #[deku(bits = 11, endian = "big", pad_bits_before = "1")]
        sequence: u16,
        text: SixBitString,
        // TODO add padding to byte boundary
    },
    /// Area notice, IMO SN.1/Circ.289: <https://www.e-navigation.nl/content/area-notice-0>.
    #[deku(id = "ApplicationId(DAC(1), Function(22))")]
    AreaNotice(AreaNotice),
    /// Area Notice, USCG, deprecated in favour of the Geographic Notice:
    /// <https://www.e-navigation.nl/content/area-notice-1>.
    #[deku(id = "ApplicationId(DAC(366), Function(22))")]
    AreaNoticeV2(AreaNoticeV2),
    /// Geographic Notice, EU inland, a proposal: <https://www.e-navigation.nl/content/geographic-notice-0>.
    #[deku(id = "ApplicationId(DAC(200), Function(42))")]
    EuGeographicNotice(GeographicNotice),
    /// Geographic Notice, USCG: <https://www.e-navigation.nl/content/geographic-notice>.
    #[deku(id = "ApplicationId(DAC(367), Function(22))")]
    UscgGeographicNotice(UscgGeographicNotice),
    /// Linked Text, USCG, release 1a (3 April 2013): <https://www.e-navigation.nl/content/linked-text>,
    /// specification
    /// <https://www.e-navigation.nl/sites/default/files/asm_files/Linked%20Text%20Release%20Version%201a.pdf>.
    /// Words for the message with the same linkage from the same station; the version is 1.
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
