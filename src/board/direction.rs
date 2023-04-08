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
    /// A diagonal line that increases in rank and increases in file
    DiagonalUpRight,
    /// A diagonal line that increases in rank and decreases in file
    DiagonalUpLeft,
    /// A diagonal line that decreases in rank and increases in file
    DiagonalDownRight,
    /// A diagonal line that decreases in rank and decreases in file
    DiagonalDownLeft,
}
