// External imports
use std::rc::Rc;

// Imports from crate
use crate::board::{Direction, Square, SquareIterator};
use crate::piece::Color;

// Imports from super
use super::PieceBehavior;

/// A piece that moves any number of squares vertically or horizontally without jumping.
///
/// ## Examples
/// ```
/// use std::rc::Rc;
/// use chess::piece::behavior::{PieceBehavior, Rook};
/// use chess::board::Square;
///
/// // Start at the d4 square
/// let origin = "d4".parse()?;
/// let origin = Rc::new(origin);
///
/// // Calculate the first 4 destinations in any direction
/// let destinations = Rook::normal_moves(origin)
///     .into_iter()
///     .flat_map(|i| i.take(4))
///     .collect::<Vec<_>>();
///
/// // Test a square in each direction
/// assert!(destinations.iter().any(|s| s.to_string() == "c4"));
/// assert!(destinations.iter().any(|s| s.to_string() == "d2"));
/// assert!(destinations.iter().any(|s| s.to_string() == "g4"));
/// assert!(destinations.iter().any(|s| s.to_string() == "d8"));
///
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub struct Rook;

impl PieceBehavior for Rook {
    fn normal_moves(origin: Rc<Square>) -> Vec<SquareIterator> {
        Vec::from([
            SquareIterator::from_direction(Rc::clone(&origin), Direction::HorizontalRight),
            SquareIterator::from_direction(Rc::clone(&origin), Direction::HorizontalLeft),
            SquareIterator::from_direction(Rc::clone(&origin), Direction::VerticalUp),
            SquareIterator::from_direction(Rc::clone(&origin), Direction::VerticalDown),
        ])
    }

    fn symbol(color: &Color) -> char {
        match color {
            Color::Black => 'r',
            Color::White => 'R',
        }
    }
}
