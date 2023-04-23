// External imports
use std::collections::HashMap;
use std::fmt;

// Imports from crate
use crate::piece::{behavior::*, Color, Piece};

// Imports from super
use super::{Square, SquareNotation::*, SquareStatus};

/// Chess board consisting of N×N squares
///
/// ## Examples
/// ```
/// use std::str::FromStr;
/// use chess::board::{Board, Square, SquareStatus, SquareNotation::*};
/// use chess::piece::Color::*;
///
/// // Create a chess board with the default starting position
/// let board = Board::starting_position();
/// let square = Square::from(C7);
///
/// // From white's perspective, the c7 square is taken by the opposite color
/// assert_eq!(board.status(&square, &White), SquareStatus::TakenByOpposite);
/// ```
#[derive(Debug)]
pub struct Board {
    pieces: HashMap<Square, Piece>,
    size: u8,
}

impl Board {
    /// Initialize an empty chess board
    pub fn empty(size: u8) -> Board {
        Board {
            pieces: HashMap::new(),
            size,
        }
    }

    /// Initialize a 8×8 chess board with the starting position
    pub fn starting_position() -> Board {
        // Allow for short name usage
        use Color::{Black, White};

        // Define pieces as an array
        let pieces = [
            // 1st rank
            Piece::new(A1, White, Rook),
            Piece::new(B1, White, Knight),
            Piece::new(C1, White, Bishop),
            Piece::new(D1, White, Queen),
            Piece::new(E1, White, King),
            Piece::new(F1, White, Bishop),
            Piece::new(G1, White, Knight),
            Piece::new(H1, White, Rook),
            // 2nd rank
            Piece::new(A2, White, Pawn),
            Piece::new(B2, White, Pawn),
            Piece::new(C2, White, Pawn),
            Piece::new(D2, White, Pawn),
            Piece::new(E2, White, Pawn),
            Piece::new(F2, White, Pawn),
            Piece::new(G2, White, Pawn),
            Piece::new(H2, White, Pawn),
            // 7th rank
            Piece::new(A7, Black, Pawn),
            Piece::new(B7, Black, Pawn),
            Piece::new(C7, Black, Pawn),
            Piece::new(D7, Black, Pawn),
            Piece::new(E7, Black, Pawn),
            Piece::new(F7, Black, Pawn),
            Piece::new(G7, Black, Pawn),
            Piece::new(H7, Black, Pawn),
            // 8th rank
            Piece::new(A8, Black, Rook),
            Piece::new(B8, Black, Knight),
            Piece::new(C8, Black, Bishop),
            Piece::new(D8, Black, Queen),
            Piece::new(E8, Black, King),
            Piece::new(F8, Black, Bishop),
            Piece::new(G8, Black, Knight),
            Piece::new(H8, Black, Rook),
        ];

        // Map each piece to a tuple and collect as HashMap
        let pieces = pieces
            .into_iter()
            .map(|piece| (piece.square(), piece))
            .collect();

        Board { pieces, size: 8 }
    }

    /// Return whether the color is in check
    pub fn is_in_check(&self, _color: &Color) -> bool {
        todo!()
    }

    /// Return whether the square is valid for a board of this size
    pub fn is_valid(&self, square: &Square) -> bool {
        let file = square.file();
        let rank = square.rank();

        file >= 1 && file <= self.size && rank >= 1 && rank <= self.size
    }

    /// Return whether a square is empty, taken by same color, or taken by opposite color
    pub fn status(&self, square: &Square, color: &Color) -> SquareStatus {
        match self.pieces.get(square) {
            Some(piece) => {
                if piece.color() == color {
                    SquareStatus::TakenBySame
                } else {
                    SquareStatus::TakenByOpposite
                }
            }
            None => SquareStatus::Empty,
        }
    }
}

// Consants for the Display implementation
const ROW_TOP_BORDER___: &str = "  ┏━━━┯━━━┯━━━┯━━━┯━━━┯━━━┯━━━┯━━━┓\n";
const ROW_SEPARATOR____: &str = "  ┠───┼───┼───┼───┼───┼───┼───┼───┨\n";
const ROW_BOTTOM_BORDER: &str = "  ┗━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┛\n";
const ROW_ANNOTATIONS__: &str = "    A   B   C   D   E   F   G   H\n";
const COLUMN_LEFT_BORDER_: &str = " ┃ ";
const COLUMN_SEPARATOR___: &str = " │ ";
const COLUMN_RIGHT_BORDER: &str = " ┃\n";
const SQUARE_EMPTY: &str = " ";

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.size != 8 {
            Err(fmt::Error)?;
        }

        // Write top border of the board
        write!(f, "{}", ROW_TOP_BORDER___)?;
        // Iterate over rows (ranks)
        for rank in (1..=8).rev() {
            // Write left border of a row
            write!(f, "{}{}", rank, COLUMN_LEFT_BORDER_)?;
            // Iterate over columns (files) in a row (rank)
            for file in 1..=8 {
                // Write piece if it exists
                let square = (file, rank).into();

                // Find piece on square
                match self.pieces.get(&square) {
                    Some(piece) => {
                        write!(f, "{}", piece)?;
                    }
                    None => {
                        write!(f, "{}", SQUARE_EMPTY)?;
                    }
                };

                // Write columns separator
                if file < 8 {
                    write!(f, "{}", COLUMN_SEPARATOR___)?;
                }
            }
            // Write right border of a row
            write!(f, "{}", COLUMN_RIGHT_BORDER)?;
            // Write row separator
            if rank > 1 {
                write!(f, "{}", ROW_SEPARATOR____)?;
            }
        }
        // Write bottom border of the board
        write!(f, "{}", ROW_BOTTOM_BORDER)?;

        // Write annotations of the files at the bottom
        write!(f, "{}", ROW_ANNOTATIONS__)?;

        // If we reached this point, none of the writes failed
        Ok(())
    }
}
