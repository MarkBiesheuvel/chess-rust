// External imports
use std::fmt;
// Absolute imports within crate
use crate::board::Square;
use crate::piece::{Kind, Piece};

#[derive(Debug, PartialEq, Clone)]
pub enum Action {
    Move,
    Capture,
    EnPassant,
    MovePromotion(Kind),
    CapturePromotion(Kind),
    ShortCastle,
    LongCastle,
}

#[derive(Debug, PartialEq, Clone)]
pub enum MoveStatus {
    Checkmate,
    Check,
    None,
}

#[derive(Debug)]
pub struct ChessMove {
    piece: Piece,
    origin_square: Square,
    action: Action,
    destination_square: Square,
    status: MoveStatus,
}
impl ChessMove {
    pub fn new(
        piece: Piece, origin_square: Square, action: Action, destination_square: Square, status: MoveStatus,
    ) -> ChessMove {
        ChessMove {
            piece,
            origin_square,
            action,
            destination_square,
            status,
        }
    }

    pub fn piece(&self) -> &Piece {
        &self.piece
    }

    pub fn origin_square(&self) -> &Square {
        &self.origin_square
    }

    pub fn action(&self) -> &Action {
        &self.action
    }

    pub fn destination_square(&self) -> &Square {
        &self.destination_square
    }
}
impl fmt::Display for ChessMove {
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
                write!(f, "{}", self.piece.kind())?;

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
                    Action::MovePromotion(kind) | Action::CapturePromotion(kind) => {
                        write!(f, "={}", kind)?;
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
            MoveStatus::Checkmate => {
                write!(f, "#")?;
            }
            MoveStatus::Check => {
                write!(f, "+")?;
            }
            _ => {}
        }

        // All okay
        Ok(())
    }
}
