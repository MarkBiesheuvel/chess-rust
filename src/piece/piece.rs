// External imports
use std::fmt;
use std::marker::PhantomData;

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
#[derive(Debug, Eq, PartialEq)]
pub struct Piece<T>
where
    T: PieceBehavior,
{
    color: Color,
    behavior: PhantomData<T>,
}

impl<T> Piece<T>
where
    T: PieceBehavior,
{
    /// Create a new chess piece.
    ///
    /// The second parameter is solely used to infer the generic type.
    pub fn new(color: Color, _behavior: T) -> Piece<T> {
        Piece {
            color,
            behavior: PhantomData,
        }
    }

    /// Get the color of the piece
    pub fn color(&self) -> &Color {
        &self.color
    }
}

impl<T> fmt::Display for Piece<T>
where
    T: PieceBehavior,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = T::symbol(self.color());

        write!(f, "{}", symbol)
    }
}
