// External imports
use std::rc::Rc;

// Imports from crate
use crate::board::{Offset, Square, SquareIterator};
use crate::piece::Color;

// Imports from super
use super::PieceBehavior;

/// A piece that moves two squares vertically and one square horizontally,
/// or two squares horizontally and one square vertically, jumping over other pieces.
///
/// ## Examples
/// ```
/// use std::rc::Rc;
/// use chess::piece::behavior::{Knight, PieceBehavior};
/// use chess::board::Square;
///
/// // Start at the f3 square
/// let origin = "f3".parse()?;
/// let origin = Rc::new(origin);
///
/// // Calculate all destinations
/// let destinations = Knight.target_squares(origin)
///     .into_iter()
///     .flatten()
///     .map(|s| s.to_string())
///     .collect::<Vec<_>>();
///
/// // Test all squares
/// assert_eq!(destinations.len(), 8);
/// assert!(destinations.iter().any(|s| s == "g5"));
/// assert!(destinations.iter().any(|s| s == "h4"));
/// assert!(destinations.iter().any(|s| s == "h2"));
/// assert!(destinations.iter().any(|s| s == "g1"));
/// assert!(destinations.iter().any(|s| s == "e1"));
/// assert!(destinations.iter().any(|s| s == "d2"));
/// assert!(destinations.iter().any(|s| s == "d4"));
/// assert!(destinations.iter().any(|s| s == "e5"));
/// #
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[derive(Debug)]
pub struct Knight;

impl PieceBehavior for Knight {
    fn target_squares(&self, origin: Rc<Square>) -> Vec<SquareIterator> {
        Vec::from([
            SquareIterator::from_single_offset(Rc::clone(&origin), Offset::new(1, 2)),
            SquareIterator::from_single_offset(Rc::clone(&origin), Offset::new(2, 1)),
            SquareIterator::from_single_offset(Rc::clone(&origin), Offset::new(2, -1)),
            SquareIterator::from_single_offset(Rc::clone(&origin), Offset::new(1, -2)),
            SquareIterator::from_single_offset(Rc::clone(&origin), Offset::new(-1, -2)),
            SquareIterator::from_single_offset(Rc::clone(&origin), Offset::new(-2, -1)),
            SquareIterator::from_single_offset(Rc::clone(&origin), Offset::new(-2, 1)),
            SquareIterator::from_single_offset(Rc::clone(&origin), Offset::new(-1, 2)),
        ])
    }

    fn symbol(&self, color: &Color) -> char {
        match color {
            Color::Black => 'n',
            Color::White => 'N',
        }
    }
}
