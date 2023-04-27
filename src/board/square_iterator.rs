// Imports from crate
use crate::piece::Piece;

// Imports from super
use super::{Board, Direction, Offset, Square, SquareStatus};

/// An iterator of squares
pub struct SquareIterator;

impl SquareIterator {
    /// Create a SquareIterator which has a single destination (like a knights jump)
    ///
    /// ## Examples
    /// ```
    /// use chess::board::{Board, Offset, Square, SquareIterator, SquareNotation::*};
    /// use chess::piece::{Piece, Color::*, behavior::*};
    ///
    /// // Start at b8 and move like a knight
    /// let square = Square::from(B8);
    /// let piece = Piece::new(square, White, Bishop);
    /// let board = Board::empty(8);
    ///
    /// let offset = Offset::new(1, -2);
    /// let mut iter = SquareIterator::from_single_offset(&piece, &board, offset).into_iter();
    ///
    /// // Line should contain only one square
    /// assert_eq!(iter.next(), Some(Square::from(C6)));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn from_single_offset(piece: &Piece, board: &Board, offset: Offset) -> Vec<Square> {
        // Calculate destination
        let target_square = piece.square() + offset;

        // Return empty list if the target_square is off the board
        if board.is_valid(&target_square) {
            match board.status(&target_square, piece.color()) {
                SquareStatus::Empty | SquareStatus::TakenByOpposite => Vec::from([target_square]),
                SquareStatus::TakenBySame => Vec::new(),
            }
        } else {
            Vec::new()
        }
    }

    /// Create a SquareIterator which originates from the origin square and moves into the direction
    ///
    /// ## Examples
    /// ```
    /// use chess::board::{Board, Direction, Square, SquareIterator, SquareNotation::*};
    /// use chess::piece::{Piece, Color::*, behavior::*};
    ///
    /// // Start at c1 and move like a bishop on an empty board
    /// let square = Square::from(C1);
    /// let piece = Piece::new(square, White, Bishop);
    /// let board = Board::empty(8);
    /// let limit = None;
    ///
    /// let direction = Direction::DiagonalRightUp;
    /// let mut iter = SquareIterator::from_direction(&piece, &board, limit, direction).into_iter();
    ///
    /// // Line should contain all these squares
    /// assert_eq!(iter.next(), Some(Square::from(D2)));
    /// assert_eq!(iter.next(), Some(Square::from(E3)));
    /// assert_eq!(iter.next(), Some(Square::from(F4)));
    /// assert_eq!(iter.next(), Some(Square::from(G5)));
    /// assert_eq!(iter.next(), Some(Square::from(H6)));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn from_direction(piece: &Piece, board: &Board, limit: Option<u8>, direction: Direction) -> Vec<Square> {
        let mut squares = Vec::new();

        // Create a limitless or limited range
        let range: Box<dyn Iterator<Item = u8>> = match limit {
            Some(limit) => Box::new(1..=limit),
            None => Box::new(1..),
        };

        // Iterate over all possible offset in range
        for i in range {
            // Calculate offset for i
            let offset = Offset::from(direction) * i;

            // Calculate destination
            let target_square = piece.square() + offset;

            // Exit if we fall off the board
            if !board.is_valid(&target_square) {
                break;
            }

            match board.status(&target_square, piece.color()) {
                SquareStatus::Empty => {
                    // Add target square and continue
                    squares.push(target_square);
                    continue;
                }
                SquareStatus::TakenByOpposite => {
                    // Add target square and stop
                    squares.push(target_square);
                    break;
                }
                SquareStatus::TakenBySame => {
                    // Stop
                    break;
                }
            }
        }

        squares
    }
}
