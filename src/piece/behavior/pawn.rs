// Imports from crate
use crate::board::{Board, Direction, Square, SquareIterator};
use crate::piece::{Color, Piece};

// Imports from super
use super::PieceBehavior;

/// A piece that moves one vacant square directly forward.
///
/// ## Examples
/// ```
/// use chess::board::SquareNotation::*;
/// use chess::piece::{Piece, Color::*, behavior::*};
///
/// // Create a new pawn
/// let pawn = Piece::new(A2, White, Pawn);
///
/// // Assertions
/// assert_eq!(pawn.board_representation(), 'P');
/// assert_eq!(pawn.square().to_string(), "a2");
/// ```
#[derive(Debug)]
pub struct Pawn;

impl PieceBehavior for Pawn {
    fn target_squares(&self, piece: &Piece, board: &Board) -> Vec<Square> {
        // TODO: determine whether the Game allows for 2 squares forward
        let limit = Some(1);

        // Determine direction based on color
        let direction = match piece.color() {
            Color::Black => Direction::VerticalDown,
            Color::White => Direction::VerticalUp,
        };

        // TODO: determine whether can capture
        // TODO: determine whether the Game allows for en-passant
        SquareIterator::from_direction(piece, board, limit, direction)
    }

    fn board_representation(&self, color: &Color) -> char {
        match color {
            Color::Black => 'p',
            Color::White => 'P',
        }
    }

    fn move_representation(&self) -> &str {
        ""
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::board::SquareNotation::*;

    #[test]
    fn test_f4() {
        // Create empty 8×8 board
        let board = Board::empty(8);

        // Create pawn on f4
        let pawn = Piece::new(F4, Color::Black, Pawn);

        // Calculate the target squares
        let destinations = pawn.target_squares(&board);

        // Test all target sqaures
        assert_eq!(destinations.len(), 1);
        assert!(destinations.contains(&F3.into()));
    }

    #[test]
    fn test_c4() {
        // Create empty 8×8 board
        let board = Board::empty(8);

        // Create pawn on c4
        let pawn = Piece::new(C4, Color::White, Pawn);

        // Calculate the target squares
        let destinations = pawn.target_squares(&board);

        // Test all target sqaures
        assert_eq!(destinations.len(), 1);
        assert!(destinations.contains(&C5.into()));
    }
}
