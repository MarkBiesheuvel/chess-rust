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
/// let destinations = Pawn::normal_moves(origin)
///     .into_iter()
///     .flatten()
///     .collect::<Vec<_>>();
///
/// // Test all squares
/// assert_eq!(destinations.len(), 1);
/// assert!(destinations.iter().any(|s| s.to_string() == "c5"));
///
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub struct Pawn;

impl PieceBehavior for Pawn {
    fn normal_moves(origin: Rc<Square>) -> Vec<SquareIterator> {
        // TODO: add color to determine correct direction
        Vec::from([SquareIterator::from_direction(Rc::clone(&origin), Direction::VerticalUp).limit(1)])
    }

    fn symbol(color: &Color) -> char {
        match color {
            Color::Black => 'p',
            Color::White => 'P',
        }
    }
}
