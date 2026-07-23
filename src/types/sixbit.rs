//  SPDX-FileCopyrightText: 2026-2026. Aeolionics, LLC
//
//  SPDX-License-Identifier: Apache-2.0

//! Support for 6-bit text as defined in §A7-3 Table 45.

use deku::{DekuRead, DekuWrite};
use std::fmt::{Display, Write};
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
            result[ch as usize] = i as u8;
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
        if s.as_bytes().len() > SIZE {
            return Err(());
        }
        let mut i = 0;
        for ch in s.as_bytes() {
            result[i] = (*ch).try_into()?;
            i += 1;
        }
        Ok(Self(result))
    }
}

impl<const SIZE: usize> Display for SixBitText<SIZE> {
    /// Display 6-bit ASCII text.
    ///
    /// Any trailing 0 characters (ASCII '@') will be omitted.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut last = SIZE;
        while last > 0 && self.0[last - 1] == 0u8.try_into().unwrap() {
            last -= 1;
        }
        for ch in self.0[..last].iter() {
            f.write_char(u8::from(*ch).into())?;
        }
        Ok(())
    }
}


#[cfg(test)]
#[test]
fn test_chars() {
    assert_eq!(b'W', SixBitAscii(23).try_into().unwrap());
    assert_eq!(u8::from(SixBitAscii(52)), b'4');
}