// External imports
use std::fmt;
use std::rc::Rc;

// Imports from crate
use crate::board::{Square, SquareIterator};
use crate::piece::Color;

/// Trait that implements the behavior of a chess piece
pub trait PieceBehavior: fmt::Debug {
    /// Returns symbol of a piece
    fn symbol(&self, color: &Color) -> char;

    /// Starting from the origin, give a list of potential destination squares
    fn target_squares(&self, origin: Rc<Square>) -> Vec<SquareIterator>;

    // TODO: implement special moves (pass in Game struct)
}
