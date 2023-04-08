// Imports from super
use super::{Direction, Offset, Square};

// Private type alias
type BoxedSquareIterator = Box<dyn Iterator<Item = Square>>;

/// A line that originates from the origin square and moves into the direction
///
/// ## Examples
/// ```
/// use chess::board::{Direction, Line, Square};
///
/// // Start at c1 and move like a bishop
/// let origin = "c1".parse()?;
/// let direction = Direction::DiagonalUpRight;
/// let line = Line::new(origin, direction);
///
/// // Line should contain these squares
/// let mut iter = line.into_iter();
/// assert_eq!(iter.next().unwrap().to_string(), "d2");
/// assert_eq!(iter.next().unwrap().to_string(), "e3");
/// assert_eq!(iter.next().unwrap().to_string(), "f4");
/// assert_eq!(iter.next().unwrap().to_string(), "g5");
/// assert_eq!(iter.next().unwrap().to_string(), "h6");
///
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[derive(Debug, PartialEq, Eq)]
pub struct Line {
    origin: Square,
    direction: Direction,
}

impl Line {
    /// Create a new line
    pub fn new(origin: Square, direction: Direction) -> Line {
        Line { origin, direction }
    }
}

impl IntoIterator for Line {
    type Item = Square;
    type IntoIter = BoxedSquareIterator;

    fn into_iter(self) -> Self::IntoIter {
        Box::new((1..).map(move |i| &self.origin + (Offset::from(&self.direction) * i)))
    }
}
