// Imports from crate
use crate::board::{Board, Offset, Square, SquareIterator};
use crate::piece::{Color, Piece};

// Imports from super
use super::PieceBehavior;

/// A piece that moves two squares vertically and one square horizontally,
/// or two squares horizontally and one square vertically, jumping over other pieces.
///
/// ## Examples
/// ```
/// use std::str::FromStr;
/// use chess::board::{Board, Square};
/// use chess::piece::{Piece, Color::*, behavior::*};
///
/// // Start at the f3 square
/// let square = Square::from_str("f3")?;
/// let piece = Piece::new(square, White, Knight);
/// let board = Board::empty(8);
///
/// // Calculate all destinations
/// let destinations = Knight.target_squares(&piece, &board);
///
/// // Test all squares
/// assert_eq!(destinations.len(), 8);
/// assert!(destinations.iter().any(|s| s.to_string() == "g5"));
/// assert!(destinations.iter().any(|s| s.to_string() == "h4"));
/// assert!(destinations.iter().any(|s| s.to_string() == "h2"));
/// assert!(destinations.iter().any(|s| s.to_string() == "g1"));
/// assert!(destinations.iter().any(|s| s.to_string() == "e1"));
/// assert!(destinations.iter().any(|s| s.to_string() == "d2"));
/// assert!(destinations.iter().any(|s| s.to_string() == "d4"));
/// assert!(destinations.iter().any(|s| s.to_string() == "e5"));
/// #
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[derive(Debug)]
pub struct Knight;

impl PieceBehavior for Knight {
    fn target_squares(&self, piece: &Piece, board: &Board) -> Vec<Square> {
        // Create a "lines" for each knights offset
        let lines = Vec::from([
            SquareIterator::from_single_offset(piece, board, Offset::new(1, 2)),
            SquareIterator::from_single_offset(piece, board, Offset::new(2, 1)),
            SquareIterator::from_single_offset(piece, board, Offset::new(2, -1)),
            SquareIterator::from_single_offset(piece, board, Offset::new(1, -2)),
            SquareIterator::from_single_offset(piece, board, Offset::new(-1, -2)),
            SquareIterator::from_single_offset(piece, board, Offset::new(-2, -1)),
            SquareIterator::from_single_offset(piece, board, Offset::new(-2, 1)),
            SquareIterator::from_single_offset(piece, board, Offset::new(-1, 2)),
        ]);

        // Flatten lines into single list
        lines.into_iter().flatten().collect()
    }

    fn symbol(&self, color: &Color) -> char {
        match color {
            Color::Black => 'n',
            Color::White => 'N',
        }
    }
}
