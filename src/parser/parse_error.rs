// External imports
use std::error;
use std::fmt;

// Enum to represent the different error types
#[derive(Debug, PartialEq)]
pub enum ParseError {
    InvalidCastling(char),
    InvalidColor(char),
    InvalidFile(char),
    InvalidPiece(char),
    UnexpectedEnd,
    IncompletePiecePlacement,
}
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::InvalidCastling(character) => {
                write!(f, "invalid castling availability provided: {:?}", character)
            }
            ParseError::InvalidColor(character) => {
                write!(f, "invalid active color provided: {:?}", character)
            }
            ParseError::InvalidFile(character) => {
                write!(f, "invalid file for en passant target square provided: {:?}", character)
            }
            ParseError::InvalidPiece(character) => {
                write!(f, "invalid piece provided: {:?}", character)
            }
            ParseError::UnexpectedEnd => {
                write!(f, "fen record was too short")
            }
            ParseError::IncompletePiecePlacement => {
                write!(f, "not all squares were provided in piece placement field")
            }
        }
    }
}
impl error::Error for ParseError {}
