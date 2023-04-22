// External imports
use std::fmt;

// Imports from super
use super::{behavior::PieceBehavior, Color};

/// A chess piece
///
/// ## Examples
/// ```
/// use chess::piece::Piece;
/// use chess::piece::Color::{Black, White};
/// use chess::piece::behavior::{Bishop, King};
///
/// // Create some new pieces
/// let white_bishop = Piece::new(White, Bishop);
/// let black_king = Piece::new(Black, King);
///
/// // Tests
/// assert_eq!(white_bishop.to_string(), "B");
/// assert_eq!(black_king.to_string(), "k");
/// ```
#[derive(Debug)]
pub struct Piece {
    color: Color,
    behavior: Box<dyn PieceBehavior>,
}

impl Piece {
    /// Create a new chess piece.
    ///
    /// The second parameter is solely used to infer the generic type.
    pub fn new(color: Color, behavior: impl PieceBehavior + 'static) -> Piece {
        Piece {
            color,
            behavior: Box::new(behavior),
        }
    }

    /// Get the color of the piece
    pub fn color(&self) -> &Color {
        &self.color
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = self.behavior.symbol(self.color());

        write!(f, "{}", symbol)
    }
}
