// External imports
use std::collections::HashMap;

// Imports from crate
use crate::piece::{behavior::*, Color, Piece};

// Imports from super
use super::{Square, SquareStatus};

/// Chess board consisting of 8×8 squares
///
/// ## Examples
/// ```
/// use chess::board::{Board, Square, SquareStatus};
/// use chess::piece::Color::*;
///
/// // Create a chess board with the default starting position
/// let board = Board::starting_position();
/// let square = "c7".parse()?;
///
/// // From white's perspective, the c7 square is taken by the opposite color
/// assert_eq!(board.status(&square, &White), SquareStatus::TakenByOpposite);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[derive(Debug)]
pub struct Board {
    pieces: HashMap<Square, Piece>,
}

impl Board {
    /// Initialize an empty chess board
    pub fn empty() -> Board {
        Board {
            pieces: HashMap::new(),
        }
    }

    /// Initialize a chess board with the starting position
    pub fn starting_position() -> Board {
        // Allow for short name usage
        use Color::{Black, White};

        // Define pieces as an array
        let pieces = [
            // 1st rank
            Piece::new((1, 1), White, Rook),
            Piece::new((2, 1), White, Knight),
            Piece::new((3, 1), White, Bishop),
            Piece::new((4, 1), White, Queen),
            Piece::new((5, 1), White, King),
            Piece::new((6, 1), White, Bishop),
            Piece::new((7, 1), White, Knight),
            Piece::new((8, 1), White, Rook),
            // 2nd rank
            Piece::new((1, 2), White, Pawn),
            Piece::new((2, 2), White, Pawn),
            Piece::new((3, 2), White, Pawn),
            Piece::new((4, 2), White, Pawn),
            Piece::new((5, 2), White, Pawn),
            Piece::new((6, 2), White, Pawn),
            Piece::new((7, 2), White, Pawn),
            Piece::new((8, 2), White, Pawn),
            // 7th rank
            Piece::new((1, 7), Black, Pawn),
            Piece::new((2, 7), Black, Pawn),
            Piece::new((3, 7), Black, Pawn),
            Piece::new((4, 7), Black, Pawn),
            Piece::new((5, 7), Black, Pawn),
            Piece::new((6, 7), Black, Pawn),
            Piece::new((7, 7), Black, Pawn),
            Piece::new((8, 7), Black, Pawn),
            // 8th rank
            Piece::new((1, 8), Black, Rook),
            Piece::new((2, 8), Black, Knight),
            Piece::new((3, 8), Black, Bishop),
            Piece::new((4, 8), Black, Queen),
            Piece::new((5, 8), Black, King),
            Piece::new((6, 8), Black, Bishop),
            Piece::new((7, 8), Black, Knight),
            Piece::new((8, 8), Black, Rook),
        ];

        // Map each piece to a tuple and collect as HashMap
        let pieces = pieces
            .into_iter()
            .map(|piece| (piece.square(), piece))
            .collect();

        Board { pieces }
    }

    /// Return whether the color is in check
    pub fn is_in_check(&self, _color: &Color) -> bool {
        todo!()
    }

    /// Return whether a square is empty, taken by same color, or taken by opposite color
    pub fn status(&self, square: &Square, active_color: &Color) -> SquareStatus {
        match self.pieces.get(square) {
            Some(piece) => {
                if piece.color() == active_color {
                    SquareStatus::TakenBySame
                } else {
                    SquareStatus::TakenByOpposite
                }
            }
            None => SquareStatus::Empty,
        }
    }
}
