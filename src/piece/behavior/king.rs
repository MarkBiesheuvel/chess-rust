// Imports from crate
use crate::board::{Board, Direction, Square, SquareIterator};
use crate::piece::{Color, Piece};

// Imports from super
use super::PieceBehavior;

/// A piece that moves one square vertically, horizontally or diagonally.
///
/// ## Examples
/// ```
/// use chess::board::SquareNotation::*;
/// use chess::piece::{Piece, Color::*, behavior::*};
///
/// // Create a new king
/// let king = Piece::new(E7, Black, King);
///
/// // Assertions
/// assert_eq!(king.board_representation(), 'k');
/// assert_eq!(king.square().to_string(), "e7");
/// ```
#[derive(Debug)]
pub struct King;

impl PieceBehavior for King {
    fn target_squares(&self, piece: &Piece, board: &Board) -> Vec<Square> {
        // King can only move 1 step at a time
        let limit = Some(1);

        // Create lines (of length 1) in all directions
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

    fn board_representation(&self, color: &Color) -> char {
        match color {
            Color::Black => 'k',
            Color::White => 'K',
        }
    }

    fn move_representation(&self) -> &str {
        "K"
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::board::SquareNotation::*;

    #[test]
    fn test_e4() {
        // Create empty 8×8 board
        let board = Board::empty(8);

        // Create king on e4
        let king = Piece::new(E4, Color::White, King);

        // Calculate the target squares
        let destinations = king.target_squares(&board);

        // Test all target sqaures
        assert_eq!(destinations.len(), 8);
        assert!(destinations.contains(&D5.into()));
        assert!(destinations.contains(&E5.into()));
        assert!(destinations.contains(&F5.into()));
        assert!(destinations.contains(&D4.into()));
        assert!(destinations.contains(&F4.into()));
        assert!(destinations.contains(&D3.into()));
        assert!(destinations.contains(&E3.into()));
        assert!(destinations.contains(&F3.into()));
    }
}
