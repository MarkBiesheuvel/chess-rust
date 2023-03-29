// External imports
use std::fmt;

// Relative imports of sub modules
pub use color::Color;
pub use kind::Kind;
mod color;
mod kind;

#[derive(Debug, Clone)]
pub struct Piece {
    color: Color,
    kind: Kind,
}

impl Piece {
    pub fn new(color: Color, kind: Kind) -> Piece {
        Piece { color, kind }
    }

    pub fn color(&self) -> &Color {
        &self.color
    }

    pub fn kind(&self) -> &Kind {
        &self.kind
    }

    pub fn promote(&mut self, kind: Kind) {
        self.kind = kind;
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = match (&self.color, &self.kind) {
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
        };

        write!(f, "{}", symbol)
    }
}
