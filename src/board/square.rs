// External imports
use std::collections;
use std::fmt;
// Absolute imports within crate
use crate::piece;

// Type for squares of the chess board
#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Square {
    file: i8,
    rank: i8,
}
impl Square {
    pub fn is_valid(index: i8) -> bool {
        1 <= index && index <= 8
    }

    pub fn new(file: i8, rank: i8) -> Square {
        if !Square::is_valid(file) {
            panic!("file needs to be between 1 and 8");
        }
        if !Square::is_valid(rank) {
            panic!("rank needs to be between 1 and 8");
        }

        Square { file, rank }
    }

    pub fn file(&self) -> i8 {
        self.file
    }

    pub fn rank(&self) -> i8 {
        self.rank
    }

    pub fn copy_with_offset(&self, file_offset: i8, rank_offset: i8) -> Square {
        Square::new(self.file + file_offset, self.rank + rank_offset)
    }

    pub fn up(&self, offset: i8) -> Square {
        // Increase rank by offset
        self.copy_with_offset(0, offset)
    }

    pub fn down(&self, offset: i8) -> Square {
        // Decrease rank by offset
        self.copy_with_offset(0, -offset)
    }

    pub fn right(&self, offset: i8) -> Square {
        // Increase file by offset
        self.copy_with_offset(offset, 0)
    }

    pub fn left(&self, offset: i8) -> Square {
        // Decrease file by offset
        self.copy_with_offset(-offset, 0)
    }

    pub fn up_vector(&self) -> Vec<Square> {
        (1..)
            .take_while(|offset| Square::is_valid(self.rank + offset))
            .map(|offset| self.up(offset))
            .collect()
    }

    pub fn down_vector(&self) -> Vec<Square> {
        (1..)
            .take_while(|offset| Square::is_valid(self.rank - offset))
            .map(|offset| self.down(offset))
            .collect()
    }

    pub fn right_vector(&self) -> Vec<Square> {
        (1..)
            .take_while(|offset| Square::is_valid(self.file + offset))
            .map(|offset| self.right(offset))
            .collect()
    }

    pub fn left_vector(&self) -> Vec<Square> {
        (1..)
            .take_while(|offset| Square::is_valid(self.file - offset))
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
