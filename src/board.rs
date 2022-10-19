#![allow(dead_code)]
// Absolute imports within crate
use crate::parser;
use crate::piece;
// Relative imports of sub modules
pub use castling_availability::CastlingAvailability;
pub use chess_move::{Action, ChessMove, Moves};
pub use square::{Square, Squares};
mod castling_availability;
mod chess_move;
mod display;
mod square;

// Standard starting position for a game of chess
// Since FEN break the spell checker, turn it of for the next line - cspell:disable-next
const STARTING_POSITION: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

// Enum to indicate whether a square is taken by no-one, by the active color or by the opposite color
pub enum OccupiedBy {
    None,
    SameColor,
    OppositeColor,
}

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

    pub fn is_occupied_by(&self, square: &Square) -> OccupiedBy {
        match self.squares.get(square) {
            Some(piece) => {
                if *piece.color() == self.active_color {
                    OccupiedBy::SameColor
                } else {
                    OccupiedBy::OppositeColor
                }
            }
            None => OccupiedBy::None,
        }
    }

    pub fn legal_moves(&self) -> Moves {
        use piece::Kind;

        // Start out with 0 moves
        let mut legal_moves = Moves::new();

        // Collect squares with pieces of the active color
        let squares_with_active_pieces = self
            .squares
            .iter()
            .filter(|(_, piece)| piece.color() == &self.active_color);

        // Iterate over active pieces to collect legal moves
        for (square, piece) in squares_with_active_pieces {
            match piece.kind() {
                Kind::Bishop => {}
                Kind::Knight => {}
                Kind::King => {}
                Kind::Pawn => {
                    let mut pawn_moves = self.legal_pawn_moves(square, piece);
                    legal_moves.append(&mut pawn_moves);
                }
                Kind::Queen => {}
                Kind::Rook => {
                    let mut rook_moves = self.legal_rook_moves(square, piece);
                    legal_moves.append(&mut rook_moves);
                }
            }
        }

        // TODO: remove any moves that leave the king in check

        legal_moves
    }

    fn legal_pawn_moves<'a>(&self, origin_square: &'a Square, piece: &'a piece::Piece) -> Moves<'a> {
        use piece::Color;

        let mut pawn_moves = Moves::new();

        // List of forward pawn moves, length of just one or two
        let mut line = Vec::new();
        match self.active_color {
            Color::White => {
                // One square forward
                line.push(origin_square.up(1));
                // Two squares forward
                if origin_square.rank() == 2 {
                    line.push(origin_square.up(2));
                }
            }
            Color::Black => {
                // One square forward
                line.push(origin_square.down(1));
                // Two squares forward
                if origin_square.rank() == 7 {
                    line.push(origin_square.down(2));
                }
            }
        };

        for destination_square in line {
            match self.is_occupied_by(&destination_square) {
                OccupiedBy::None => {
                    let chess_move = ChessMove::new(piece, origin_square, Action::Move, destination_square);
                    pawn_moves.push(chess_move);
                }
                _ => {
                    // Cannot capture or move through occupied squares, regardless of color
                    break;
                }
            }
        }

        // TODO: implement diagonal captures
        // TODO: implement en passant
        // TODO: implement promotion
        pawn_moves
    }

    fn legal_rook_moves<'a>(&self, origin_square: &'a Square, piece: &'a piece::Piece) -> Moves<'a> {
        let mut rook_moves = Moves::new();

        let lines: [Vec<Square>; 4] = [
            origin_square.up_vector(),
            origin_square.down_vector(),
            origin_square.right_vector(),
            origin_square.left_vector(),
        ];

        for line in lines {
            for destination_square in line {
                match self.is_occupied_by(&destination_square) {
                    OccupiedBy::SameColor => {
                        // Cannot take or move through own piece
                        break;
                    }
                    OccupiedBy::OppositeColor => {
                        // Can capture opposite color
                        let chess_move = ChessMove::new(piece, origin_square, Action::Capture, destination_square);
                        rook_moves.push(chess_move);

                        // But cannot move any further
                        break;
                    }
                    OccupiedBy::None => {
                        // Can move to empty square and keep moving
                        let chess_move = ChessMove::new(piece, origin_square, Action::Move, destination_square);
                        rook_moves.push(chess_move);
                    }
                };
            }
        }

        rook_moves
    }
}
