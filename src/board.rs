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

    pub fn is_empty(&self, square: &Square) -> bool {
        !self.squares.contains_key(square)
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

        self.squares
            .iter()
            .filter(|(_, piece)| piece.color() == &self.active_color)
            .flat_map(|(square, piece)| match piece.kind() {
                Kind::Bishop => self.legal_bishop_moves(square, piece),
                Kind::Knight => self.legal_knight_moves(square, piece),
                Kind::King => self.legal_king_moves(square, piece),
                Kind::Pawn => self.legal_pawn_moves(square, piece),
                Kind::Queen => self.legal_queen_moves(square, piece),
                Kind::Rook => self.legal_rook_moves(square, piece),
            })
            // TODO: .filter() any moves that leave or bring the king in check
            .collect()
    }

    fn legal_bishop_moves<'a>(&self, origin_square: &'a Square, piece: &'a piece::Piece) -> Moves<'a> {
        let mut bishop_moves = Moves::new();

        let lines: [Vec<Square>; 4] = [
            origin_square.top_right_diagonal(),
            origin_square.top_left_diagonal(),
            origin_square.bottom_right_diagonal(),
            origin_square.bottom_left_diagonal(),
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
                        bishop_moves.push(chess_move);

                        // But cannot move any further
                        break;
                    }
                    OccupiedBy::None => {
                        // Can move to empty square and keep moving
                        let chess_move = ChessMove::new(piece, origin_square, Action::Move, destination_square);
                        bishop_moves.push(chess_move);
                    }
                };
            }
        }

        bishop_moves
    }

    fn legal_king_moves<'a>(&self, origin_square: &'a Square, piece: &'a piece::Piece) -> Moves<'a> {
        let mut king_moves = Moves::new();

        // Regular king moves
        for destination_square in origin_square.king_moves() {
            match self.is_occupied_by(&destination_square) {
                OccupiedBy::SameColor => {
                    // Cannot take piece
                }
                OccupiedBy::OppositeColor => {
                    // Can capture opposite color
                    let chess_move = ChessMove::new(piece, origin_square, Action::Capture, destination_square);
                    king_moves.push(chess_move);
                }
                OccupiedBy::None => {
                    // Can move to empty square
                    let chess_move = ChessMove::new(piece, origin_square, Action::Move, destination_square);
                    king_moves.push(chess_move);
                }
            };
        }

        // Short castling
        if self.castling_availability.is_short_castle_available(&self.active_color) {
            // Check whether the two square right of the king are empty
            let in_between_square_are_empty = origin_square
                .right_horizontal()
                .iter()
                .take(2)
                .all(|square| self.is_empty(square));

            // TODO: verify in-between sqaures are not in check

            // Can short castle
            if in_between_square_are_empty {
                let destination_square = Square::new(7, 1);
                let chess_move = ChessMove::new(piece, origin_square, Action::ShortCastle, destination_square);
                king_moves.push(chess_move);
            }
        }

        // Long castling
        if self.castling_availability.is_long_castle_available(&self.active_color) {
            // Check whether the three square left of the king are empty
            let in_between_square_are_empty = origin_square
                .left_horizontal()
                .iter()
                .take(3)
                .all(|square| self.is_empty(square));

            // TODO: verify in-between sqaures are not in check

            // Can short castle
            if in_between_square_are_empty {
                let destination_square = Square::new(3, 1);
                let chess_move = ChessMove::new(piece, origin_square, Action::LongCastle, destination_square);
                king_moves.push(chess_move);
            }
        }

        king_moves
    }

    fn legal_knight_moves<'a>(&self, origin_square: &'a Square, piece: &'a piece::Piece) -> Moves<'a> {
        let mut knight_moves = Moves::new();

        for destination_square in origin_square.knight_moves() {
            match self.is_occupied_by(&destination_square) {
                OccupiedBy::SameColor => {
                    // Cannot take piece
                }
                OccupiedBy::OppositeColor => {
                    // Can capture opposite color
                    let chess_move = ChessMove::new(piece, origin_square, Action::Capture, destination_square);
                    knight_moves.push(chess_move);
                }
                OccupiedBy::None => {
                    // Can move to empty square
                    let chess_move = ChessMove::new(piece, origin_square, Action::Move, destination_square);
                    knight_moves.push(chess_move);
                }
            };
        }

        knight_moves
    }

    fn legal_pawn_moves<'a>(&self, origin_square: &'a Square, piece: &'a piece::Piece) -> Moves<'a> {
        use piece::Color;

        let mut pawn_moves = Moves::new();

        // List of forward pawn moves, length of just one or two
        let mut line = Vec::new();
        match self.active_color {
            Color::White => {
                // One square forward
                line.push(origin_square.copy_with_offset(0, 1));
                // Two squares forward
                if origin_square.rank() == 2 {
                    line.push(origin_square.copy_with_offset(0, 2));
                }
            }
            Color::Black => {
                // One square forward
                line.push(origin_square.copy_with_offset(0, -1));
                // Two squares forward
                if origin_square.rank() == 7 {
                    line.push(origin_square.copy_with_offset(0, -2));
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

    fn legal_queen_moves<'a>(&self, origin_square: &'a Square, piece: &'a piece::Piece) -> Moves<'a> {
        let mut queen_moves = Moves::new();

        let lines: [Vec<Square>; 8] = [
            origin_square.top_vertical(),
            origin_square.down_vertical(),
            origin_square.left_horizontal(),
            origin_square.right_horizontal(),
            origin_square.top_right_diagonal(),
            origin_square.top_left_diagonal(),
            origin_square.bottom_right_diagonal(),
            origin_square.bottom_left_diagonal(),
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
                        queen_moves.push(chess_move);

                        // But cannot move any further
                        break;
                    }
                    OccupiedBy::None => {
                        // Can move to empty square and keep moving
                        let chess_move = ChessMove::new(piece, origin_square, Action::Move, destination_square);
                        queen_moves.push(chess_move);
                    }
                };
            }
        }

        queen_moves
    }

    fn legal_rook_moves<'a>(&self, origin_square: &'a Square, piece: &'a piece::Piece) -> Moves<'a> {
        let mut rook_moves = Moves::new();

        let lines: [Vec<Square>; 4] = [
            origin_square.top_vertical(),
            origin_square.down_vertical(),
            origin_square.left_horizontal(),
            origin_square.right_horizontal(),
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
