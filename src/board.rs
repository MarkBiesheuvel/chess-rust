// Absolute imports within crate
use crate::parser::{self, ParseError};
use crate::piece::{Color, Kind, Piece};
// Relative imports of sub modules
pub use castling_availability::CastlingAvailability;
pub use chess_move::{Action, ChessMove, Moves};
pub use offset::Offset;
pub use square::{File, Line, PiecePlacement, Rank, Square};
mod castling_availability;
mod chess_move;
mod display;
mod offset;
mod square;

// Enum to indicate whether a square is taken by no-one, by the active color or by the opposite color
enum OccupiedBy {
    None,
    SameColor,
    OppositeColor,
}

// Chess board consisting of 64 squares and indicators for various special moves
#[derive(Debug)]
pub struct Board {
    piece_placement: PiecePlacement,
    active_color: Color,
    castling_availability: CastlingAvailability,
    en_passant_target: Option<Square>,
}
impl Board {
    // Public initializer
    pub fn new(
        piece_placement: PiecePlacement,
        active_color: Color,
        castling_availability: CastlingAvailability,
        en_passant_target: Option<Square>,
    ) -> Board {
        Board {
            piece_placement,
            active_color,
            castling_availability,
            en_passant_target,
        }
    }

    // Initialize a board with the starting position
    pub fn starting_position() -> Board {
        let piece_placement = piece_placement! {
            // 1st rank
            (1, 1) => (Color::White, Kind::Rook),
            (2, 1) => (Color::White, Kind::Knight),
            (3, 1) => (Color::White, Kind::Bishop),
            (4, 1) => (Color::White, Kind::Queen),
            (5, 1) => (Color::White, Kind::King),
            (6, 1) => (Color::White, Kind::Bishop),
            (7, 1) => (Color::White, Kind::Knight),
            (8, 1) => (Color::White, Kind::Rook),
            // 2nd rank
            (1, 2) => (Color::White, Kind::Pawn),
            (2, 2) => (Color::White, Kind::Pawn),
            (3, 2) => (Color::White, Kind::Pawn),
            (4, 2) => (Color::White, Kind::Pawn),
            (5, 2) => (Color::White, Kind::Pawn),
            (6, 2) => (Color::White, Kind::Pawn),
            (7, 2) => (Color::White, Kind::Pawn),
            (8, 2) => (Color::White, Kind::Pawn),
            // 7th rank
            (1, 7) => (Color::Black, Kind::Pawn),
            (2, 7) => (Color::Black, Kind::Pawn),
            (3, 7) => (Color::Black, Kind::Pawn),
            (4, 7) => (Color::Black, Kind::Pawn),
            (5, 7) => (Color::Black, Kind::Pawn),
            (6, 7) => (Color::Black, Kind::Pawn),
            (7, 7) => (Color::Black, Kind::Pawn),
            (8, 7) => (Color::Black, Kind::Pawn),
            // 8th rank
            (1, 8) => (Color::Black, Kind::Rook),
            (2, 8) => (Color::Black, Kind::Knight),
            (3, 8) => (Color::Black, Kind::Bishop),
            (4, 8) => (Color::Black, Kind::Queen),
            (5, 8) => (Color::Black, Kind::King),
            (6, 8) => (Color::Black, Kind::Bishop),
            (7, 8) => (Color::Black, Kind::Knight),
            (8, 8) => (Color::Black, Kind::Rook),
        };
        let active_color = Color::White;
        let castling_availability = CastlingAvailability::default();
        let en_passant_target = None;

        Board {
            piece_placement,
            active_color,
            castling_availability,
            en_passant_target,
        }
    }

    // Initialize a board from Forsyth–Edwards Notation
    pub fn forsyth_edwards_notation(record: &str) -> Result<Board, ParseError> {
        parser::parse_forsyth_edwards_notation(record)
    }

    // Returns all pieces
    pub fn pieces(&self) -> Vec<&Piece> {
        self.piece_placement.values().collect()
    }

    // Returns all white pieces
    pub fn white_pieces(&self) -> Vec<&Piece> {
        self.pieces()
            .into_iter()
            .filter(|piece| piece.color() == &Color::White)
            .collect()
    }

    // Returns all black pieces
    pub fn black_pieces(&self) -> Vec<&Piece> {
        self.pieces()
            .into_iter()
            .filter(|piece| piece.color() == &Color::Black)
            .collect()
    }

    fn is_empty(&self, square: &Square) -> bool {
        !self.piece_placement.contains_key(square)
    }

    fn is_occupied_by(&self, square: &Square) -> OccupiedBy {
        match self.piece_placement.get(square) {
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
        self.piece_placement
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

    fn legal_bishop_moves<'a>(&self, origin_square: &'a Square, piece: &'a Piece) -> Moves<'a> {
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

    fn legal_king_moves<'a>(&self, origin_square: &'a Square, piece: &'a Piece) -> Moves<'a> {
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

            // TODO: verify in-between squares are not in check

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

            // TODO: verify in-between squares are not in check

            // Can short castle
            if in_between_square_are_empty {
                let destination_square = Square::new(3, 1);
                let chess_move = ChessMove::new(piece, origin_square, Action::LongCastle, destination_square);
                king_moves.push(chess_move);
            }
        }

        king_moves
    }

    fn legal_knight_moves<'a>(&self, origin_square: &'a Square, piece: &'a Piece) -> Moves<'a> {
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

    fn legal_pawn_moves<'a>(&self, origin_square: &'a Square, piece: &'a Piece) -> Moves<'a> {
        let mut pawn_moves = Moves::new();

        // Two squares forward if the pawn hasn't moved from the starting rank yet, otherwise one square forward
        let number_of_steps = if origin_square.rank() == self.active_color.get_pawn_starting_rank() {
            2
        } else {
            1
        };

        // Get the vertical line for a specific number of steps
        let line: Line = match self.active_color {
            Color::White => origin_square.up_vertical(),
            Color::Black => origin_square.down_vertical(),
        };

        // Limit the amount of squares in the line
        let line: Line = line.into_iter().take(number_of_steps).collect();

        // Filter our any squares that are blocked
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

        // Right diagonal capture or en passant
        let mut diagonals: Vec<Line> = Vec::new();

        match self.active_color {
            Color::White => {
                diagonals.push(origin_square.top_left_diagonal().into_iter().take(1).collect());
                diagonals.push(origin_square.top_right_diagonal().into_iter().take(1).collect());
            }
            Color::Black => {
                diagonals.push(origin_square.bottom_left_diagonal().into_iter().take(1).collect());
                diagonals.push(origin_square.bottom_right_diagonal().into_iter().take(1).collect());
            }
        };
        for diagonal in diagonals {
            for destination_square in diagonal {
                match self.is_occupied_by(&destination_square) {
                    OccupiedBy::OppositeColor => {
                        // Diagonal captures
                        let chess_move = ChessMove::new(piece, origin_square, Action::Capture, destination_square);
                        pawn_moves.push(chess_move);
                    }
                    OccupiedBy::None => {
                        // Check if en passant is available
                        if let Some(square) = &self.en_passant_target {
                            // Check if the destination square matches en passant target square
                            if destination_square == *square {
                                let chess_move =
                                    ChessMove::new(piece, origin_square, Action::EnPassant, destination_square);
                                pawn_moves.push(chess_move);
                            }
                        }
                    }
                    OccupiedBy::SameColor => {
                        // No move possible if occupied by same color
                    }
                }
            }
        }

        // TODO: implement promotion

        pawn_moves
    }

    fn legal_queen_moves<'a>(&self, origin_square: &'a Square, piece: &'a Piece) -> Moves<'a> {
        let mut queen_moves = Moves::new();

        let lines: [Vec<Square>; 8] = [
            origin_square.up_vertical(),
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

    fn legal_rook_moves<'a>(&self, origin_square: &'a Square, piece: &'a Piece) -> Moves<'a> {
        let mut rook_moves = Moves::new();

        let lines: [Vec<Square>; 4] = [
            origin_square.up_vertical(),
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
