use super::{Linkage, Version};
use deku::{DekuRead, DekuWrite};
use std::str::FromStr;
use crate::six_bit_ascii::SixBitText;
use crate::types::{Minutes, MonthDayHourMinute};

#[derive(DekuRead, DekuWrite)]
pub struct GeographicNotice {
    #[deku(pad_bits_after = "3")]
    pub version: Version,
    pub linkage: Linkage,
    pub description: Description,
    pub start: MonthDayHourMinute,
    pub duration: Minutes,
    #[deku(pad_bits_after = "2")]
    pub action: Action,
    #[deku(read_all)]
    pub subareas: Vec<SubArea>,
}

#[derive(DekuRead, DekuWrite)]
#[deku(id_type = "u8", bits = 3)]
pub enum SubArea {
    #[deku(id = 0)]
    Circle(Circle),
    #[deku(id = 1)]
    Rectangle(Rectangle),
    #[deku(id = 2)]
    Sector(Sector),
    #[deku(id = 3)]
    Polyline(Shape),
    #[deku(id = 4)]
    Polygon(Shape),
    #[deku(id = 5)]
    AssociatedText(Text),
}

#[derive(DekuRead, DekuWrite)]
pub struct Circle {
    #[deku(bits = "2")]
    pub scale: u8,
    #[deku(bits = "28", endian = "big")]
    pub longitude: i32,
    #[deku(bits = "27", endian = "big")]
    pub latitude: i32,
    #[deku(bits = "3")]
    pub precision: u8,
    #[deku(bits = "12", endian = "big")]
    pub radius: u16,
    #[deku(bits = "2", pad_bits_after = "19")]
    pub link: u8,
}

#[derive(DekuRead, DekuWrite)]
pub struct Rectangle {
    #[deku(bits = "2")]
    pub scale: u8,
    #[deku(bits = "28", endian = "big")]
    pub longitude: i32,
    #[deku(bits = "27", endian = "big")]
    pub latitude: i32,
    #[deku(bits = "3")]
    pub precision: u8,
    #[deku(bits = "8")]
    pub east: u8,
    #[deku(bits = "8")]
    pub north: u8,
    #[deku(bits = "9", endian = "big", pad_bits_after = "8")]
    pub orientation: u16,
}

#[derive(DekuRead, DekuWrite)]
pub struct Sector {
    #[deku(bits = "2")]
    pub scale: u8,
    #[deku(bits = "28", endian = "big")]
    pub longitude: i32,
    #[deku(bits = "27", endian = "big")]
    pub latitude: i32,
    #[deku(bits = "3")]
    pub precision: u8,
    #[deku(bits = "12")]
    pub radius: u16,
    #[deku(bits = "9", endian = "big")]
    pub left: u16,
    #[deku(bits = "9", endian = "big", pad_bits_after = "3")]
    pub right: u16,
}

#[derive(DekuRead, DekuWrite)]
pub struct Leg {
    #[deku(bits = "10", endian = "big")]
    pub angle: u16,
    #[deku(bits = "11", endian = "big")]
    pub distance: u16,
}
#[derive(DekuRead, DekuWrite)]
pub struct Shape {
    #[deku(bits = "2")]
    pub scale: u8,
    points: [Leg; 4],
    #[deku(bits = "2", pad_bits_after = "6")]
    pub link: u8,
}

#[derive(DekuRead, DekuWrite)]
pub struct Text(#[deku(pad_bits_after = "3")] pub SixBitText<15>);
impl FromStr for Text {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(SixBitText::from_str(s)?))
    }
}

#[repr(u8)]
#[derive(DekuRead, DekuWrite)]
#[deku(id_type = "u8", bits = 1)]
pub enum Action {
    Advisement = 0,
    Directive = 1,
}

#[allow(non_camel_case_types)]
#[repr(u8)]
#[non_exhaustive]
#[derive(DekuRead, DekuWrite)]
#[deku(id_type = "u8", bits = 7)]
pub enum Description {
    MarineMammalHabitat = 0,
    MammalsInArea_ReduceSpeed = 1,
    MammalsInArea_StayClear = 2,
    MammalsInArea_ReportSightings = 3,
    ProtectedHabitat_ReduceSpeed = 4,
    ProtectedHabitat_StayClear = 5,
    ProtectedHabitat_NoFishing = 6,
    Other = 125,
    Cancellation = 126,
    Undefined = 127,
}
