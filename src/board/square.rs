// External imports
use std::collections;
// Absolute imports within crate
use crate::piece;

// Type for squares of the chess board
#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Square {
    file: usize,
    rank: usize,
}
impl Square {
    pub fn new(file: usize, rank: usize) -> Square {
        Square { file, rank }
    }
}

// Custom type alias for 8×8 board of squares
pub type Squares = collections::HashMap<Square, piece::Piece>;
