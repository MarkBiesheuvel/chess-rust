// External imports
use thiserror::Error;
// Imports from parent
use super::{Kind, Square};

// Enum to represent the different error types
#[derive(Error, Debug, PartialEq)]
pub enum BoardError {
    #[error("no piece found on required square: {0}")]
    PieceMissing(Square),
    #[error("expected promotion pawn, found: {0}")]
    InvalidPromotionPawn(Kind),
    #[error("expected en-passant pawn, found: {0}")]
    InvalidEnPassantPawn(Kind),
    #[error("expected castling kind, found: {0}")]
    InvalidCastlingKing(Kind),
    #[error("expected castling kind, found: {0}")]
    InvalidCastlingRook(Kind),
}
