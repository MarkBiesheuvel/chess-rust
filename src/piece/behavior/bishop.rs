// External imports
use std::rc::Rc;

// Imports from crate
use crate::board::{Direction, Square, SquareIterator};
use crate::piece::Color;

// Imports from super
use super::PieceBehavior;

/// A piece that moves and captures along diagonals without jumping over intervening pieces.
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
/// let destinations = Bishop::normal_moves(origin)
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
pub struct Bishop;

impl PieceBehavior for Bishop {
    fn normal_moves(origin: Rc<Square>) -> Vec<SquareIterator> {
        vec![
            SquareIterator::from_direction(Rc::clone(&origin), Direction::DiagonalUpRight),
            SquareIterator::from_direction(Rc::clone(&origin), Direction::DiagonalUpLeft),
            SquareIterator::from_direction(Rc::clone(&origin), Direction::DiagonalDownRight),
            SquareIterator::from_direction(Rc::clone(&origin), Direction::DiagonalDownLeft),
        ]
    }

    fn symbol(color: Color) -> char {
        match color {
            Color::Black => 'b',
            Color::White => 'B',
        }
    }
}
