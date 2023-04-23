// Imports from crate
use crate::board::{Board, Direction, Square, SquareIterator};
use crate::piece::{Color, Piece};

// Imports from super
use super::PieceBehavior;

/// A piece that moves any number of squares vertically or horizontally without jumping.
///
/// ## Examples
/// ```
/// use std::str::FromStr;
/// use chess::board::{Board, Square};
/// use chess::piece::{Piece, Color::*, behavior::*};
///
/// // Start at the d4 square
/// let square = Square::from_str("d4")?;
/// let piece = Piece::new(square, White, Rook);
/// let board = Board::empty(8);
///
/// // Calculate the first 4 destinations in any direction
/// let destinations = Rook.target_squares(&piece, &board);
///
/// // Test a square in each direction
/// assert!(destinations.iter().any(|s| s.to_string() == "c4"));
/// assert!(destinations.iter().any(|s| s.to_string() == "d2"));
/// assert!(destinations.iter().any(|s| s.to_string() == "g4"));
/// assert!(destinations.iter().any(|s| s.to_string() == "d8"));
/// #
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[derive(Debug)]
pub struct Rook;

impl PieceBehavior for Rook {
    fn target_squares(&self, piece: &Piece, board: &Board) -> Vec<Square> {
        // Rook has no limit when moving
        let limit = None;

        // Create lines in all four vertical/horizontal directions
        let lines = [
            SquareIterator::from_direction(piece, board, limit, Direction::HorizontalRight),
            SquareIterator::from_direction(piece, board, limit, Direction::HorizontalLeft),
            SquareIterator::from_direction(piece, board, limit, Direction::VerticalUp),
            SquareIterator::from_direction(piece, board, limit, Direction::VerticalDown),
        ];

        // Flatten lines into single list
        lines.into_iter().flatten().collect()
    }

    fn symbol(&self, color: &Color) -> char {
        match color {
            Color::Black => 'r',
            Color::White => 'R',
        }
    }
}
