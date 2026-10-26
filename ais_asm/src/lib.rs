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
use ais_types::{BinaryData, Minutes, MonthDayHourMinute};
use area_notice::AreaNotice;
use area_notice_v2::AreaNoticeV2;
use core::fmt::{self, Display, Formatter};
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
#[derive(Debug, Clone, Copy, DekuRead, DekuWrite)]
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

impl Display for DAC {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "DAC {}", self.0)
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "FI {}", self.0)
    }
}

impl Display for ApplicationId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.0, self.1)
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "version {}", self.0)
    }
}

impl Display for MessageVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.0 {
            0 => f.write_str("test"),
            version => write!(f, "version {version}"),
        }
    }
}

impl Display for Linkage {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Display for Capability {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(if self.0 { "supported" } else { "not supported" })
    }
}

impl Display for ApplicationResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ApplicationResponse::UnableToResponse => f.write_str("unable to respond"),
            ApplicationResponse::ReceptionAcknowledged => f.write_str("reception acknowledged"),
            ApplicationResponse::ResponseToFollow => f.write_str("response to follow"),
            ApplicationResponse::Inhibited => f.write_str("inhibited"),
            reserved => write!(f, "response {}", *reserved as u8),
        }
    }
}

/// Text as it reads, without the '@' and spaces that pad it.
fn text(text: &impl Display) -> String {
    text.to_string().trim_end_matches(['@', ' ']).to_owned()
}

/// Function indicators as runs, `0, 2-5, 22`.
fn runs(f: &mut Formatter<'_>, numbers: impl Iterator<Item = usize>) -> fmt::Result {
    let mut numbers = numbers.peekable();
    let mut first = true;
    while let Some(start) = numbers.next() {
        let mut end = start;
        while numbers.peek() == Some(&(end + 1)) {
            end = numbers.next().unwrap_or(end);
        }
        f.write_str(if first { "" } else { ", " })?;
        first = false;
        if end == start { write!(f, "{start}")? } else { write!(f, "{start}-{end}")? }
    }
    if first { f.write_str("none") } else { Ok(()) }
}

/// A notice's sub-area, as the summary of a notice names it.
pub(crate) trait Piece {
    /// The shape it is, `circle` or `point` say; `None` for a piece of the notice's text.
    fn shape(&self) -> Option<&'static str>;
    /// Its piece of the notice's text, if it is one.
    fn text(&self) -> Option<String>;
}

/// A notice on one line: its name and linkage, what it is about, its action, when it starts and
/// for how long, its shapes in order (a run of one shape counted), and its text.
pub(crate) fn notice(
    f: &mut Formatter<'_>,
    name: impl Display,
    linkage: &Linkage,
    (description, cancels): (&dyn Display, bool),
    action: Option<&dyn Display>,
    (start, duration): (&MonthDayHourMinute, &Minutes),
    pieces: &[impl Piece],
) -> fmt::Result {
    write!(f, "{name} {linkage}: {description}")?;
    if cancels {
        return Ok(());
    }
    if let Some(action) = action {
        write!(f, ", {action}")?;
    }
    match start.to_string().as_str() {
        "n/a" => f.write_str(", start n/a")?,
        start => write!(f, ", {start}")?,
    }
    match duration.0 {
        0 => f.write_str(", cancelled")?,
        262_143 => f.write_str(", duration n/a")?,
        _ => write!(f, " for {duration}")?,
    }
    let mut shapes: Vec<(&str, usize)> = Vec::new();
    for shape in pieces.iter().filter_map(Piece::shape) {
        match shapes.last_mut() {
            Some((last, count)) if *last == shape => *count += 1,
            _ => shapes.push((shape, 1)),
        }
    }
    for (i, (shape, count)) in shapes.iter().enumerate() {
        f.write_str(if i == 0 { "; " } else { ", " })?;
        if *count == 1 { f.write_str(shape)? } else { write!(f, "{shape} ×{count}")? }
    }
    let words: String = pieces.iter().filter_map(Piece::text).collect();
    let words = text(&words);
    if !words.is_empty() {
        write!(f, "; {words:?}")?;
    }
    Ok(())
}

impl Display for Addressed {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Addressed::Text { must_acknowledge, sequence, text: words } => {
                write!(f, "text #{sequence}, {:?}", text(words))?;
                if *must_acknowledge { f.write_str(", to be acknowledged") } else { Ok(()) }
            }
            Addressed::FunctionInterrogation { request } => write!(f, "asks for {request}"),
            Addressed::CapabilityInterrogation { request } => write!(f, "asks for the {request} capabilities"),
            Addressed::CapabilityReply { dac, functions } => {
                write!(f, "{dac} supports FI ")?;
                runs(f, functions.iter().enumerate().filter(|(_, c)| c.0).map(|(i, _)| i))
            }
            Addressed::ApplicationAcknowledgement { requested, sequence, is_available, response } => {
                write!(f, "acknowledges {requested} #{sequence}, {response}")?;
                if *is_available { Ok(()) } else { f.write_str(", not available") }
            }
            Addressed::AreaNotice(notice) => notice.fmt(f),
            Addressed::EuGeographicNotice(notice) => notice.fmt(f),
            Addressed::UscgGeographicNotice(notice) => notice.fmt(f),
            Addressed::Unknown(application, data) => write!(f, "{application}, {data}"),
        }
    }
}

impl Display for Broadcast {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Broadcast::Text { sequence, text: words } => write!(f, "text #{sequence}, {:?}", text(words)),
            Broadcast::AreaNotice(notice) => notice.fmt(f),
            Broadcast::AreaNoticeV2(notice) => notice.fmt(f),
            Broadcast::EuGeographicNotice(notice) => notice.fmt(f),
            Broadcast::UscgGeographicNotice(notice) => notice.fmt(f),
            Broadcast::LinkedText { linkage, text: words, .. } => {
                write!(f, "Linked Text {linkage}, {:?}", text(words))
            }
            Broadcast::Unknown(application, data) => write!(f, "{application}, {data}"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use ais_types::{Minutes, MonthDayHourMinute};
    use area_notice::{Description, Leg, Position, Radius, Scale, Shape, SubArea};

    fn circle(radius: u16) -> SubArea {
        SubArea::Circle { scale: Scale(1), position: Position::new(51.51, -0.08, 4), radius: Radius(radius) }
    }

    #[test]
    fn a_notice_displays_on_one_line() {
        let polygon = SubArea::Polygon(Shape {
            scale: Scale(0),
            points: [
                Leg { angle: 180, distance: 300 },
                Leg { angle: 360, distance: 300 },
                Leg { angle: 540, distance: 300 },
                Leg { angle: 720, distance: 0 },
            ],
        });
        let notice = AreaNotice {
            linkage: Linkage(2),
            description: Description::DiversDown,
            start: MonthDayHourMinute { month: 10, day: 26, hour: 18, minute: 59 },
            duration: Minutes(10),
            subareas: vec![
                circle(50),
                circle(0),
                polygon,
                SubArea::AssociatedText("DIVERS DOWN @@".parse().unwrap()),
                SubArea::AssociatedText("KEEP CLEAR@@@@".parse().unwrap()),
            ],
        };
        assert_eq!(
            Broadcast::AreaNotice(notice).to_string(),
            "Area Notice 2: divers down, 10-26 18:59 UTC for 10 min; circle, point, polygon; \"DIVERS DOWN KEEP CLEAR\""
        );
        assert_eq!(circle(50).to_string(), "circle 500 m at 51.51000°N 0.08000°W");

        // A cancellation says no more; a run of one shape is counted.
        let cancellation = AreaNotice {
            linkage: Linkage(2),
            description: Description::Cancellation,
            start: MonthDayHourMinute::default(),
            duration: Minutes(0),
            subareas: vec![],
        };
        assert_eq!(cancellation.to_string(), "Area Notice 2: cancellation");
        let undated = AreaNotice {
            linkage: Linkage(3),
            description: Description::StrongCurrents,
            start: MonthDayHourMinute::default(),
            duration: Minutes(262_143),
            subareas: vec![circle(10), circle(20)],
        };
        assert_eq!(undated.to_string(), "Area Notice 3: strong currents, start n/a, duration n/a; circle ×2");
    }

    /// The Area Notices and the Geographic Notices each have their own table: the same code may mean
    /// something else, and mostly doesn't.
    #[test]
    fn each_family_reads_its_own_descriptions() {
        use deku::DekuContainerRead;
        use geographic_notice::Description as Geographic;
        let read = |code: u8| {
            let area = Description::from_bytes((&[code << 1], 0)).unwrap().1;
            let geographic = Geographic::from_bytes((&[code << 1], 0)).unwrap().1;
            (area.to_string(), geographic.to_string())
        };
        assert_eq!(read(32), ("fishing prohibited".into(), "other environmental caution".into()));
        assert_eq!(read(33), ("no anchoring".into(), "fishing prohibited".into()));
        assert_eq!(read(96), ("sunken vessel".into(), "other information".into()));
        assert_eq!(read(47), ("description 47".into(), "ice edge".into()), "reserved in the Area Notice");
        assert_eq!(read(10), ("divers down".into(), "divers down".into()));
        assert_eq!(read(126), ("cancellation".into(), "cancellation".into()));
    }

    #[test]
    fn the_other_messages_display_what_they_say() {
        let linked = Broadcast::LinkedText { version: 1, linkage: Linkage(1), text: "WHALES SIGHTED".parse().unwrap() };
        assert_eq!(linked.to_string(), "Linked Text 1, \"WHALES SIGHTED\"");
        let text = Addressed::Text { must_acknowledge: true, sequence: 7, text: "HELLO@@".parse().unwrap() };
        assert_eq!(text.to_string(), "text #7, \"HELLO\", to be acknowledged");
        let mut functions: [Capability; 64] = core::array::from_fn(|_| Capability(false));
        for i in [0, 2, 3, 4, 5, 22] {
            functions[i] = Capability(true);
        }
        let reply = Addressed::CapabilityReply { dac: DAC(1), functions };
        assert_eq!(reply.to_string(), "DAC 1 supports FI 0, 2-5, 22");
        let unknown = Broadcast::Unknown(ApplicationId(DAC(235), Function(10)), vec![0u8; 3].into());
        assert_eq!(unknown.to_string(), "DAC 235 FI 10, 24 bits");
    }
}
