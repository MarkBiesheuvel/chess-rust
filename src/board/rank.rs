// External imports
use core::cmp;
use std::{fmt, ops};

/// Represents a row of the chess board
///
/// A signed integer is used to allow for easy arithmetic of adding and substracting offsets
///
/// ```
/// use chess::board::Rank;
///
/// // Seventh rank is simply the seventh rank
/// let rank = Rank::from(7);
/// assert_eq!(rank.to_string(), "7");
/// ```
#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
pub struct Rank(u8);

impl From<u8> for Rank {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl ops::Add<i8> for &Rank {
    type Output = Rank;

    fn add(self, offset: i8) -> Self::Output {
        // It is fine to use saturating_add since 0 is an invalid file anyways
        Rank(self.0.saturating_add_signed(offset))
    }
}

impl cmp::PartialEq<u8> for Rank {
    fn eq(&self, other: &u8) -> bool {
        self.0.eq(other)
    }
}

impl cmp::PartialOrd<u8> for Rank {
    fn partial_cmp(&self, other: &u8) -> Option<cmp::Ordering> {
        self.0.partial_cmp(other)
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if 0 < self.0 && self.0 <= 8 {
            write!(f, "{}", self.0)
        } else {
            write!(f, "?")
        }
    }
}
