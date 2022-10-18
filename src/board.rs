#![allow(dead_code)]
// Absolute imports within crate
use crate::parser;
use crate::piece;
// Relative imports of sub modules
pub use castling_availability::CastlingAvailability;
pub use chess_move::{ChessMove, Moves};
pub use square::{Square, Squares};
mod castling_availability;
mod chess_move;
mod display;
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

    pub fn is_square_taken(&self, square: &Square) -> bool {
        match self.squares.get(square) {
            Some(_) => true,
            None => false,
        }
    }

    pub fn legal_moves(&self) -> Moves {
        use piece::{Color, Kind};

        // Start out with 0 moves
        let mut legal_moves = Moves::new();

        // Collect squares with pieces of the active color
        let squares_with_active_pieces = self
            .squares
            .iter()
            .filter(|(_, piece)| piece.color() == &self.active_color);

        // Iterate over active pieces to collect legal moves
        for (origin_square, piece) in squares_with_active_pieces {
            match piece.kind() {
                Kind::Bishop => {}
                Kind::Knight => {}
                Kind::King => {}
                Kind::Pawn => {
                    match piece.color() {
                        Color::White => {
                            if let Some(destination_square) = origin_square.up(1) {
                                if !self.is_square_taken(&destination_square) {
                                    legal_moves.push(ChessMove::new(origin_square, destination_square))
                                }
                            }
                        }
                        Color::Black => {
                            if let Some(destination_square) = origin_square.down(1) {
                                if !self.is_square_taken(&destination_square) {
                                    legal_moves.push(ChessMove::new(origin_square, destination_square))
                                }
                            }
                        }
                    }
                    // TODO: implement 2-square first move
                    // TODO: implement en passant
                    // TODO: implement promotion
                }
                Kind::Queen => {}
                Kind::Rook => {}
            }
        }

        legal_moves
    }
}
