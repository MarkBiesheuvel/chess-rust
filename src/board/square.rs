// External imports
use std::{fmt, ops};

// Imports from super
use super::{File, Offset, Rank};

/// A single square on the chess board
#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Square {
    file: File,
    rank: Rank,
}

impl Square {
    /// Create new Square
    ///
    /// ## Examples
    /// ```
    /// use chess::board::{File, Rank, Square};
    ///
    /// // Eighth file, seventh rank is h7
    /// let file = File::from(8);
    /// let rank = Rank::from(7);
    /// let square = Square::new(file, rank);
    /// assert_eq!(square.to_string(), "h7");
    /// ```
    pub fn new(file: File, rank: Rank) -> Square {
        Square { file, rank }
    }

    /// Get the file of this Square
    pub fn file(&self) -> &File {
        &self.file
    }

    /// Get the rank of this Square
    pub fn rank(&self) -> &Rank {
        &self.rank
    }
}

impl<F, R> From<(F, R)> for Square
where
    F: Into<File>,
    R: Into<Rank>,
{
    /// Create new Square from a tuple of items which can be converted into File and Rank
    ///
    /// ## Examples
    /// ```
    /// use chess::board::Square;
    ///
    /// // Second file, fifth rank is b5
    /// let square = Square::from((2, 5));
    /// assert_eq!(square.to_string(), "b5");
    /// ```
    fn from((file, rank): (F, R)) -> Self {
        Square::new(file.into(), rank.into())
    }
}

impl ops::Add<Offset> for Square {
    type Output = Square;

    /// Add an offset to a square
    ///
    /// ## Examples
    /// ```
    /// use chess::board::{Square, Offset};
    ///
    /// // Start at g8
    /// let starting_square = Square::from((7, 8));
    /// assert_eq!(starting_square.to_string(), "g8");
    ///
    /// // Make a knight move to f6
    /// let offset = Offset::new(-1 , -2);
    /// let desination_square = starting_square + offset;
    /// assert_eq!(desination_square.to_string(), "f6");
    /// ```
    fn add(self, offset: Offset) -> Self::Output {
        Square {
            file: self.file + offset.file(),
            rank: self.rank + offset.rank(),
        }
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.file, self.rank)
    }
}
