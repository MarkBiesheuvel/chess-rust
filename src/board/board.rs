// External imports
use std::collections::HashMap;
use std::fmt;

// Imports from crate
use crate::piece::{behavior::*, Color, Piece};

// Imports from super
use super::{Square, SquareStatus};

/// Chess board consisting of N×N squares
///
/// ## Examples
/// ```
/// use std::str::FromStr;
/// use chess::board::{Board, Square, SquareStatus};
/// use chess::piece::Color::*;
///
/// // Create a chess board with the default starting position
/// let board = Board::starting_position();
/// let square = Square::from_str("c7")?;
///
/// // From white's perspective, the c7 square is taken by the opposite color
/// assert_eq!(board.status(&square, &White), SquareStatus::TakenByOpposite);
/// #
/// # Ok::<(), Box<dyn std::error::Error>>(())
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
            Piece::new((1, 1), White, Rook),
            Piece::new((2, 1), White, Knight),
            Piece::new((3, 1), White, Bishop),
            Piece::new((4, 1), White, Queen),
            Piece::new((5, 1), White, King),
            Piece::new((6, 1), White, Bishop),
            Piece::new((7, 1), White, Knight),
            Piece::new((8, 1), White, Rook),
            // 2nd rank
            Piece::new((1, 2), White, Pawn),
            Piece::new((2, 2), White, Pawn),
            Piece::new((3, 2), White, Pawn),
            Piece::new((4, 2), White, Pawn),
            Piece::new((5, 2), White, Pawn),
            Piece::new((6, 2), White, Pawn),
            Piece::new((7, 2), White, Pawn),
            Piece::new((8, 2), White, Pawn),
            // 7th rank
            Piece::new((1, 7), Black, Pawn),
            Piece::new((2, 7), Black, Pawn),
            Piece::new((3, 7), Black, Pawn),
            Piece::new((4, 7), Black, Pawn),
            Piece::new((5, 7), Black, Pawn),
            Piece::new((6, 7), Black, Pawn),
            Piece::new((7, 7), Black, Pawn),
            Piece::new((8, 7), Black, Pawn),
            // 8th rank
            Piece::new((1, 8), Black, Rook),
            Piece::new((2, 8), Black, Knight),
            Piece::new((3, 8), Black, Bishop),
            Piece::new((4, 8), Black, Queen),
            Piece::new((5, 8), Black, King),
            Piece::new((6, 8), Black, Bishop),
            Piece::new((7, 8), Black, Knight),
            Piece::new((8, 8), Black, Rook),
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
