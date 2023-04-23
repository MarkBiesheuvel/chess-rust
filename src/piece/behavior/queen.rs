// Imports from crate
use crate::board::{Board, Direction, Square, SquareIterator};
use crate::piece::{Color, Piece};

// Imports from super
use super::PieceBehavior;

/// A piece that moves any number of squares vertically, horizontally or diagonally without jumping.
///
/// ## Examples
/// ```
/// use chess::board::SquareNotation::*;
/// use chess::piece::{Piece, Color::*, behavior::*};
///
/// // Create a new queen
/// let queen = Piece::new(G6, Black, Queen);
///
/// // Assertions
/// assert_eq!(queen.to_string(), "q");
/// assert_eq!(queen.square().to_string(), "g6");
/// ```
#[derive(Debug)]
pub struct Queen;

impl PieceBehavior for Queen {
    fn target_squares(&self, piece: &Piece, board: &Board) -> Vec<Square> {
        // Queen has no limit when moving
        let limit = None;

        // Create lines in all directions
        let lines = [
            SquareIterator::from_direction(piece, board, limit, Direction::HorizontalRight),
            SquareIterator::from_direction(piece, board, limit, Direction::HorizontalLeft),
            SquareIterator::from_direction(piece, board, limit, Direction::VerticalUp),
            SquareIterator::from_direction(piece, board, limit, Direction::VerticalDown),
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
            Color::Black => 'q',
            Color::White => 'Q',
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

        // Create queen on d4
        let queen = Piece::new(D4, Color::White, Queen);

        // Calculate the target squares
        let destinations = queen.target_squares(&board);

        // Test a square in each direction
        assert_eq!(destinations.len(), 27);
        assert!(destinations.contains(&A1.into()));
        assert!(destinations.contains(&B6.into()));
        assert!(destinations.contains(&H8.into()));
        assert!(destinations.contains(&E3.into()));
        assert!(destinations.contains(&C4.into()));
        assert!(destinations.contains(&D2.into()));
        assert!(destinations.contains(&G4.into()));
        assert!(destinations.contains(&D8.into()));
    }
}
