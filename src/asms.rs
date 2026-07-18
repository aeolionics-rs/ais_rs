pub mod area_notice;

use crate::asms::area_notice::GeographicNotice;
use deku::{DekuRead, DekuWrite};

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
pub enum ApplicationMessage {
    #[deku(id = "(DAC(200), Function(42))")]
    GeographicNotice_EU(GeographicNotice),
    #[deku(id = "(DAC(367), Function(22))")]
    GeographicNotice_USCG(GeographicNotice),
    #[deku(id_pat = "_")]
    Unknown((DAC, Function), #[deku(read_all)] Vec<u8>),
}
