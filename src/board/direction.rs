// Imports from super
use super::Offset;

/// A direction to move on the chess board
#[derive(Debug, PartialEq, Eq)]
pub enum Direction {
    /// A vertical line that increases in file
    HorizontalRight,
    /// A vertical line that decreases in file
    HorizontalLeft,
    /// A vertical line that increases in rank
    VerticalUp,
    /// A vertical line that decreases in rank
    VerticalDown,
    /// A diagonal line that increases in file and increases in rank
    DiagonalRightUp,
    /// A diagonal line that increases in file and decreases in rank
    DiagonalRightDown,
    /// A diagonal line that decreases in file and increases in rank
    DiagonalLeftUp,
    /// A diagonal line that decreases in file and decreases in rank
    DiagonalLeftDown,
}

impl From<&Direction> for Offset {
    /// Create new Offset from Direction
    ///
    /// ## Examples
    /// ```
    /// use chess::board::{Offset, Direction};
    ///
    /// // Create move one square down and one square to the right
    /// let offset = Offset::from(&Direction::DiagonalRightDown);
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
            Direction::DiagonalRightUp => Offset::new(1, 1),
            Direction::DiagonalRightDown => Offset::new(1, -1),
            Direction::DiagonalLeftUp => Offset::new(-1, 1),
            Direction::DiagonalLeftDown => Offset::new(-1, -1),
        }
    }
}
