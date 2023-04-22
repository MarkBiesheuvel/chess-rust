// External imports
use std::rc::Rc;

// Imports from crate
use crate::board::{Direction, Square, SquareIterator};
use crate::piece::Color;

// Imports from super
use super::PieceBehavior;

/// A piece that moves any number of squares diagonally without jumping.
///
/// ## Examples
/// ```
/// use std::rc::Rc;
/// use chess::piece::behavior::{Bishop, PieceBehavior};
/// use chess::board::Square;
///
/// // Start at the d4 square
/// let origin = "d4".parse()?;
/// let origin = Rc::new(origin);
///
/// // Calculate the first 4 destinations in any direction
/// let destinations = Bishop.target_squares(origin)
///     .into_iter()
///     .flat_map(|i| i.take(4))
///     .collect::<Vec<_>>();
///
/// // Test a square in each direction
/// assert!(destinations.iter().any(|s| s.to_string() == "a1"));
/// assert!(destinations.iter().any(|s| s.to_string() == "b6"));
/// assert!(destinations.iter().any(|s| s.to_string() == "h8"));
/// assert!(destinations.iter().any(|s| s.to_string() == "e3"));
///
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[derive(Debug)]
pub struct Bishop;

impl PieceBehavior for Bishop {
    fn target_squares(&self, origin: Rc<Square>) -> Vec<SquareIterator> {
        Vec::from([
            SquareIterator::from_direction(Rc::clone(&origin), Direction::DiagonalRightUp),
            SquareIterator::from_direction(Rc::clone(&origin), Direction::DiagonalRightDown),
            SquareIterator::from_direction(Rc::clone(&origin), Direction::DiagonalLeftUp),
            SquareIterator::from_direction(Rc::clone(&origin), Direction::DiagonalLeftDown),
        ])
    }

    fn symbol(&self, color: &Color) -> char {
        match color {
            Color::Black => 'b',
            Color::White => 'B',
        }
    }
}
