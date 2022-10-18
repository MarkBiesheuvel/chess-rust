// External imports
use std::fmt;
// Absolute imports within crate
use crate::piece;

// Type for squares of the chess board
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Square {
    Taken(piece::Piece),
    Empty,
}
impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Square::Taken(piece) => {
                write!(f, "{}", piece)
            }
            Square::Empty => {
                write!(f, " ")
            }
        }
    }
}

// Custom type alias for 8×8 board of squares
pub type Squares = [[Square; 8]; 8];
