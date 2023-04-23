// Imports from crate
use crate::board::{Board, Direction, Square, SquareIterator};
use crate::piece::{Color, Piece};

// Imports from super
use super::PieceBehavior;

/// A piece that moves any number of squares vertically, horizontally or diagonally without jumping.
///
/// ## Examples
/// ```
/// use std::str::FromStr;
/// use chess::board::{Board, Square};
/// use chess::piece::{Piece, Color::*, behavior::*};
///
/// // Start at the d4 square
/// let square = Square::from_str("d4")?;
/// let piece = Piece::new(square, White, Queen);
/// let board = Board::empty(8);
///
/// // Calculate the first 4 destinations in any direction
/// let destinations = Queen.target_squares(&piece, &board);
///
/// // Test a square in each direction
/// assert!(destinations.iter().any(|s| s.to_string() == "a1"));
/// assert!(destinations.iter().any(|s| s.to_string() == "b6"));
/// assert!(destinations.iter().any(|s| s.to_string() == "h8"));
/// assert!(destinations.iter().any(|s| s.to_string() == "e3"));
/// assert!(destinations.iter().any(|s| s.to_string() == "c4"));
/// assert!(destinations.iter().any(|s| s.to_string() == "d2"));
/// assert!(destinations.iter().any(|s| s.to_string() == "g4"));
/// assert!(destinations.iter().any(|s| s.to_string() == "d8"));
/// #
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[derive(Debug)]
pub struct Queen;

impl PieceBehavior for Queen {
    fn target_squares(&self, piece: &Piece, board: &Board) -> Vec<Square> {
        // Queen has no limit when moving
        let limit = None;

        // Create lines in all directions
        let lines = [
            SquareIterator::from_direction(piece, board, limit, Direction::HorizontalRight),
            SquareIterator::from_direction(piece, board, limit, Direction::HorizontalLeft),
            SquareIterator::from_direction(piece, board, limit, Direction::VerticalUp),
            SquareIterator::from_direction(piece, board, limit, Direction::VerticalDown),
            SquareIterator::from_direction(piece, board, limit, Direction::DiagonalRightUp),
            SquareIterator::from_direction(piece, board, limit, Direction::DiagonalRightDown),
            SquareIterator::from_direction(piece, board, limit, Direction::DiagonalLeftUp),
            SquareIterator::from_direction(piece, board, limit, Direction::DiagonalLeftDown),
        ];

        // Flatten lines into single list
        lines.into_iter().flatten().collect()
    }

    fn symbol(&self, color: &Color) -> char {
        match color {
            Color::Black => 'q',
            Color::White => 'Q',
        }
    }
}
