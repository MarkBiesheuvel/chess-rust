// Imports from crate
use crate::board::{Board, Direction, Square, SquareIterator};
use crate::piece::{Color, Piece};

// Imports from super
use super::PieceBehavior;

/// A piece that moves any number of squares diagonally without jumping.
///
/// ## Examples
/// ```
/// use chess::board::SquareNotation::*;
/// use chess::piece::{Piece, Color::*, behavior::*};
///
/// // Create a new bishop
/// let bishop = Piece::new(D4, White, Bishop);
///
/// // Assertions
/// assert_eq!(bishop.to_string(), "B");
/// assert_eq!(bishop.square().to_string(), "d4");
/// ```
#[derive(Debug)]
pub struct Bishop;

impl PieceBehavior for Bishop {
    fn target_squares(&self, piece: &Piece, board: &Board) -> Vec<Square> {
        // Bishop has no limit when moving
        let limit = None;

        // Create lines in all four diagonal directions
        let lines = [
            SquareIterator::from_direction(piece, board, limit, Direction::DiagonalRightUp),
            SquareIterator::from_direction(piece, board, limit, Direction::DiagonalRightDown),
            SquareIterator::from_direction(piece, board, limit, Direction::DiagonalLeftUp),
            SquareIterator::from_direction(piece, board, limit, Direction::DiagonalLeftDown),
        ];

        // Flatten lines into single list
        lines.into_iter().flatten().collect()
    }

    fn symbol(&self, color: &Color) -> char {
        match color {
            Color::Black => 'b',
            Color::White => 'B',
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

        // Create bishop on d4
        let bishop = Piece::new(D4, Color::White, Bishop);

        // Calculate the target squares
        let destinations = bishop.target_squares(&board);

        // Test a square in each direction
        assert_eq!(destinations.len(), 13);
        assert!(destinations.contains(&A1.into()));
        assert!(destinations.contains(&B6.into()));
        assert!(destinations.contains(&H8.into()));
        assert!(destinations.contains(&E3.into()));
    }
}
