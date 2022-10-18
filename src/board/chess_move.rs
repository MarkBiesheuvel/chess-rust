// External imports
use std::fmt;
// Absolute imports within crate
use crate::board;

// The starting square is owned by the Board, so we are just borrowing (referencing) it in the ChessMove
// The destination square was just created, so we can take over ownership
#[derive(Debug)]
pub struct ChessMove<'a> {
    origin_square: &'a board::Square,
    destination_square: board::Square,
}
impl ChessMove<'_> {
    pub fn new(origin_square: &board::Square, destination_square: board::Square) -> ChessMove {
        if *origin_square == destination_square {
            panic!("a move needs to go from one square to a different one");
        }

        ChessMove {
            origin_square,
            destination_square,
        }
    }
}
impl fmt::Display for ChessMove<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Coordinate notation
        // TODO: add &Piece and switch to Long algebraic notation
        // TODO: add captures
        // TODO: add check(mate)
        // TODO: add castling
        write!(f, "{}-", self.origin_square)?;
        write!(f, "{}", self.destination_square)?;
        Ok(())
    }
}

// Custom type alias for vector of moves
pub type Moves<'a> = Vec<ChessMove<'a>>;
