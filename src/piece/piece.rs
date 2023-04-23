// External imports
use std::fmt;

// Imports from crate
use crate::board::Square;

// Imports from super
use super::{behavior::PieceBehavior, Color};

/// A chess piece
///
/// ## Examples
/// ```
/// use chess::piece::{Piece, Color::*, behavior::*};
///
/// // Create some new pieces
/// //let white_rook = Piece::new((1, 1), White, Rook);
/// //let black_king = Piece::new((5, 8), Black, King);
///
/// // Tests
/// //assert_eq!(white_rook.to_string(), "R");
/// //assert_eq!(black_king.to_string(), "k");
/// ```
#[derive(Debug)]
pub struct Piece {
    square: Square,
    color: Color,
    behavior: Box<dyn PieceBehavior>,
}

impl Piece {
    /// Create a new chess piece.
    ///
    /// The second parameter is solely used to infer the generic type.
    pub fn new<S, B>(square: S, color: Color, behavior: B) -> Piece
    where
        S: Into<Square>,
        B: PieceBehavior + 'static,
    {
        Piece {
            square: square.into(),
            color,
            behavior: Box::new(behavior),
        }
    }

    /// Get the color of the piece
    pub fn color(&self) -> &Color {
        &self.color
    }

    /// Get the square of the piece
    pub fn square(&self) -> Square {
        // This performs a copy
        self.square
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = self.behavior.symbol(self.color());

        write!(f, "{}", symbol)
    }
}
