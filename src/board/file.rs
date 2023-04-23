// External imports
use core::cmp;
use std::{fmt, ops};

/// Represents a column of the chess board
///
/// A signed integer is used to allow for easy arithmetic of adding and substracting offsets
///
/// ```
/// use chess::board::File;
///
/// // Second file is the B-file
/// let file = File::from(2);
/// assert_eq!(file.to_string(), "b");
/// ```
#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
pub struct File(u8);

impl From<u8> for File {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl ops::Add<i8> for &File {
    type Output = File;

    fn add(self, offset: i8) -> Self::Output {
        // It is fine to use saturating_add since 0 is an invalid file anyways
        File(self.0.saturating_add_signed(offset))
    }
}

impl cmp::PartialEq<u8> for File {
    fn eq(&self, other: &u8) -> bool {
        self.0.eq(other)
    }
}

impl cmp::PartialOrd<u8> for File {
    fn partial_cmp(&self, other: &u8) -> Option<cmp::Ordering> {
        self.0.partial_cmp(other)
    }
}

impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if 0 < self.0 && self.0 <= 8 {
            // Calculate the ASCII value
            let ascii = 96_u32.saturating_add_signed(self.0 as i32);

            let character = char::from_u32(ascii).expect("bound already verified");

            write!(f, "{}", character)
        } else {
            write!(f, "?")
        }
    }
}
