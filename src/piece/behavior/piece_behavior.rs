// External imports
use std::fmt;

// Imports from crate
use crate::board::{Board, Square};
use crate::piece::{Color, Piece};

/// Trait that implements the behavior of a chess piece
pub trait PieceBehavior: fmt::Debug {
    /// Returns symbol of a piece
    fn symbol(&self, color: &Color) -> char;

    /// Starting from the origin on the board, give a list of target squares
    fn target_squares(&self, piece: &Piece, board: &Board) -> Vec<Square>;
}
