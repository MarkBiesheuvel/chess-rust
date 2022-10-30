// External imports
use std::collections;
// Imports from parent
use super::{ChessMove, Piece, Square};

// Custom type alias for file and rank
pub type File = i8;
pub type Rank = i8;

// Custom type alias for 8×8 board of squares
pub type PiecePlacement = collections::HashMap<Square, Piece>;

// Custom type alias for list of chess moves
pub type MoveList = Vec<ChessMove>;

// Custom type alias for list of squares
pub type SquareList<'a> = Vec<Square>;
