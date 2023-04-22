// External imports
use std::rc::Rc;

// Imports from crate
use crate::board::{Direction, Square, SquareIterator};
use crate::piece::Color;

// Imports from super
use super::PieceBehavior;

/// A piece that moves any number of squares vertically, horizontally or diagonally without jumping.
///
/// ## Examples
/// ```
/// use std::rc::Rc;
/// use chess::piece::behavior::{PieceBehavior, Queen};
/// use chess::board::Square;
///
/// // Start at the d4 square
/// let origin = "d4".parse()?;
/// let origin = Rc::new(origin);
///
/// // Calculate the first 4 destinations in any direction
/// let destinations = Queen.target_squares(origin)
///     .into_iter()
///     .flat_map(|i| i.take(4))
///     .collect::<Vec<_>>();
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
///
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[derive(Debug)]
pub struct Queen;

impl PieceBehavior for Queen {
    fn target_squares(&self, origin: Rc<Square>) -> Vec<SquareIterator> {
        Vec::from([
            SquareIterator::from_direction(Rc::clone(&origin), Direction::HorizontalRight),
            SquareIterator::from_direction(Rc::clone(&origin), Direction::HorizontalLeft),
            SquareIterator::from_direction(Rc::clone(&origin), Direction::VerticalUp),
            SquareIterator::from_direction(Rc::clone(&origin), Direction::VerticalDown),
            SquareIterator::from_direction(Rc::clone(&origin), Direction::DiagonalRightUp),
            SquareIterator::from_direction(Rc::clone(&origin), Direction::DiagonalRightDown),
            SquareIterator::from_direction(Rc::clone(&origin), Direction::DiagonalLeftUp),
            SquareIterator::from_direction(Rc::clone(&origin), Direction::DiagonalLeftDown),
        ])
    }

    fn symbol(&self, color: &Color) -> char {
        match color {
            Color::Black => 'q',
            Color::White => 'Q',
        }
    }
}
