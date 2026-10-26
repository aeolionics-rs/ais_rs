//  SPDX-FileCopyrightText: 2026-2026. Aeolionics, LLC
//
//  SPDX-License-Identifier: Apache-2.0

//! Support for 6-bit text as defined in §A7-3 Table 45.

use deku::no_std_io::{Read, Seek};
use deku::prelude::{DekuError, DekuRead, DekuReader, DekuWrite, Reader};
use std::fmt::{Debug, Display, Write};
use std::str::FromStr;

/// A single 6-bit ASCII character.
#[derive(Clone, Copy, PartialOrd, PartialEq, DekuRead, DekuWrite)]
pub struct SixBitAscii(#[deku(bits = 6)] u8);

impl SixBitAscii {
    const ALPHABET: &[u8] = b"@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_ !\"#$%&'()*+,-./0123456789:;<=>?";

    const REVERSE: &[u8] = &{
        const _: () = assert!(SixBitAscii::ALPHABET.len() == 64);
        let mut result: [u8; 256] = [0xFFu8; 256];
        let mut i = 0;
        while i < Self::ALPHABET.len() {
            let ch = Self::ALPHABET[i];
            #[expect(clippy::cast_possible_truncation, reason = "the alphabet has 64 characters")]
            let code = i as u8;
            result[ch as usize] = code;
            i += 1;
        }
        result
    };
}

impl TryFrom<u8> for SixBitAscii {
    type Error = ();

    /// Conversion from a u8 containing a standard ASCII character.
    ///
    /// Returns Err if the ASCII character is not valid per Table 45.
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let result = SixBitAscii::REVERSE[value as usize];
        if result == 0xff { Err(()) } else { Ok(Self(result)) }
    }
}

impl From<SixBitAscii> for u8 {
    /// Conversion to a standard ASCII character.
    fn from(value: SixBitAscii) -> Self {
        SixBitAscii::ALPHABET[value.0 as usize]
    }
}

impl From<SixBitAscii> for char {
    /// Conversion to a standard ASCII character.
    fn from(value: SixBitAscii) -> Self {
        SixBitAscii::ALPHABET[value.0 as usize] as char
    }
}

impl Debug for SixBitAscii {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&char::from(*self), f)
    }
}

/// A sequence of 6-bit ASCII characters with a fixed length.
///
/// The text will be padded with value zero (ASCII '@').
#[derive(DekuRead, DekuWrite)]
pub struct SixBitText<const SIZE: usize>([SixBitAscii; SIZE]);

impl<const SIZE: usize> FromStr for SixBitText<SIZE> {
    type Err = ();

    /// Create 6-bit text from a `str`.
    ///
    /// An error will be returned if the string is too long or contains characters that cannot be
    /// represented as 6-bit ASCII.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut result = [b'@'.try_into()?; SIZE];
        if s.len() > SIZE {
            return Err(());
        }
        for (i, ch) in s.as_bytes().iter().enumerate() {
            result[i] = (*ch).try_into()?;
        }
        Ok(Self(result))
    }
}

impl<const SIZE: usize> SixBitText<SIZE> {
    /// Iterate over the characters as standard ASCII, including any padding.
    #[must_use]
    pub fn iter(&self) -> SixBitIter<'_> {
        SixBitIter(self.0.iter())
    }
}

/// An iterator over the characters of a [`SixBitText`] or [`SixBitString`] as standard ASCII.
pub struct SixBitIter<'a>(std::slice::Iter<'a, SixBitAscii>);

impl Iterator for SixBitIter<'_> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|ch| u8::from(*ch))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl DoubleEndedIterator for SixBitIter<'_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back().map(|ch| u8::from(*ch))
    }
}

impl ExactSizeIterator for SixBitIter<'_> {}

impl<'a, const SIZE: usize> IntoIterator for &'a SixBitText<SIZE> {
    type Item = u8;
    type IntoIter = SixBitIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<const SIZE: usize> Display for SixBitText<SIZE> {
    /// Display 6-bit ASCII text.
    ///
    /// Any trailing 0 characters (ASCII '@') will be omitted.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut last = SIZE;
        while last > 0 && self.0[last - 1] == SixBitAscii(0) {
            last -= 1;
        }
        for ch in &self.0[..last] {
            f.write_char(u8::from(*ch).into())?;
        }
        Ok(())
    }
}

impl<const SIZE: usize> Debug for SixBitText<SIZE> {
    /// Debug output as a quoted string, including any padding.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text: String = self.iter().map(char::from).collect();
        Debug::fmt(&text, f)
    }
}

/// Variable-length 6-bit text that extends to the end of a message.
///
/// When decoding, any trailing bits too few to form a character are treated as spare bits and
/// ignored, e.g. the bits that pad an AtoN report's name extension to a byte boundary.
#[derive(DekuRead, DekuWrite)]
pub struct SixBitString {
    #[deku(reader = "SixBitString::read_chars(deku::reader)")]
    value: Vec<SixBitAscii>,
}

impl SixBitString {
    /// Read characters until fewer than 6 bits remain.
    fn read_chars<R: Read + Seek>(reader: &mut Reader<R>) -> Result<Vec<SixBitAscii>, DekuError> {
        let mut value = Vec::new();
        while !reader.end() {
            match SixBitAscii::from_reader_with_ctx(reader, ()) {
                Ok(ch) => value.push(ch),
                Err(DekuError::Incomplete(_)) => break,
                Err(e) => return Err(e),
            }
        }
        Ok(value)
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.value.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.value.is_empty()
    }

    /// Iterate over the characters as standard ASCII.
    #[must_use]
    pub fn iter(&self) -> SixBitIter<'_> {
        SixBitIter(self.value.iter())
    }
}

impl<'a> IntoIterator for &'a SixBitString {
    type Item = u8;
    type IntoIter = SixBitIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl FromStr for SixBitString {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut result = Vec::with_capacity(s.len());
        for ch in s.as_bytes() {
            result.push(SixBitAscii::try_from(*ch)?);
        }
        Ok(SixBitString { value: result })
    }
}

impl Display for SixBitString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for six_bit in &self.value {
            f.write_char((*six_bit).into())?;
        }
        Ok(())
    }
}

impl Debug for SixBitString {
    /// Debug output as a quoted string.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.to_string(), f)
    }
}

#[cfg(test)]
#[test]
fn test_chars() {
    assert_eq!(b'W', SixBitAscii(23).into());
    assert_eq!(u8::from(SixBitAscii(52)), b'4');
}

#[cfg(test)]
#[test]
fn test_text_iter() {
    let text = SixBitText::<8>::from_str("HELLO").unwrap();
    assert_eq!(text.iter().collect::<Vec<u8>>(), b"HELLO@@@");
    assert_eq!(text.iter().len(), 8);
    assert_eq!(text.iter().next_back(), Some(b'@'));
    assert_eq!(text.into_iter().take(2).collect::<Vec<u8>>(), b"HE");
}

#[cfg(test)]
#[test]
fn test_string_iter() {
    let text = SixBitString::from_str("AIS").unwrap();
    assert_eq!(text.iter().collect::<Vec<u8>>(), b"AIS");
    assert_eq!(text.iter().len(), 3);
    assert_eq!(text.iter().next_back(), Some(b'S'));
    assert_eq!(text.into_iter().take(2).collect::<Vec<u8>>(), b"AI");
}

#[cfg(test)]
#[test]
fn test_text_display() {
    let text = SixBitText::<7>::from_str("AIS").unwrap();
    assert_eq!(text.to_string(), "AIS");
}

#[cfg(test)]
#[test]
fn test_debug() {
    let text = SixBitText::<5>::from_str("AIS").unwrap();
    assert_eq!(format!("{text:?}"), "\"AIS@@\"");
    let text = SixBitString::from_str("AIS").unwrap();
    assert_eq!(format!("{text:?}"), "\"AIS\"");
}
