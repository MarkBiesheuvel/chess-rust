#![allow(dead_code)]
use super::piece::{Piece, PieceColor};
use std::fmt;

#[derive(Copy, Clone)]
enum Square {
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

//
pub struct Board {
    squares: [[Square; 8]; 8],
    castling_white_king_side_is_available: bool,
    castling_white_queen_side_is_available: bool,
    castling_black_king_side_is_available: bool,
    castling_black_queen_side_is_available: bool,
    active_color: PieceColor,
}
impl Board {
    // Initialize a board from Forsyth–Edwards Notation
    pub fn fen(specification: &str) -> Board {
        // Start with empty squares
        let mut squares = [[Square::Empty; 8]; 8];

        let mut rank: usize = 7;
        let mut file: usize = 0;

        let mut characters = specification.chars();

        loop {
            match characters.next() {
                Some(character) => {
                    match character {
                        ' ' => {
                            break;
                        }
                        '/' => {
                            file = 0;
                            rank -= 1;
                        }
                        '1'..='8' => {
                            file += character.to_digit(10).unwrap() as usize;
                        }
                        _ => {
                            squares[rank][file] = Square::Taken(Piece::fen(character));
                            file += 1;
                        }
                    }
                }
                None => {
                    panic!("Not enough squares");
                }
            }
        }

        // TODO: write as a function
        let active_color;
        match characters.next() {
            Some(character) => {
                match character {
                    'b' => {
                        active_color = PieceColor::Black;
                    }
                    'w' => {
                        active_color = PieceColor::White;
                    }
                    _ => {
                        panic!("Invalid active color");
                    }
                }
            }
            None => {
                panic!("Missing active color");
            }
        }

        // TODO: get other properties from FEN notation

        Board {
            squares,
            active_color,
            castling_white_king_side_is_available: true,
            castling_white_queen_side_is_available: true,
            castling_black_king_side_is_available: true,
            castling_black_queen_side_is_available: true,
        }
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
