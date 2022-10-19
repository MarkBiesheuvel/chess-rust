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

// All permutations of 1,-1 and 2,-2 (in other words L shapes, in other words knight moves)
const L_SHAPES: [(i8, i8); 8] = [(2, 1), (2, -1), (-2, 1), (-2, -1), (1, 2), (1, -2), (-1, 2), (-1, -2)];

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
            // Get moves based on piece kind
            let mut moves = match piece.kind() {
                Kind::Bishop => self.legal_bishop_moves(square, piece),
                Kind::Knight => self.legal_knight_moves(square, piece),
                Kind::King => self.legal_king_moves(square, piece),
                Kind::Pawn => self.legal_pawn_moves(square, piece),
                Kind::Queen => self.legal_queen_moves(square, piece),
                Kind::Rook => self.legal_rook_moves(square, piece),
            };

            // Add moves to list
            legal_moves.append(&mut moves);
        }

        // TODO: remove any moves that leave the king in check

        legal_moves
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

    fn legal_king_moves<'a>(&self, _origin_square: &'a Square, _piece: &'a piece::Piece) -> Moves<'a> {
        Moves::new()
        // TODO: implement simple King moves
        // TODO: implement castling
    }

    fn legal_knight_moves<'a>(&self, origin_square: &'a Square, piece: &'a piece::Piece) -> Moves<'a> {
        let mut knight_moves = Moves::new();

        for (file_offset, rank_offset) in L_SHAPES {
            // Verify that the knight is not jumping of the board
            if origin_square.is_valid_offset(file_offset, rank_offset) {
                // Calculate the destination square
                let destination_square = origin_square.copy_with_offset(file_offset, rank_offset);
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
