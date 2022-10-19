// External imports
use std::collections;
use std::fmt;
// Absolute imports within crate
use crate::piece;

// TODO: create offset class

// All permutations of 1,-1 and 2,-2 (in other words L shapes, in other words knight moves)
const KNIGHT_MOVE_OFFSETS: [(i8, i8); 8] = [(2, 1), (2, -1), (-2, 1), (-2, -1), (1, 2), (1, -2), (-1, 2), (-1, -2)];

// Type for squares of the chess board
#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Square {
    file: i8,
    rank: i8,
}
impl Square {
    fn is_valid_file(file: i8) -> bool {
        1 <= file && file <= 8
    }

    fn is_valid_rank(rank: i8) -> bool {
        1 <= rank && rank <= 8
    }

    pub fn new(file: i8, rank: i8) -> Square {
        if !Square::is_valid_file(file) {
            panic!("file needs to be between 1 and 8");
        }
        if !Square::is_valid_rank(rank) {
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

    pub fn is_valid_offset(&self, file_offset: i8, rank_offset: i8) -> bool {
        Square::is_valid_file(self.file + file_offset) && Square::is_valid_rank(self.rank + rank_offset)
    }

    pub fn copy_with_offset(&self, file_offset: i8, rank_offset: i8) -> Square {
        Square::new(self.file + file_offset, self.rank + rank_offset)
    }

    pub fn top_vertical(&self) -> Vec<Square> {
        (1..)
            .take_while(|offset| self.is_valid_offset(0, *offset))
            .map(|offset| self.copy_with_offset(0, offset))
            .collect()
    }

    pub fn down_vertical(&self) -> Vec<Square> {
        (1..)
            .take_while(|offset| self.is_valid_offset(0, -offset))
            .map(|offset| self.copy_with_offset(0, -offset))
            .collect()
    }

    pub fn right_horizontal(&self) -> Vec<Square> {
        (1..)
            .take_while(|offset| self.is_valid_offset(*offset, 0))
            .map(|offset| self.copy_with_offset(offset, 0))
            .collect()
    }

    pub fn left_horizontal(&self) -> Vec<Square> {
        (1..)
            .take_while(|offset| self.is_valid_offset(-offset, 0))
            .map(|offset| self.copy_with_offset(-offset, 0))
            .collect()
    }

    pub fn top_right_diagonal(&self) -> Vec<Square> {
        (1..)
            .take_while(|offset| self.is_valid_offset(*offset, *offset))
            .map(|offset| self.copy_with_offset(offset, offset))
            .collect()
    }

    pub fn top_left_diagonal(&self) -> Vec<Square> {
        (1..)
            .take_while(|offset| self.is_valid_offset(-offset, *offset))
            .map(|offset| self.copy_with_offset(-offset, offset))
            .collect()
    }

    pub fn bottom_right_diagonal(&self) -> Vec<Square> {
        (1..)
            .take_while(|offset| self.is_valid_offset(*offset, -offset))
            .map(|offset| self.copy_with_offset(offset, -offset))
            .collect()
    }

    pub fn bottom_left_diagonal(&self) -> Vec<Square> {
        (1..)
            .take_while(|offset| self.is_valid_offset(-offset, -offset))
            .map(|offset| self.copy_with_offset(-offset, -offset))
            .collect()
    }

    pub fn knight_moves(&self) -> Vec<Square> {
        KNIGHT_MOVE_OFFSETS
            .iter()
            .filter(|(file_offset, rank_offset)| self.is_valid_offset(*file_offset, *rank_offset))
            .map(|(file_offset, rank_offset)| self.copy_with_offset(*file_offset, *rank_offset))
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
