#![allow(dead_code)]
use super::parser::{parse_forsyth_edwards_notation, FenParserError};
use super::piece::{Piece, PieceColor};
use std::fmt;

// Standard starting position for a game of chess
// Since FEN break the spell checker, turn it of for the next line - cspell:disable-next
const STARTING_POSITION: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

// Type for squares of the chess board
#[derive(Clone, Copy, Debug)]
pub enum Square {
    Taken(Piece),
    Empty,
}
impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Square::Taken(piece) => {
                write!(f, "{}", piece)?;
            }
            Square::Empty => {
                write!(f, " ")?;
            }
        }

        Ok(())
    }
}

// Custom type alias for 8×8 board of squares
pub type Squares = [[Square; 8]; 8];

// Type to indicate whether castling is available for the either player on either king- or queenside
#[derive(Debug)]
pub struct CastlingAvailability {
    white_kingside: bool,
    white_queenside: bool,
    black_kingside: bool,
    black_queenside: bool,
}
impl CastlingAvailability {
    pub fn new(
        white_kingside: bool,
        white_queenside: bool,
        black_kingside: bool,
        black_queenside: bool,
    ) -> CastlingAvailability {
        CastlingAvailability {
            white_kingside,
            white_queenside,
            black_kingside,
            black_queenside,
        }
    }
}

// Chess board consisting of 64 squares and indicators for various special moves
#[derive(Debug)]
pub struct Board {
    squares: Squares,
    active_color: PieceColor,
    castling_availability: CastlingAvailability,
}
impl Board {
    // Public initializer
    pub fn new(squares: Squares, active_color: PieceColor, castling_availability: CastlingAvailability) -> Board {
        Board {
            squares,
            active_color,
            castling_availability,
        }
    }

    // Initialize a board with the starting position
    pub fn starting_position() -> Board {
        parse_forsyth_edwards_notation(STARTING_POSITION).expect("Hardcoded FEN should not give parser error")
    }

    // Initialize a board from Forsyth–Edwards Notation
    pub fn forsyth_edwards_notation(record: &str) -> Result<Board, FenParserError> {
        parse_forsyth_edwards_notation(record)
    }
}
impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "┏━━━┯━━━┯━━━┯━━━┯━━━┯━━━┯━━━┯━━━┓\n")?;

        for rank in (0..8).rev() {
            let row = self.squares[rank];

            write!(
                f,
                "┃ {} │ {} │ {} │ {} │ {} │ {} │ {} │ {} ┃\n",
                row[0], row[1], row[2], row[3], row[4], row[5], row[6], row[7],
            )?;

            if rank > 0 {
                write!(f, "┠───┼───┼───┼───┼───┼───┼───┼───┨\n")?;
            }
        }

        write!(f, "┗━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┛\n")?;

        Ok(())
    }
}
