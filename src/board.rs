// Absolute imports within crate
use crate::parser::{self, ParseError};
use crate::piece::{Color, Kind, Piece};
// Relative imports of sub modules
pub use castling_availability::CastlingAvailability;
pub use chess_move::{Action, ChessMove};
pub use offset::Offset;
pub use square::Square;
pub use types::{File, MoveList, PiecePlacement, Rank, SquareList};
mod castling_availability;
mod chess_move;
mod display;
mod offset;
mod square;
mod types;

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
    halfmove_clock: u8,
    fullmove_number: u8,
}
impl Board {
    // Public initializer
    pub fn new(
        piece_placement: PiecePlacement,
        active_color: Color,
        castling_availability: CastlingAvailability,
        en_passant_target: Option<Square>,
        halfmove_clock: u8,
        fullmove_number: u8,
    ) -> Board {
        Board {
            piece_placement,
            active_color,
            castling_availability,
            en_passant_target,
            halfmove_clock,
            fullmove_number,
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

        Board {
            piece_placement,
            active_color: Color::White,
            castling_availability: CastlingAvailability::default(),
            en_passant_target: None,
            halfmove_clock: 0,
            fullmove_number: 1,
        }
    }

    // Initialize a board from Forsyth–Edwards Notation
    pub fn forsyth_edwards_notation(record: &str) -> Result<Board, ParseError> {
        parser::parse_forsyth_edwards_notation(record)
    }

    // Make a move and update the board
    pub fn make_move(&mut self, chess_move: ChessMove) {
        // Get owned clones of the piece and destination square
        let piece = chess_move.piece().clone();
        let origin_square = chess_move.origin_square();
        let destination_square = chess_move.destination_square().clone();

        // Detect whether this is a pawn move
        // NOTE: captures promotion is covered under pawn move
        let is_pawn_move = *piece.kind() == Kind::Pawn;
        let is_captures = *chess_move.action() == Action::Capture;

        // Reset the halfmove clock is there was a pawn move or captures, and increment otherwise
        if is_pawn_move || is_captures {
            self.halfmove_clock = 0;
        } else {
            self.halfmove_clock += 1;
        }

        // Remove the piece from the origin square (always happens)
        self.piece_placement.remove(origin_square);

        // Update the piece placement
        match chess_move.action() {
            Action::CapturePromotion(_kind) | Action::MovePromotion(_kind) => {
                // TODO: implement update to piece kind
            }
            Action::ShortCastle => {
                // TODO: implement king and rook move
            }
            Action::LongCastle => {
                // TODO: implement king and rook move
            }
            Action::EnPassant => {
                // TODO: implement removal of opposite side pawn
            }
            Action::Move | Action::Capture => {
                // Simply place the piece on the destination square
                // If the move was a capture, the piece that was originally on the square will automatically be removed
                self.piece_placement.insert(destination_square, piece);
            }
        }

        // TODO: disable castling availability if king or rook moved

        // Update active color and update fullmove clock
        match self.active_color {
            Color::White => {
                self.active_color = Color::Black;
            }
            Color::Black => {
                self.active_color = Color::White;
                self.fullmove_number += 1;
            }
        }
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

    pub fn legal_moves(&self) -> MoveList {
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

    fn legal_bishop_moves(&self, origin_square: &Square, piece: &Piece) -> MoveList {
        // Bishop moves into 4 different directions (4 diagonal)
        let lines: Vec<SquareList> = vec![
            origin_square.squares_on_top_right_diagonal(),
            origin_square.squares_on_top_left_diagonal(),
            origin_square.squares_on_bottom_right_diagonal(),
            origin_square.squares_on_bottom_left_diagonal(),
        ];

        // Return result
        self.legal_moves_for_lines(origin_square, piece, lines)
    }

    fn legal_king_moves(&self, origin_square: &Square, piece: &Piece) -> MoveList {
        let mut moves = MoveList::new();

        // Regular king moves
        let group: SquareList = origin_square.squares_on_king_move();
        moves.append(&mut self.legal_moves_for_group(origin_square, piece, group));

        // Short castling
        if self.castling_availability.is_short_castle_available(&self.active_color) {
            // Check whether the two square right of the king are empty
            let in_between_square_are_empty = origin_square
                .squares_on_right_horizontal()
                .into_iter()
                .take(2)
                .all(|square| self.is_empty(&square));

            // TODO: verify in-between squares are not in check

            // Can short castle
            if in_between_square_are_empty {
                let destination_square = Square::new(7, origin_square.rank());
                add_move! {
                     moves <- (piece, origin_square, Action::ShortCastle, destination_square)
                }
            }
        }

        // Long castling
        if self.castling_availability.is_long_castle_available(&self.active_color) {
            // Check whether the three square left of the king are empty
            let in_between_square_are_empty = origin_square
                .squares_on_left_horizontal()
                .into_iter()
                .take(3)
                .all(|square| self.is_empty(&square));

            // TODO: verify in-between squares are not in check

            // Can long castle
            if in_between_square_are_empty {
                let destination_square = Square::new(3, origin_square.rank());
                add_move! {
                     moves <- (piece, origin_square, Action::LongCastle, destination_square)
                }
            }
        }

        moves
    }

    fn legal_knight_moves(&self, origin_square: &Square, piece: &Piece) -> MoveList {
        let group: SquareList = origin_square.squares_on_knight_moves();
        self.legal_moves_for_group(origin_square, piece, group)
    }

    fn legal_pawn_moves(&self, origin_square: &Square, piece: &Piece) -> MoveList {
        let mut moves = MoveList::new();

        // Two squares forward if the pawn hasn't moved from the starting rank yet, otherwise one square forward
        let number_of_steps = if origin_square.rank() == self.active_color.get_pawn_starting_rank() {
            2
        } else {
            1
        };

        // Get the vertical line for a specific number of steps
        let line: SquareList = match self.active_color {
            Color::White => origin_square.squares_on_up_vertical(),
            Color::Black => origin_square.squares_on_down_vertical(),
        };

        // Filter our any squares that are blocked
        for destination_square in line.into_iter().take(number_of_steps) {
            match self.is_occupied_by(&destination_square) {
                OccupiedBy::None => {
                    // If reached last rank, the pawn can promote
                    if destination_square.rank() == self.active_color.get_pawn_promotion_rank() {
                        // Iterate over all possible promotions
                        for kind in Kind::get_promotable_kinds() {
                            let action = Action::MovePromotion(kind);
                            add_move! {
                                 moves <- (piece, origin_square, action, destination_square.copy())
                            }
                        }
                    } else {
                        // Regular move forward
                        add_move! {
                             moves <- (piece, origin_square, Action::Move, destination_square)
                        }
                    }
                }
                _ => {
                    // Cannot capture or move through occupied squares, regardless of color
                    break;
                }
            }
        }

        // Right diagonal capture or en passant
        let diagonals: [SquareList; 2] = match self.active_color {
            Color::White => [
                origin_square.squares_on_top_left_diagonal(),
                origin_square.squares_on_top_right_diagonal(),
            ],
            Color::Black => [
                origin_square.squares_on_bottom_left_diagonal(),
                origin_square.squares_on_bottom_right_diagonal(),
            ],
        };

        for diagonal in diagonals {
            // Only go one square into the diagonal direction
            // NOTE: the diagonal might be empty if the piece is at the edge of the board
            for destination_square in diagonal.into_iter().take(1) {
                match self.is_occupied_by(&destination_square) {
                    OccupiedBy::OppositeColor => {
                        // If reached last rank, the pawn can promote
                        if destination_square.rank() == self.active_color.get_pawn_promotion_rank() {
                            // Iterate over all possible promotions
                            for kind in Kind::get_promotable_kinds() {
                                let action = Action::CapturePromotion(kind);
                                add_move! {
                                     moves <- (piece, origin_square, action, destination_square.clone())
                                }
                            }
                        } else {
                            // Regular diagonal captures
                            add_move! {
                                 moves <- (piece, origin_square, Action::Capture, destination_square)
                            }
                        }
                    }
                    OccupiedBy::None => match &self.en_passant_target {
                        Some(square) => {
                            // Check if the destination square matches en passant target square
                            if destination_square == *square {
                                add_move! {
                                     moves <- (piece, origin_square, Action::EnPassant, destination_square)
                                }
                            }
                        }
                        None => {
                            // No en passant possible
                        }
                    },
                    OccupiedBy::SameColor => {
                        // No move possible if occupied by same color
                    }
                }
            }
        }

        moves
    }

    fn legal_queen_moves(&self, origin_square: &Square, piece: &Piece) -> MoveList {
        // Queen moves into 8 different directions (2 vertical, 2 horizontal, and 4 diagonal)
        let lines: Vec<SquareList> = vec![
            origin_square.squares_on_up_vertical(),
            origin_square.squares_on_down_vertical(),
            origin_square.squares_on_left_horizontal(),
            origin_square.squares_on_right_horizontal(),
            origin_square.squares_on_top_right_diagonal(),
            origin_square.squares_on_top_left_diagonal(),
            origin_square.squares_on_bottom_right_diagonal(),
            origin_square.squares_on_bottom_left_diagonal(),
        ];

        // Return result
        self.legal_moves_for_lines(origin_square, piece, lines)
    }

    fn legal_rook_moves(&self, origin_square: &Square, piece: &Piece) -> MoveList {
        // Rook moves into 4 different directions(2 vertical and 2 horizontal)
        let lines: Vec<SquareList> = vec![
            origin_square.squares_on_up_vertical(),
            origin_square.squares_on_down_vertical(),
            origin_square.squares_on_right_horizontal(),
            origin_square.squares_on_left_horizontal(),
        ];

        // Return result
        self.legal_moves_for_lines(origin_square, piece, lines)
    }

    fn legal_moves_for_lines(&self, origin_square: &Square, piece: &Piece, lines: Vec<SquareList>) -> MoveList {
        let mut moves = Vec::new();

        // Add legal moves for each of those direction
        for line in lines {
            for destination_square in line.into_iter() {
                match self.is_occupied_by(&destination_square) {
                    OccupiedBy::SameColor => {
                        // Cannot take or move through own piece
                        break;
                    }
                    OccupiedBy::OppositeColor => {
                        // Can capture opposite color
                        add_move! {
                            moves <- (piece, origin_square, Action::Capture, destination_square)
                        }

                        // But cannot move any further
                        break;
                    }
                    OccupiedBy::None => {
                        // Can move to empty square and keep moving
                        add_move! {
                            moves <- (piece, origin_square, Action::Move, destination_square)
                        }
                    }
                };
            }
        }

        moves
    }

    fn legal_moves_for_group(&self, origin_square: &Square, piece: &Piece, group: SquareList) -> MoveList {
        let mut moves = Vec::new();

        for destination_square in group.into_iter() {
            match self.is_occupied_by(&destination_square) {
                OccupiedBy::SameColor => {
                    // Cannot take piece of same color
                }
                OccupiedBy::OppositeColor => {
                    // Can capture opposite color
                    add_move! {
                        moves <- (piece, origin_square, Action::Capture, destination_square)
                    }
                }
                OccupiedBy::None => {
                    // Can move to empty square
                    add_move! {
                        moves <- (piece, origin_square, Action::Move, destination_square)
                    }
                }
            };
        }

        moves
    }
}
