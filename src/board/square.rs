// External imports
use std::collections;
use std::fmt;
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
        if file == 0 || file > 8 {
            panic!("file needs to be between 1 and 8");
        }
        if rank == 0 || rank > 8 {
            panic!("rank needs to be between 1 and 8");
        }

        Square { file, rank }
    }

    pub fn file(&self) -> char {
        match self.file {
            1 => 'A',
            2 => 'B',
            3 => 'C',
            4 => 'D',
            5 => 'E',
            6 => 'F',
            7 => 'G',
            8 => 'H',
            _ => {
                panic!("file needs to be between 1 and 8");
            }
        }
    }

    pub fn rank(&self) -> char {
        match self.rank {
            1 => '1',
            2 => '2',
            3 => '3',
            4 => '4',
            5 => '5',
            6 => '6',
            7 => '7',
            8 => '8',
            _ => {
                panic!("file needs to be between 1 and 8");
            }
        }
    }

    pub fn up(&self, offset: usize) -> Option<Square> {
        if self.rank + offset > 8 {
            // Avoid going off the board
            None
        } else {
            // Increase rank by offset
            Some(Square::new(self.file, self.rank + offset))
        }
    }

    pub fn down(&self, offset: usize) -> Option<Square> {
        if self.rank <= offset {
            // Avoid going off the board
            None
        } else {
            // Decrease rank by offset
            Some(Square::new(self.file, self.rank - offset))
        }
    }
}
impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.file(), self.rank())
    }
}

// Custom type alias for 8×8 board of squares
pub type Squares = collections::HashMap<Square, piece::Piece>;
