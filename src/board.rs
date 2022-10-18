#![allow(dead_code)]
// External imports
use std::fmt;
// Absolute imports within crate
use crate::parser;
use crate::piece;
// Relative imports of sub modules
pub use castling_availability::CastlingAvailability;
pub use square::{Square, Squares};
mod castling_availability;
mod square;

// Standard starting position for a game of chess
// Since FEN break the spell checker, turn it of for the next line - cspell:disable-next
const STARTING_POSITION: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

// Chess board consisting of 64 squares and indicators for various special moves
#[derive(Debug)]
pub struct Board {
    squares: Squares,
    active_color: piece::Color,
    castling_availability: CastlingAvailability,
}
impl Board {
    // Public initializer
    pub fn new(squares: Squares, active_color: piece::Color, castling_availability: CastlingAvailability) -> Board {
        Board {
            squares,
            active_color,
            castling_availability,
        }
    }

    // Initialize a board with the starting position
    pub fn starting_position() -> Board {
        parser::parse_forsyth_edwards_notation(STARTING_POSITION).expect("Hardcoded FEN should not give parser error")
    }

    // Initialize a board from Forsyth–Edwards Notation
    pub fn forsyth_edwards_notation(record: &str) -> Result<Board, parser::ParseError> {
        parser::parse_forsyth_edwards_notation(record)
    }

    // Returns all pieces as an Iterator
    pub fn pieces(&self) -> impl Iterator<Item = &piece::Piece> {
        self.squares.iter().map(|(_, piece)| piece)
    }

    // Returns all white pieces as an Iterator
    pub fn white_pieces(&self) -> impl Iterator<Item = &piece::Piece> {
        self.pieces().filter(|piece| piece.color() == &piece::Color::White)
    }

    // Returns all white pieces as an Iterator
    pub fn black_pieces(&self) -> impl Iterator<Item = &piece::Piece> {
        self.pieces().filter(|piece| piece.color() == &piece::Color::Black)
    }
}
impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Write top border of the board
        write!(f, "┏━━━┯━━━┯━━━┯━━━┯━━━┯━━━┯━━━┯━━━┓\n")?;
        // Iterate over rows (ranks)
        for rank in (0..8).rev() {
            // Write left border of a row
            write!(f, "┃ ")?;
            // Iterate over columns (files) in a row (rank)
            for file in 0..8 {
                // Write piece if it exists
                let square = Square::new(file, rank);
                match self.squares.get(&square) {
                    Some(piece) => {
                        write!(f, "{}", piece)?;
                    }
                    None => {
                        write!(f, " ")?;
                    }
                };
                // Write columns seperator
                if file < 7 {
                    write!(f, " │ ")?;
                }
            }
            // Write right border of a row
            write!(f, " ┃\n")?;
            // Write row seperator
            if rank > 0 {
                write!(f, "┠───┼───┼───┼───┼───┼───┼───┼───┨\n")?;
            }
        }
        // Write bottom border of the board
        write!(f, "┗━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┷━━━┛\n")?;

        // If we reached this point, none of the writes failed
        Ok(())
    }
}
