//  SPDX-FileCopyrightText: 2026-2026. Aeolionics, LLC
//
//  SPDX-License-Identifier: Apache-2.0

//! Application-specific messages.
pub mod area_notice;
mod area_notice_v2;
mod geographic_notice;

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
#[derive(DekuRead, DekuWrite)]
pub struct Version(#[deku(bits = 3)] pub u8);

/// Identifier linking together a group of messages from the same source.
#[derive(DekuRead, DekuWrite)]
pub struct Linkage(#[deku(bits = 10, endian = "big")] pub u16);

#[allow(non_camel_case_types)]
#[derive(DekuRead, DekuWrite)]
#[deku(id_type = "(DAC, Function)")]
pub enum Addressed {
    #[deku(id = "(DAC(1), Function(23))")]
    AreaNotice(AreaNotice),
    #[deku(id = "(DAC(200), Function(42))")]
    GeographicNotice(GeographicNotice),
    #[deku(id_pat = "_")]
    Unknown((DAC, Function), #[deku(read_all)] Vec<u8>),
}


#[allow(non_camel_case_types)]
#[derive(DekuRead, DekuWrite)]
#[deku(id_type = "ApplicationId")]
pub enum Broadcast {
    #[deku(id = "ApplicationId(DAC(1), Function(22))")]
    AreaNotice(AreaNotice),
    #[deku(id = "ApplicationId(DAC(366), Function(22))")]
    AreaNoticeV2(AreaNoticeV2),
    #[deku(id = "ApplicationId(DAC(367), Function(22))")]
    GeographicNotice(GeographicNotice),
    #[deku(id_pat = "_")]
    Unknown(ApplicationId, #[deku(read_all)] Vec<u8>),
}

