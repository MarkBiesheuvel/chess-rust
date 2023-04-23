// Imports from crate
use crate::board::{Board, Direction, Square, SquareIterator};
use crate::piece::{Color, Piece};

// Imports from super
use super::PieceBehavior;

/// A piece that moves any number of squares vertically or horizontally without jumping.
///
/// ## Examples
/// ```
/// use chess::board::SquareNotation::*;
/// use chess::piece::{Piece, Color::*, behavior::*};
///
/// // Create a new rook
/// let rook = Piece::new(H8, Black, Rook);
///
/// // Assertions
/// assert_eq!(rook.to_string(), "r");
/// assert_eq!(rook.square().to_string(), "h8");
/// ```
#[derive(Debug)]
pub struct Rook;

impl PieceBehavior for Rook {
    fn target_squares(&self, piece: &Piece, board: &Board) -> Vec<Square> {
        // Rook has no limit when moving
        let limit = None;

        // Create lines in all four vertical/horizontal directions
        let lines = [
            SquareIterator::from_direction(piece, board, limit, Direction::HorizontalRight),
            SquareIterator::from_direction(piece, board, limit, Direction::HorizontalLeft),
            SquareIterator::from_direction(piece, board, limit, Direction::VerticalUp),
            SquareIterator::from_direction(piece, board, limit, Direction::VerticalDown),
        ];

        // Flatten lines into single list
        lines.into_iter().flatten().collect()
    }

    fn symbol(&self, color: &Color) -> char {
        match color {
            Color::Black => 'r',
            Color::White => 'R',
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::board::SquareNotation::*;

    #[test]
    fn test_d4() {
        // Create empty 8×8 board
        let board = Board::empty(8);

        // Create rook on d4
        let rook = Piece::new(D4, Color::Black, Rook);

        // Calculate the target squares
        let destinations = rook.target_squares(&board);

        // Test a square in each direction
        assert_eq!(destinations.len(), 14);
        assert!(destinations.contains(&C4.into()));
        assert!(destinations.contains(&D2.into()));
        assert!(destinations.contains(&G4.into()));
        assert!(destinations.contains(&D8.into()));
    }
}
