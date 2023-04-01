// External imports
use std::fmt;

/// Offset on the chess board
///
/// Can be used to add to existing squares
#[derive(Debug, PartialEq, Eq)]
pub struct Offset {
    file: i8,
    rank: i8,
}

impl Offset {
    /// Create new Square
    ///
    /// ## Examples
    /// ```
    /// use chess::board::Offset;
    ///
    /// // Offset might contain negative numbers
    /// let offset = Offset::new(-2, 1);
    /// assert_eq!(offset.to_string(), "(-2, 1)");
    /// ```
    pub const fn new(file: i8, rank: i8) -> Offset {
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

impl fmt::Display for Offset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.file, self.rank)
    }
}
