// External imports
use std::rc::Rc;

// Imports from crate
use crate::board::{Square, SquareIterator};
use crate::piece::Color;

/// Trait that implements the behavior of a chess piece
pub trait PieceBehavior {
    /// Returns symbol of a piece
    fn symbol(color: Color) -> char;

    /// Starting from the origin, give a list of potential destination squares
    fn normal_moves(origin: Rc<Square>) -> Vec<SquareIterator>;

    // TODO: implement special moves (pass in Game struct)
}
