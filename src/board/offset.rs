// External imports
use std::{fmt, ops};

// Imports from super
use super::Direction;

/// Offset on the chess board
///
/// Can be used to add to existing squares
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Offset {
    file: i8,
    rank: i8,
}

impl Offset {
    /// Create new Offset from integers
    ///
    /// ## Examples
    /// ```
    /// use chess::board::Offset;
    ///
    /// // Create a knights move
    /// let offset = Offset::new(1, 2);
    ///
    /// assert_eq!(offset.file(), 1);
    /// assert_eq!(offset.rank(), 2);
    /// ```
    pub fn new(file: i8, rank: i8) -> Offset {
        Offset { file, rank }
    }

    /// Get the file of this Offset
    pub fn file(&self) -> i8 {
        self.file
    }

    /// Get the rank of this Offset
    pub fn rank(&self) -> i8 {
        self.rank
    }
}

impl From<&Direction> for Offset {
    /// Create new Offset from Direction
    ///
    /// ## Examples
    /// ```
    /// use chess::board::{Offset, Direction};
    ///
    /// // Create move one square down and one square to the right
    /// let offset = Offset::from(&Direction::DiagonalDownRight);
    ///
    /// assert_eq!(offset.file(), 1);
    /// assert_eq!(offset.rank(), -1);
    /// ```
    fn from(direction: &Direction) -> Self {
        match direction {
            Direction::HorizontalRight => Offset::new(1, 0),
            Direction::HorizontalLeft => Offset::new(-1, 0),
            Direction::VerticalUp => Offset::new(0, 1),
            Direction::VerticalDown => Offset::new(0, -1),
            Direction::DiagonalUpRight => Offset::new(1, 1),
            Direction::DiagonalUpLeft => Offset::new(-1, 1),
            Direction::DiagonalDownRight => Offset::new(1, -1),
            Direction::DiagonalDownLeft => Offset::new(-1, -1),
        }
    }
}

impl ops::Mul<i8> for Offset {
    type Output = Offset;

    /// Multiply an offset with a scalar
    ///
    /// ## Examples
    /// ```
    /// use chess::board::{Offset, Direction};
    ///
    /// // Create move one square down
    /// let offset = Offset::from(&Direction::VerticalDown);
    ///
    /// // Multipe offset with different numbers
    /// assert_eq!(offset, Offset::new(0, -1));
    /// assert_eq!(offset * 5, Offset::new(0, -5));
    /// assert_eq!(offset * 0, Offset::new(0, 0));
    /// ```
    fn mul(self, factor: i8) -> Self::Output {
        Offset::new(self.file * factor, self.rank * factor)
    }
}

impl fmt::Display for Offset {
    /// Converts the Offset to string
    ///
    /// ## Examples
    /// ```
    /// use chess::board::Offset;
    ///
    /// // Create a knights move
    /// let offset = Offset::new(-2, 1);
    ///
    /// assert_eq!(offset.to_string(), "(-2, 1)");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.file, self.rank)
    }
}
