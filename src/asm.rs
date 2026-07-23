//  SPDX-FileCopyrightText: 2026-2026. Aeolionics, LLC
//
//  SPDX-License-Identifier: Apache-2.0

//! Application-specific messages.
mod geographic_notice;
pub mod area_notice;
mod area_notice_v2;

use area_notice::AreaNotice;
use area_notice_v2::AreaNoticeV2;
use deku::{DekuRead, DekuWrite};
use geographic_notice::GeographicNotice;

#[derive(PartialOrd, PartialEq, DekuRead, DekuWrite, Debug)]
pub struct DAC(#[deku(bits = 10, endian = "big")] pub u16);

#[derive(PartialOrd, PartialEq, DekuRead, DekuWrite, Debug)]
pub struct Function(#[deku(bits = 6)] pub u8);

#[derive(DekuRead, DekuWrite)]
pub struct Version(#[deku(bits = 3)] pub u8);

#[derive(DekuRead, DekuWrite)]
pub struct Linkage(#[deku(bits = 10, endian = "big")] pub u16);

#[allow(non_camel_case_types)]
#[derive(DekuRead, DekuWrite)]
#[deku(id_type = "(DAC, Function)")]
pub enum AddressedApplicationMessage {
    #[deku(id = "(DAC(1), Function(23))")]
    AreaNotice(AreaNotice),
    #[deku(id = "(DAC(200), Function(42))")]
    GeographicNotice(GeographicNotice),
    #[deku(id_pat = "_")]
    Unknown((DAC, Function), #[deku(read_all)] Vec<u8>),
}

#[allow(non_camel_case_types)]
#[derive(DekuRead, DekuWrite)]
#[deku(id_type = "(DAC, Function)")]
pub enum BroadcastApplicationMessage {
    #[deku(id = "(DAC(1), Function(22))")]
    AreaNotice(AreaNotice),
    #[deku(id = "(DAC(366), Function(22))")]
    AreaNoticeV2(AreaNoticeV2),
    #[deku(id = "(DAC(367), Function(22))")]
    GeographicNotice(GeographicNotice),
    #[deku(id_pat = "_")]
    Unknown((DAC, Function), #[deku(read_all)] Vec<u8>),
}
