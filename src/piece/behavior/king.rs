// External imports
use std::rc::Rc;

// Imports from crate
use crate::board::{Direction, Square, SquareIterator};
use crate::piece::Color;

// Imports from super
use super::PieceBehavior;

/// A piece that moves one square vertically, horizontally or diagonally.
///
/// ## Examples
/// ```
/// use std::rc::Rc;
/// use chess::piece::behavior::{King, PieceBehavior};
/// use chess::board::Square;
///
/// // Start at the e4 square
/// let origin = "e4".parse()?;
/// let origin = Rc::new(origin);
///
/// // Calculate all destinations
/// let destinations = King::normal_moves(origin)
///     .into_iter()
///     .flatten()
///     .collect::<Vec<_>>();
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
///
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub struct King;

impl PieceBehavior for King {
    fn normal_moves(origin: Rc<Square>) -> Vec<SquareIterator> {
        Vec::from([
            SquareIterator::from_direction(Rc::clone(&origin), Direction::HorizontalRight).limit(1),
            SquareIterator::from_direction(Rc::clone(&origin), Direction::HorizontalLeft).limit(1),
            SquareIterator::from_direction(Rc::clone(&origin), Direction::VerticalUp).limit(1),
            SquareIterator::from_direction(Rc::clone(&origin), Direction::VerticalDown).limit(1),
            SquareIterator::from_direction(Rc::clone(&origin), Direction::DiagonalRightUp).limit(1),
            SquareIterator::from_direction(Rc::clone(&origin), Direction::DiagonalRightDown).limit(1),
            SquareIterator::from_direction(Rc::clone(&origin), Direction::DiagonalLeftUp).limit(1),
            SquareIterator::from_direction(Rc::clone(&origin), Direction::DiagonalLeftDown).limit(1),
        ])
    }

    fn symbol(color: &Color) -> char {
        match color {
            Color::Black => 'k',
            Color::White => 'K',
        }
    }
}
