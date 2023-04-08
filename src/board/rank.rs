// External imports
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
pub struct Rank(i8);

impl From<i8> for Rank {
    fn from(value: i8) -> Self {
        Self(value)
    }
}

impl ops::Add<i8> for &Rank {
    type Output = Rank;

    fn add(self, offset: i8) -> Self::Output {
        Rank(self.0 + offset)
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
