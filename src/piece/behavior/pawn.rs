// Imports from crate
use crate::board::{Board, Direction, Square, SquareIterator};
use crate::piece::{Color, Piece};

// Imports from super
use super::PieceBehavior;

/// A piece that moves one vacant square directly forward.
///
/// ## Examples
/// ```
/// use std::str::FromStr;
/// use chess::board::{Board, Square};
/// use chess::piece::{Piece, Color::*, behavior::*};
///
/// // Start at the c4 square
/// let square = Square::from_str("c4")?;
/// let piece = Piece::new(square, White, Pawn);
/// let board = Board::empty(8);
///
/// // Calculate all destinations
/// let destinations = Pawn.target_squares(&piece, &board);
///
/// // Test all squares
/// assert_eq!(destinations.len(), 1);
/// assert!(destinations.iter().any(|s| s.to_string() == "c5"));
/// #
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[derive(Debug)]
pub struct Pawn;

impl PieceBehavior for Pawn {
    fn target_squares(&self, piece: &Piece, board: &Board) -> Vec<Square> {
        // TODO: determine whether the Game allows for 2 squares forward
        let limit = Some(1);

        // Determine direction based on color
        let direction = match piece.color() {
            Color::Black => Direction::VerticalDown,
            Color::White => Direction::VerticalUp,
        };

        // TODO: determine whether can capture
        // TODO: determine whether the Game allows for en-passant
        SquareIterator::from_direction(piece, board, limit, direction)
    }

    fn symbol(&self, color: &Color) -> char {
        match color {
            Color::Black => 'p',
            Color::White => 'P',
        }
    }
}
