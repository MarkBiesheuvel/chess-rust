// Absolute imports within crate
use crate::parser::{self, ParseError};
use crate::piece::{Color, Kind, Piece};
// Relative imports of sub modules
use board_error::BoardError;
pub use offset::Offset;
pub use square::Square;
pub use types::{File, MoveList, PiecePlacement, Rank, SquareList};
mod board_error;
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
    halfmove_clock: u16,
    fullmove_number: u16,
}
impl Board {
    // Public initializer
    pub fn new(
        piece_placement: PiecePlacement, active_color: Color, castling_availability: CastlingAvailability,
        en_passant_target: Option<Square>, halfmove_clock: u16, fullmove_number: u16,
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

    pub fn active_color(&self) -> &Color {
        &self.active_color
    }

    // Initialize a board from Forsyth–Edwards Notation
    pub fn forsyth_edwards_notation(record: &str) -> Result<Board, ParseError> {
        parser::parse_forsyth_edwards_notation(record)
    }

    // Make a move and update the board
    pub fn make_move(&mut self, chess_move: ChessMove) -> Result<(), BoardError> {
        // Get owned clones of the piece and destination square
        let mut piece = chess_move.piece().clone();
        let origin_square = chess_move.origin_square();
        let destination_square = chess_move.destination_square().clone();

        // Get a borrowed reference to the color
        let color = &self.active_color;

        // Get first rank for active color
        let first_rank = color.get_first_rank();

        // Reset the halfmove clock is there was a pawn move or captures, and increment otherwise
        if *piece.kind() == Kind::Pawn || *chess_move.action() == Action::Capture {
            self.halfmove_clock = 0;
        } else {
            self.halfmove_clock += 1;
        }

        // Update the castling availability
        match piece.kind() {
            Kind::King => {
                // Disable castling on both sides when king moves
                self.castling_availability.disable_both(color);
            }
            Kind::Rook => {
                // NOTE: it does not matter if the rook has or hasn't moved.
                // If it was moved back to a1/a8/h1/h8, it would have left the square before
                // and castling would already be disabled.
                if origin_square.rank() == first_rank {
                    if origin_square.file() == 8 {
                        // Disable king side castling if the rook on the H-file moves
                        self.castling_availability.disable_kingside(color);
                    } else if origin_square.file() == 1 {
                        // Disable queen side castling if the rook on the A-file moves
                        self.castling_availability.disable_queenside(color);
                    }
                }
            }
            _ => {
                // Irrelevant for castling
            }
        }

        // Remove the piece from the origin square (always happens)
        let origin_piece = self.piece_placement.remove(origin_square);

        // Validation
        match origin_piece {
            Some(_) => {
                // TODO: match `origin_piece` with piece in `chess_move`
            }
            None => {
                // There was no piece on the square specified in the move
                return Err(BoardError::PieceMissing(origin_square.clone()));
            }
        }

        // Update the rest of the piece placement based on the type of move
        match chess_move.action() {
            Action::CapturePromotion(kind) | Action::MovePromotion(kind) => {
                // Only pawns can promote
                if *piece.kind() != Kind::Pawn {
                    return Err(BoardError::InvalidPromotionPawn(piece.kind().clone()));
                }

                // Promote the piece to the new kind
                piece.promote(kind.clone());

                // And move it to the destination square
                self.piece_placement.insert(destination_square, piece);
            }
            Action::ShortCastle => {
                // Only king can castle
                if *piece.kind() != Kind::King {
                    return Err(BoardError::InvalidCastlingKing(piece.kind().clone()));
                }

                // Move the king to the destination square
                self.piece_placement.insert(destination_square, piece);

                // Move rook from H-file, ...
                let rook_origin_square = Square::new(8, first_rank);
                let rook_piece = self
                    .piece_placement
                    .remove(&rook_origin_square)
                    .ok_or(BoardError::PieceMissing(rook_origin_square))?;

                // Only rook can castle
                if *rook_piece.kind() != Kind::Rook {
                    return Err(BoardError::InvalidCastlingRook(rook_piece.kind().clone()));
                }

                // ... to F-file
                let rook_destination_square = Square::new(6, first_rank);
                self.piece_placement
                    .insert(rook_destination_square, rook_piece);
            }
            Action::LongCastle => {
                // Only king can castle
                if *piece.kind() != Kind::King {
                    return Err(BoardError::InvalidCastlingKing(piece.kind().clone()));
                }

                // Move the king to the destination square
                self.piece_placement.insert(destination_square, piece);

                // Move rook from A-file, ...
                let rook_origin_square = Square::new(1, first_rank);
                let rook_piece = self
                    .piece_placement
                    .remove(&rook_origin_square)
                    .ok_or(BoardError::PieceMissing(rook_origin_square))?;

                // Only rook can castle
                if *rook_piece.kind() != Kind::Rook {
                    return Err(BoardError::InvalidCastlingRook(rook_piece.kind().clone()));
                }

                // ... to D-file
                let rook_destination_square = Square::new(4, first_rank);
                self.piece_placement
                    .insert(rook_destination_square, rook_piece);
            }
            Action::EnPassant => {
                // ASSERT: only pawns can promote
                if *piece.kind() != Kind::Pawn {
                    return Err(BoardError::InvalidEnPassantPawn(piece.kind().clone()));
                }

                // Move the pawn to the destination square
                self.piece_placement.insert(destination_square, piece);

                // TODO: calculate the square of the other pawn and remove it
            }
            Action::Move | Action::Capture => {
                // Simply place the piece on the destination square
                // If the move was a capture, the piece that was originally on the square will automatically be removed
                self.piece_placement.insert(destination_square, piece);

                // TODO: validate that a piece has been captured
            }
        }

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

        Ok(())
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

    fn is_occupied_by(&self, square: &Square, active_color: &Color) -> OccupiedBy {
        match self.piece_placement.get(square) {
            Some(piece) => {
                if piece.color() == active_color {
                    OccupiedBy::SameColor
                } else {
                    OccupiedBy::OppositeColor
                }
            }
            None => OccupiedBy::None,
        }
    }

    pub fn is_in_check(&self, active_color: &Color) -> bool {
        // Get color of opposite player
        let opposite_color = match active_color {
            Color::Black => &Color::White,
            Color::White => &Color::Black,
        };

        // Get all pieces of the opponent
        let opposite_pieces = self
            .piece_placement
            .iter()
            .filter(|(_, piece)| piece.color() == opposite_color);

        // Determine which chess moves the opponent can make
        let mut opposite_moves =
            opposite_pieces.flat_map(|(square, piece)| self.legal_piece_moves(square, piece, opposite_color));

        // Get the king of the active color
        let king = self
            .piece_placement
            .iter()
            .filter(|(_, piece)| piece.color() == active_color && piece.kind() == &Kind::King)
            .next();

        match king {
            Some((king_square, _piece)) => {
                // Return true if any of the chess moves of the opponent could (theoretically) capture the kind
                opposite_moves.any(|chess_move| chess_move.destination_square() == king_square)
            }
            None => {
                // No king, so technically not in check
                // This would be an invalid position, but it's not up to this function to raise this issue
                false
            }
        }
    }

    fn new_move(&self, piece: &Piece, origin_square: &Square, action: Action, destination_square: Square) -> ChessMove {
        // Clone `piece` and `origin_square` so they are independent of Board
        let piece = piece.clone();
        let origin_square = origin_square.clone();

        // TODO: Calculate whether this move will be checkmate, check or nothing
        let status = crate::board::chess_move::MoveStatus::None;

        ChessMove::new(piece, origin_square, action, destination_square, status)
    }

    pub fn legal_moves(&self) -> MoveList {
        self.piece_placement
            .iter()
            .filter(|(_, piece)| piece.color() == &self.active_color)
            .flat_map(|(square, piece)| self.legal_piece_moves(square, piece, &self.active_color))
            // TODO: .filter() any moves that leave or bring the king in check
            .collect()
    }

    fn legal_piece_moves(&self, square: &Square, piece: &Piece, active_color: &Color) -> MoveList {
        match piece.kind() {
            Kind::Bishop => self.legal_bishop_moves(square, piece, active_color),
            Kind::Knight => self.legal_knight_moves(square, piece, active_color),
            Kind::King => self.legal_king_moves(square, piece, active_color),
            Kind::Pawn => self.legal_pawn_moves(square, piece, active_color),
            Kind::Queen => self.legal_queen_moves(square, piece, active_color),
            Kind::Rook => self.legal_rook_moves(square, piece, active_color),
        }
    }

    fn legal_bishop_moves(&self, origin_square: &Square, piece: &Piece, active_color: &Color) -> MoveList {
        // Bishop moves into 4 different directions (4 diagonal)
        let lines: Vec<SquareList> = vec![
            origin_square.squares_on_top_right_diagonal(),
            origin_square.squares_on_top_left_diagonal(),
            origin_square.squares_on_bottom_right_diagonal(),
            origin_square.squares_on_bottom_left_diagonal(),
        ];

        // Return result
        self.legal_moves_for_lines(origin_square, piece, active_color, lines)
    }

    fn legal_king_moves(&self, origin_square: &Square, piece: &Piece, active_color: &Color) -> MoveList {
        let mut moves = MoveList::new();

        // Regular king moves
        let group: SquareList = origin_square.squares_on_king_move();
        moves.append(&mut self.legal_moves_for_group(origin_square, piece, active_color, group));

        // Short castling
        if self
            .castling_availability
            .is_short_castle_available(active_color)
        {
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
                let chess_move = self.new_move(piece, origin_square, Action::ShortCastle, destination_square);
                moves.push(chess_move);
            }
        }

        // Long castling
        if self
            .castling_availability
            .is_long_castle_available(active_color)
        {
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
                let chess_move = self.new_move(piece, origin_square, Action::LongCastle, destination_square);
                moves.push(chess_move);
            }
        }

        moves
    }

    fn legal_knight_moves(&self, origin_square: &Square, piece: &Piece, active_color: &Color) -> MoveList {
        let group: SquareList = origin_square.squares_on_knight_moves();
        self.legal_moves_for_group(origin_square, piece, active_color, group)
    }

    fn legal_pawn_moves(&self, origin_square: &Square, piece: &Piece, active_color: &Color) -> MoveList {
        let mut moves = MoveList::new();

        // Two squares forward if the pawn hasn't moved from the starting rank yet, otherwise one square forward
        let number_of_steps = if origin_square.rank() == active_color.get_second_rank() {
            2
        } else {
            1
        };

        // Get the vertical line for a specific number of steps
        let line: SquareList = match active_color {
            Color::White => origin_square.squares_on_up_vertical(),
            Color::Black => origin_square.squares_on_down_vertical(),
        };

        // Filter our any squares that are blocked
        for destination_square in line.into_iter().take(number_of_steps) {
            match self.is_occupied_by(&destination_square, active_color) {
                OccupiedBy::None => {
                    // If reached last rank, the pawn can promote
                    if destination_square.rank() == active_color.get_eight_rank() {
                        // Iterate over all possible promotions
                        for kind in Kind::get_promotable_kinds() {
                            let action = Action::MovePromotion(kind);
                            let chess_move = self.new_move(piece, origin_square, action, destination_square.copy());
                            moves.push(chess_move);
                        }
                    } else {
                        // Regular move forward
                        let chess_move = self.new_move(piece, origin_square, Action::Move, destination_square);
                        moves.push(chess_move);
                    }
                }
                _ => {
                    // Cannot capture or move through occupied squares, regardless of color
                    break;
                }
            }
        }

        // Right diagonal capture or en passant
        let diagonals: [SquareList; 2] = match active_color {
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
                match self.is_occupied_by(&destination_square, active_color) {
                    OccupiedBy::OppositeColor => {
                        // If reached last rank, the pawn can promote
                        if destination_square.rank() == active_color.get_eight_rank() {
                            // Iterate over all possible promotions
                            for kind in Kind::get_promotable_kinds() {
                                let action = Action::CapturePromotion(kind);
                                let chess_move = self.new_move(piece, origin_square, action, destination_square.copy());
                                moves.push(chess_move);
                            }
                        } else {
                            // Regular diagonal captures
                            let chess_move = self.new_move(piece, origin_square, Action::Capture, destination_square);
                            moves.push(chess_move);
                        }
                    }
                    OccupiedBy::None => match &self.en_passant_target {
                        Some(square) => {
                            // Check if the destination square matches en passant target square
                            if destination_square == *square {
                                let action = Action::EnPassant;
                                let chess_move = self.new_move(piece, origin_square, action, destination_square);
                                moves.push(chess_move);
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

    fn legal_queen_moves(&self, origin_square: &Square, piece: &Piece, active_color: &Color) -> MoveList {
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
        self.legal_moves_for_lines(origin_square, piece, active_color, lines)
    }

    fn legal_rook_moves(&self, origin_square: &Square, piece: &Piece, active_color: &Color) -> MoveList {
        // Rook moves into 4 different directions(2 vertical and 2 horizontal)
        let lines: Vec<SquareList> = vec![
            origin_square.squares_on_up_vertical(),
            origin_square.squares_on_down_vertical(),
            origin_square.squares_on_right_horizontal(),
            origin_square.squares_on_left_horizontal(),
        ];

        // Return result
        self.legal_moves_for_lines(origin_square, piece, active_color, lines)
    }

    fn legal_moves_for_lines(
        &self, origin_square: &Square, piece: &Piece, active_color: &Color, lines: Vec<SquareList>,
    ) -> MoveList {
        let mut moves = Vec::new();

        // Add legal moves for each of those direction
        for line in lines {
            for destination_square in line.into_iter() {
                match self.is_occupied_by(&destination_square, active_color) {
                    OccupiedBy::SameColor => {
                        // Cannot take or move through own piece
                        break;
                    }
                    OccupiedBy::OppositeColor => {
                        // Can capture opposite color
                        let chess_move = self.new_move(piece, origin_square, Action::Capture, destination_square);
                        moves.push(chess_move);

                        // But cannot move any further
                        break;
                    }
                    OccupiedBy::None => {
                        // Can move to empty square and keep moving
                        let chess_move = self.new_move(piece, origin_square, Action::Move, destination_square);
                        moves.push(chess_move);
                    }
                };
            }
        }

        moves
    }

    fn legal_moves_for_group(
        &self, origin_square: &Square, piece: &Piece, active_color: &Color, group: SquareList,
    ) -> MoveList {
        let mut moves = Vec::new();

        for destination_square in group.into_iter() {
            match self.is_occupied_by(&destination_square, active_color) {
                OccupiedBy::SameColor => {
                    // Cannot take piece of same color
                }
                OccupiedBy::OppositeColor => {
                    // Can capture opposite color
                    let chess_move = self.new_move(piece, origin_square, Action::Capture, destination_square);
                    moves.push(chess_move);
                }
                OccupiedBy::None => {
                    // Can move to empty square
                    let chess_move = self.new_move(piece, origin_square, Action::Move, destination_square);
                    moves.push(chess_move);
                }
            };
        }

        moves
    }
}
