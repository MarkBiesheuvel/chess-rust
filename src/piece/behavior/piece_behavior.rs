// External imports
use std::fmt;

// Imports from crate
use crate::board::{Board, Square};
use crate::piece::{Color, Piece};

/// Trait that implements the behavior of a chess piece
pub trait PieceBehavior: fmt::Debug {
    /// Returns character to represent piece on chess board
    fn board_representation(&self, color: &Color) -> char;

    /// Return string to represent piece in a chess move
    fn move_representation(&self) -> &str;

    /// Starting from the origin on the board, give a list of target squares
    fn target_squares(&self, piece: &Piece, board: &Board) -> Vec<Square>;
}
