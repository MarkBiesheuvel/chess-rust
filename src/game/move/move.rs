// External imports
use std::fmt;

// Absolute imports within crate
use crate::board::Square;
use crate::piece::Piece;

// Imports from super
use super::{Action, Status};

/// A chess move
///
/// ## Examples
/// ```
/// use chess::board::{Square, SquareNotation::*};
/// use chess::piece::{Piece, Color::*, behavior::*};
/// use chess::game::r#move::{Move, Action, Status};
///
/// // Create components
/// let origin = Square::from(E1);
/// let destination = Square::from(E2);
/// let piece = Piece::new(origin, White, King);
///
/// // Create move
/// let chess_move = Move::new(piece, origin, Action::Capture, destination, Status::Check);
///
/// assert_eq!(chess_move.to_string(), "Ke1xe2+");
/// ```
#[derive(Debug)]
pub struct Move {
    piece: Piece,
    origin_square: Square,
    action: Action,
    destination_square: Square,
    status: Status,
}

impl Move {
    /// Create a new move
    pub fn new(
        piece: Piece,
        origin_square: Square,
        action: Action,
        destination_square: Square,
        status: Status,
    ) -> Move {
        Move {
            piece,
            origin_square,
            action,
            destination_square,
            status,
        }
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Long algebraic notation
        match &self.action {
            Action::ShortCastle => {
                // Short castle notation
                write!(f, "0-0")?;
            }
            Action::LongCastle => {
                // Long castle notation
                write!(f, "0-0-0")?;
            }
            _ => {
                // Letter as used in algebraic notation
                write!(f, "{}", self.piece.move_representation())?;

                // Starting square
                // TODO: remove when not need and keep when ambiguous (short algebraic)
                write!(f, "{}", self.origin_square)?;

                // Captures
                match &self.action {
                    Action::Capture | Action::EnPassant | Action::CapturePromotion(_) => {
                        write!(f, "x")?;
                    }
                    _ => {}
                }

                // Destination square
                write!(f, "{}", self.destination_square)?;

                // Extra notation for promotion or en passant
                match &self.action {
                    Action::MovePromotion(behavior) | Action::CapturePromotion(behavior) => {
                        write!(f, "={}", behavior.move_representation())?;
                    }
                    Action::EnPassant => {
                        write!(f, " e.p.")?;
                    }
                    _ => {}
                }
            }
        }

        // Check or checkmate notation
        match &self.status {
            Status::Checkmate => {
                write!(f, "#")?;
            }
            Status::Check => {
                write!(f, "+")?;
            }
            _ => {}
        }

        // All okay
        Ok(())
    }
}
