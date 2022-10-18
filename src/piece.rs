#![allow(dead_code)]
// External imports
use std::fmt;
// Relative imports of sub modules
pub use color::Color;
pub use kind::Kind;
mod color;
mod kind;

#[derive(Debug)]
pub struct Piece {
    color: Color,
    kind: Kind,
}
impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol())
    }
}
impl Piece {
    pub fn new(color: Color, kind: Kind) -> Piece {
        Piece { color, kind }
    }

    fn symbol(&self) -> &str {
        match (&self.color, &self.kind) {
            (Color::White, Kind::Bishop) => "♗",
            (Color::Black, Kind::Bishop) => "♝",
            (Color::White, Kind::King) => "♔",
            (Color::Black, Kind::King) => "♚",
            (Color::White, Kind::Knight) => "♘",
            (Color::Black, Kind::Knight) => "♞",
            (Color::White, Kind::Pawn) => "♙",
            (Color::Black, Kind::Pawn) => "♟",
            (Color::White, Kind::Queen) => "♕",
            (Color::Black, Kind::Queen) => "♛",
            (Color::White, Kind::Rook) => "♖",
            (Color::Black, Kind::Rook) => "♜",
        }
    }

    pub fn color(&self) -> &Color {
        &self.color
    }

    pub fn kind(&self) -> &Kind {
        &self.kind
    }

    pub fn legal_moves(&self) -> Vec<(i8, i8)> {
        match self.kind {
            Kind::Knight => {
                vec![(2, 1), (2, -1), (-2, 1), (-2, -1), (1, 2), (1, -2), (-1, 2), (-1, 2)]
            }
            _ => {
                // TODO: implement for other pieces
                vec![]
            }
        }
    }
}
