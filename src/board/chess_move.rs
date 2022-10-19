// External imports
use std::fmt;
// Absolute imports within crate
use crate::board;
use crate::piece;

#[derive(Debug)]
pub enum Action {
    Move,
    Capture,
}

// The starting square is owned by the Board, so we are just borrowing (referencing) it in the ChessMove
// The destination square was just created, so we can take over ownership
#[derive(Debug)]
pub struct ChessMove<'a> {
    piece: &'a piece::Piece,
    origin_square: &'a board::Square,
    action: Action,
    destination_square: board::Square,
}
impl ChessMove<'_> {
    pub fn new<'a>(
        piece: &'a piece::Piece,
        origin_square: &'a board::Square,
        action: Action,
        destination_square: board::Square,
    ) -> ChessMove {
        // It wouldn't be much of a move if we stay at the same square
        if *origin_square == destination_square {
            panic!("a move needs to go from one square to a different one");
        }

        ChessMove {
            piece,
            origin_square,
            action,
            destination_square,
        }
    }
}
impl fmt::Display for ChessMove<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use piece::Kind;
        // Long algebraic notation

        // TODO: add captures
        // TODO: add check(mate)
        // TODO: add castling

        // Letter as used in algebraic notation
        match self.piece.kind() {
            Kind::Bishop => {
                write!(f, "B")?;
            }
            Kind::King => {
                write!(f, "K")?;
            }
            Kind::Knight => {
                write!(f, "N")?;
            }
            Kind::Pawn => {
                // Implicit
            }
            Kind::Queen => {
                write!(f, "Q")?;
            }
            Kind::Rook => {
                write!(f, "R")?;
            }
        }

        // Starting square
        write!(f, "{}", self.origin_square)?;

        // Capture or not capture
        match self.action {
            Action::Capture => {
                write!(f, "x")?;
            }
            Action::Move => {}
        }

        // Destination square
        write!(f, "{}", self.destination_square)?;

        // All okay
        Ok(())
    }
}

// Custom type alias for vector of moves
pub type Moves<'a> = Vec<ChessMove<'a>>;
