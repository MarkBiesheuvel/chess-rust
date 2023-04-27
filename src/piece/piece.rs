// Imports from crate
use crate::board::{Board, Square};

// Imports from super
use super::{behavior::PieceBehavior, Color};

/// A chess piece
///
/// ## Examples
/// ```
/// use chess::piece::{Piece, Color::*, behavior::*};
///
/// // Create some new pieces
/// let white_rook = Piece::new((1, 1), White, Rook);
/// let black_king = Piece::new((5, 8), Black, King);
///
/// // Tests
/// assert_eq!(white_rook.board_representation(), 'R');
/// assert_eq!(black_king.board_representation(), 'k');
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

    /// Get a list of target squares
    pub fn target_squares(&self, board: &Board) -> Vec<Square> {
        self.behavior.target_squares(self, board)
    }

    /// Returns character to represent piece on chess board
    pub fn board_representation(&self) -> char {
        self.behavior.board_representation(self.color())
    }

    /// Return string to represent piece in a chess move
    pub fn move_representation(&self) -> &str {
        self.behavior.move_representation()
    }
}
