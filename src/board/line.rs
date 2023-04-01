// Imports from super
use super::{Offset, Square};

/// A line that originates from the origin square and moves into the direction
///
/// ## Examples
/// ```
/// use chess::board::{Square, Offset, Line};
///
/// // Start at c1 and move like a bishop
/// let origin = Square::from((3, 1));
/// let direction = Offset::new(1, 1);
/// let line = Line::new(origin, direction);
///
/// // Line should contain these squares
/// let mut iter = line.into_iterator();
/// assert_eq!(iter.next().unwrap().to_string(), "d2");
/// assert_eq!(iter.next().unwrap().to_string(), "e3");
/// assert_eq!(iter.next().unwrap().to_string(), "f4");
/// assert_eq!(iter.next().unwrap().to_string(), "g5");
/// assert_eq!(iter.next().unwrap().to_string(), "h6");
/// ```
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Line {
    origin: Square,
    direction: Offset,
}

impl Line {
    /// Create a new line
    pub fn new(origin: Square, direction: Offset) -> Line {
        Line { origin, direction }
    }

    /// Create an iterator of Squares
    ///
    /// TODO: impl IntoIter
    pub fn into_iterator(self) -> impl Iterator<Item = Square> {
        (1..).map(move |i| self.origin + (self.direction * i))
    }
}
