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

    pub fn file(&self) -> usize {
        self.file
    }

    pub fn rank(&self) -> usize {
        self.rank
    }

    pub fn up(&self, offset: usize) -> Square {
        // Increase rank by offset
        Square::new(self.file, self.rank + offset)
    }

    pub fn down(&self, offset: usize) -> Square {
        // Decrease rank by offset
        Square::new(self.file, self.rank - offset)
    }

    pub fn right(&self, offset: usize) -> Square {
        // Increase file by offset
        Square::new(self.file + offset, self.rank)
    }

    pub fn left(&self, offset: usize) -> Square {
        // Decrease file by offset
        Square::new(self.file - offset, self.rank)
    }

    pub fn up_vector(&self) -> Vec<Square> {
        (1..)
            .take_while(|offset| self.rank + offset <= 8)
            .map(|offset| self.up(offset))
            .collect()
    }

    pub fn down_vector(&self) -> Vec<Square> {
        (1..)
            .take_while(|offset| self.rank - offset >= 1)
            .map(|offset| self.down(offset))
            .collect()
    }

    pub fn right_vector(&self) -> Vec<Square> {
        (1..)
            .take_while(|offset| self.file + offset <= 8)
            .map(|offset| self.right(offset))
            .collect()
    }

    pub fn left_vector(&self) -> Vec<Square> {
        (1..)
            .take_while(|offset| self.file - offset >= 1)
            .map(|offset| self.left(offset))
            .collect()
    }
}
impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let file = match self.file {
            1 => 'a',
            2 => 'b',
            3 => 'c',
            4 => 'd',
            5 => 'e',
            6 => 'f',
            7 => 'g',
            8 => 'h',
            _ => {
                panic!("file needs to be between 1 and 8");
            }
        };
        let rank = match self.rank {
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
        };
        write!(f, "{}{}", file, rank)
    }
}

// Custom type alias for 8×8 board of squares
pub type Squares = collections::HashMap<Square, piece::Piece>;
