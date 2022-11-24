// External imports
use thiserror::Error;

// Enum to represent the different error types
#[derive(Error, Debug, PartialEq)]
pub enum ParseError {
    #[error("invalid castling availability provided: {0:?}")]
    InvalidCastling(char),
    #[error("invalid active color provided: {0:?}")]
    InvalidColor(char),
    #[error("invalid file for en passant target square provided: {0:?}")]
    InvalidFile(char),
    #[error("invalid rank for en passant target square provided: {0:?}")]
    InvalidRank(char),
    #[error("invalid piece provided: {0:?}")]
    InvalidPiece(char),
    #[error("invalid number")]
    InvalidNumber,
    #[error("fen record was too short")]
    UnexpectedEnd,
    #[error("not all squares were provided in piece placement field")]
    IncompletePiecePlacement,
}
