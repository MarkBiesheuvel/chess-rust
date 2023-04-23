// External imports
use std::rc::Rc;

// Imports from crate
use crate::board::{Direction, Square, SquareIterator};
use crate::piece::Color;

// Imports from super
use super::PieceBehavior;

/// A piece that moves one vacant square directly forward.
///
/// ## Examples
/// ```
/// use std::rc::Rc;
/// use chess::piece::behavior::{Pawn, PieceBehavior};
/// use chess::board::Square;
///
/// // Start at the c4 square
/// let origin = "c4".parse()?;
/// let origin = Rc::new(origin);
///
/// // Calculate all destinations
/// let destinations = Pawn.target_squares(origin)
///     .into_iter()
///     .flatten()
///     .map(|s| s.to_string())
///     .collect::<Vec<_>>();
///
/// // Test all squares
/// assert_eq!(destinations.len(), 1);
/// assert!(destinations.iter().any(|s| s == "c5"));
///
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[derive(Debug)]
pub struct Pawn;

impl PieceBehavior for Pawn {
    fn target_squares(&self, origin: Rc<Square>) -> Vec<SquareIterator> {
        // TODO: add color to determine correct direction
        Vec::from([SquareIterator::from_direction(Rc::clone(&origin), Direction::VerticalUp).limit(1)])
    }

    fn symbol(&self, color: &Color) -> char {
        match color {
            Color::Black => 'p',
            Color::White => 'P',
        }
    }
}
