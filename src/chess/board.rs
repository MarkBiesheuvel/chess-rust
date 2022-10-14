#![allow(dead_code)]
use super::piece::{Piece, PieceColor};
use std::fmt;

const STARTING_POSITION: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

#[derive(Clone, Copy, Debug)]
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

// Chess board consisting of 64 squares and indicators for various special moves
#[derive(Debug)]
pub struct Board {
    squares: [[Square; 8]; 8],
    castling_white_kingside_is_available: bool,
    castling_white_queenside_is_available: bool,
    castling_black_kingside_is_available: bool,
    castling_black_queenside_is_available: bool,
    active_color: PieceColor,
}
impl Board {
    // Initialize a board with the starting position
    pub fn starting_position() -> Board {
        Board::fen(STARTING_POSITION)
    }

    // Initialize a board from Forsyth–Edwards Notation
    pub fn fen(specification: &str) -> Board {
        // Start with empty squares
        let mut squares = [[Square::Empty; 8]; 8];

        // Go from highest rank to lowest, and from lowest file to highest
        let mut rank: usize = 7;
        let mut file: usize = 0;

        // Convert &str to std::str::Chars
        let mut characters = specification.chars();

        // Loop over characters until a space is found
        loop {
            match characters.next() {
                Some(character) => match character {
                    // Space indicates end of the piece specification, break out of loop
                    ' ' => {
                        break;
                    }
                    // Slash indicates end of current rank, go down one rank and reset file
                    '/' => {
                        file = 0;
                        rank -= 1;
                    }
                    // A number indicates the amount of empty squares, increase file by that number
                    '1'..='8' => {
                        file += character.to_digit(10).unwrap() as usize;
                    }
                    // Any character implies a piece on the current square, so create a new piece and increase file
                    _ => {
                        squares[rank][file] = Square::Taken(Piece::fen(character));
                        file += 1;
                    }
                },
                // End of specification reached too early
                None => {
                    panic!("Specification is too short");
                }
            }
        }

        // Detect whether it is the turn of black or white
        let active_color;
        match characters.next() {
            Some(character) => match character {
                // Blacks turn to move
                'b' => {
                    active_color = PieceColor::Black;
                }
                // Whites turn to move
                'w' => {
                    active_color = PieceColor::White;
                }
                // Invalid character
                _ => {
                    panic!("Invalid active color");
                }
            },
            // End of specification reached too early
            None => {
                panic!("Specification is too short");
            }
        }

        // This character should just be a space
        characters.next();

        // Detect whether castling is still allowed
        let mut castling_white_kingside_is_available = false;
        let mut castling_white_queenside_is_available = false;
        let mut castling_black_kingside_is_available = false;
        let mut castling_black_queenside_is_available = false;

        loop {
            // Loop over characters until a space is found
            match characters.next() {
                Some(character) => match character {
                    // Space indicates end of the piece specification, break out of loop
                    ' ' => {
                        break;
                    }
                    // A dash indicates that no player can castle anymore
                    '-' => {
                        continue;
                    }
                    // Letter K indicates white can castle kingside
                    'K' => {
                        castling_white_kingside_is_available = true;
                    }
                    // Letter Q indicates white can castle queenside
                    'Q' => {
                        castling_white_queenside_is_available = true;
                    }
                    // Letter k indicates black can castle kingside
                    'k' => {
                        castling_black_kingside_is_available = true;
                    }
                    // Letter q indicates black can castle queenside
                    'q' => {
                        castling_black_queenside_is_available = true;
                    }
                    // Invalid character
                    _ => {
                        panic!("Invalid castling availability: {}", character);
                    }
                },
                // End of specification reached too early
                None => {
                    panic!("Specification is too short");
                }
            }
        }

        // Detect possible en passant target square
        match characters.next() {
            Some(first_character) => match first_character {
                // First character should be a file
                'a'..='h' => match characters.next() {
                    Some(second_character) => match second_character {
                        // Second character should be a rank
                        '1'..='8' => {
                            // TODO: store this location
                            println!("{}{}", first_character, second_character);
                        }
                        _ => {
                            panic!("Invalid rank for en passant");
                        }
                    },
                    // End of specification reached too early
                    None => {
                        panic!("Specification is too short");
                    }
                },
                '-' => {
                    // No en passant target square, no problem
                }
                _ => {
                    panic!("Invalid file for en passant");
                }
            },
            // End of specification reached too early
            None => {
                panic!("Specification is too short");
            }
        }

        Board {
            squares,
            active_color,
            castling_white_kingside_is_available,
            castling_white_queenside_is_available,
            castling_black_kingside_is_available,
            castling_black_queenside_is_available,
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
