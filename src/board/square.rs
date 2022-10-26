// External imports
use std::collections;
use std::fmt;
// Absolute imports within crate
use crate::board::Offset;
use crate::piece::Piece;

// Custom type alias for file and rank
pub type File = i8;
pub type Rank = i8;

// Custom type alias for a line of squares
pub type Line = Vec<Square>;

// Custom type alias for 8×8 board of squares
pub type PiecePlacement = collections::HashMap<Square, Piece>;

// All offsets corresponding to knight moves
const KNIGHT_MOVE_OFFSETS: [Offset; 8] = [
    Offset::new(2, 1),
    Offset::new(2, -1),
    Offset::new(-2, 1),
    Offset::new(-2, -1),
    Offset::new(1, 2),
    Offset::new(1, -2),
    Offset::new(-1, 2),
    Offset::new(-1, -2),
];

// All offsets corresponding to knight moves
const KING_MOVE_OFFSETS: [Offset; 8] = [
    Offset::new(1, 1),
    Offset::new(1, 0),
    Offset::new(1, -1),
    Offset::new(0, -1),
    Offset::new(-1, -1),
    Offset::new(-1, 0),
    Offset::new(-1, 1),
    Offset::new(0, 1),
];

// Type for squares of the chess board
#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Square {
    file: File,
    rank: Rank,
}
impl Square {
    fn is_valid_file(file: File) -> bool {
        1 <= file && file <= 8
    }

    fn is_valid_rank(rank: Rank) -> bool {
        1 <= rank && rank <= 8
    }

    pub fn new(file: File, rank: Rank) -> Square {
        if !Square::is_valid_file(file) {
            panic!("file needs to be between 1 and 8");
        }
        if !Square::is_valid_rank(rank) {
            panic!("rank needs to be between 1 and 8");
        }

        Square { file, rank }
    }

    pub fn file(&self) -> File {
        self.file
    }

    pub fn rank(&self) -> Rank {
        self.rank
    }

    pub fn is_valid_offset(&self, offset: &Offset) -> bool {
        Square::is_valid_file(self.file() + offset.file()) && Square::is_valid_rank(self.rank() + offset.rank())
    }

    pub fn copy_with_offset(&self, offset: &Offset) -> Square {
        Square::new(self.file() + offset.file(), self.rank() + offset.rank())
    }

    pub fn knight_moves(&self) -> Vec<Square> {
        KNIGHT_MOVE_OFFSETS
            .iter()
            .filter(|offset| self.is_valid_offset(offset))
            .map(|offset| self.copy_with_offset(offset))
            .collect()
    }

    pub fn king_moves(&self) -> Vec<Square> {
        KING_MOVE_OFFSETS
            .iter()
            .filter(|offset| self.is_valid_offset(offset))
            .map(|offset| self.copy_with_offset(offset))
            .collect()
    }

    fn line(&self, file_direction: File, rank_direction: Rank) -> Line {
        (1..)
            .map(|i| Offset::new(i * file_direction, i * rank_direction))
            .take_while(|offset| self.is_valid_offset(offset))
            .map(|offset| self.copy_with_offset(&offset))
            .collect()
    }

    pub fn top_vertical(&self) -> Line {
        self.line(0, 1)
    }

    pub fn down_vertical(&self) -> Line {
        self.line(0, -1)
    }

    pub fn right_horizontal(&self) -> Line {
        self.line(1, 0)
    }

    pub fn left_horizontal(&self) -> Line {
        self.line(-1, 0)
    }

    pub fn top_right_diagonal(&self) -> Line {
        self.line(1, 1)
    }

    pub fn top_left_diagonal(&self) -> Line {
        self.line(-1, 1)
    }

    pub fn bottom_right_diagonal(&self) -> Line {
        self.line(1, -1)
    }

    pub fn bottom_left_diagonal(&self) -> Line {
        self.line(-1, -1)
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
