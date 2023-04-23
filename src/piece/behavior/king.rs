// Imports from crate
use crate::board::{Board, Direction, Square, SquareIterator};
use crate::piece::{Color, Piece};

// Imports from super
use super::PieceBehavior;

/// A piece that moves one square vertically, horizontally or diagonally.
///
/// ## Examples
/// ```
/// use std::str::FromStr;
/// use chess::board::{Board, Square};
/// use chess::piece::{Piece, Color::*, behavior::*};
///
/// // Start at the e4 square
/// let square = Square::from_str("e4")?;
/// let piece = Piece::new(square, White, King);
/// let board = Board::empty(8);
///
/// // Calculate all destinations
/// let destinations = King.target_squares(&piece, &board);
///
/// // Test all squares
/// assert_eq!(destinations.len(), 8);
/// assert!(destinations.iter().any(|s| s.to_string() == "d5"));
/// assert!(destinations.iter().any(|s| s.to_string() == "e5"));
/// assert!(destinations.iter().any(|s| s.to_string() == "f5"));
/// assert!(destinations.iter().any(|s| s.to_string() == "d4"));
/// assert!(destinations.iter().any(|s| s.to_string() == "f4"));
/// assert!(destinations.iter().any(|s| s.to_string() == "d3"));
/// assert!(destinations.iter().any(|s| s.to_string() == "e3"));
/// assert!(destinations.iter().any(|s| s.to_string() == "f3"));
/// #
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[derive(Debug)]
pub struct King;

impl PieceBehavior for King {
    fn target_squares(&self, piece: &Piece, board: &Board) -> Vec<Square> {
        // King can only move 1 step at a time
        let limit = Some(1);

        // Create lines (of length 1) in all directions
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
            Color::Black => 'k',
            Color::White => 'K',
        }
    }
}
