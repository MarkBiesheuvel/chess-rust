// Imports from crate
use crate::board::{Board, Offset, Square, SquareIterator};
use crate::piece::{Color, Piece};

// Imports from super
use super::PieceBehavior;

/// A piece that moves two squares vertically and one square horizontally,
/// or two squares horizontally and one square vertically, jumping over other pieces.
///
/// ## Examples
/// ```
/// use chess::board::SquareNotation::*;
/// use chess::piece::{Piece, Color::*, behavior::*};
///
/// // Create a new knight
/// let knight = Piece::new(B1, White, Knight);
///
/// // Assertions
/// assert_eq!(knight.board_representation(), 'N');
/// assert_eq!(knight.square().to_string(), "b1");
/// ```
#[derive(Debug)]
pub struct Knight;

impl PieceBehavior for Knight {
    fn target_squares(&self, piece: &Piece, board: &Board) -> Vec<Square> {
        // Create a "lines" for each knights offset
        let lines = Vec::from([
            SquareIterator::from_single_offset(piece, board, Offset::new(1, 2)),
            SquareIterator::from_single_offset(piece, board, Offset::new(2, 1)),
            SquareIterator::from_single_offset(piece, board, Offset::new(2, -1)),
            SquareIterator::from_single_offset(piece, board, Offset::new(1, -2)),
            SquareIterator::from_single_offset(piece, board, Offset::new(-1, -2)),
            SquareIterator::from_single_offset(piece, board, Offset::new(-2, -1)),
            SquareIterator::from_single_offset(piece, board, Offset::new(-2, 1)),
            SquareIterator::from_single_offset(piece, board, Offset::new(-1, 2)),
        ]);

        // Flatten lines into single list
        lines.into_iter().flatten().collect()
    }

    fn board_representation(&self, color: &Color) -> char {
        match color {
            Color::Black => 'n',
            Color::White => 'N',
        }
    }

    fn move_representation(&self) -> &str {
        "N"
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

        // Create knight on f3
        let knight = Piece::new(F3, Color::White, Knight);

        // Calculate the target squares
        let destinations = knight.target_squares(&board);

        // Test all target sqaures
        assert_eq!(destinations.len(), 8);
        assert!(destinations.contains(&G5.into()));
        assert!(destinations.contains(&H4.into()));
        assert!(destinations.contains(&H2.into()));
        assert!(destinations.contains(&G1.into()));
        assert!(destinations.contains(&E1.into()));
        assert!(destinations.contains(&D2.into()));
        assert!(destinations.contains(&D4.into()));
        assert!(destinations.contains(&E5.into()));
    }
}
