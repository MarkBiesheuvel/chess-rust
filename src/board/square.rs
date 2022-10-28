// External imports
use std::fmt;
// Imports from parent
use super::{File, Offset, Rank, SquareList};

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

    // Square should generally not be copied, but only referenced
    // Only exception is for pawn promotion, where one destination square is used in multiple moves/promotions
    pub fn copy(&self) -> Square {
        Square::new(self.file(), self.rank())
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

    // Represents a group of square created from a list of offsets
    fn create_group_of_squares(&self, offsets: &[Offset]) -> SquareList {
        // Iterate through the list and keep all squares that are valid (within the 8×8 board)
        offsets
            .into_iter()
            .filter(|offset| self.is_valid_offset(&offset))
            .map(|offset| self.copy_with_offset(&offset))
            .collect()
    }

    // Represents a line of sight into a particular direction
    fn create_line_of_squares(&self, file_direction: File, rank_direction: Rank) -> SquareList {
        // Iterate into the given direction and stop as soon as it reached the end of the 8×8 board
        (1..)
            .map(|i| Offset::new(i * file_direction, i * rank_direction))
            .take_while(|offset| self.is_valid_offset(offset))
            .map(|offset| self.copy_with_offset(&offset))
            .collect()
    }

    pub fn squares_on_knight_moves(&self) -> SquareList {
        self.create_group_of_squares(&KNIGHT_MOVE_OFFSETS)
    }

    pub fn squares_on_king_move(&self) -> SquareList {
        self.create_group_of_squares(&KING_MOVE_OFFSETS)
    }

    pub fn squares_on_up_vertical(&self) -> SquareList {
        self.create_line_of_squares(0, 1)
    }

    pub fn squares_on_down_vertical(&self) -> SquareList {
        self.create_line_of_squares(0, -1)
    }

    pub fn squares_on_right_horizontal(&self) -> SquareList {
        self.create_line_of_squares(1, 0)
    }

    pub fn squares_on_left_horizontal(&self) -> SquareList {
        self.create_line_of_squares(-1, 0)
    }

    pub fn squares_on_top_right_diagonal(&self) -> SquareList {
        self.create_line_of_squares(1, 1)
    }

    pub fn squares_on_top_left_diagonal(&self) -> SquareList {
        self.create_line_of_squares(-1, 1)
    }

    pub fn squares_on_bottom_right_diagonal(&self) -> SquareList {
        self.create_line_of_squares(1, -1)
    }

    pub fn squares_on_bottom_left_diagonal(&self) -> SquareList {
        self.create_line_of_squares(-1, -1)
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
