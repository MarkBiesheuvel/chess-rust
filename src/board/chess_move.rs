// External imports
use std::fmt;
// Absolute imports within crate
use crate::board::Square;
use crate::piece::{Kind, Piece};

#[derive(Debug, PartialEq)]
pub enum Action {
    Move,
    Capture,
    EnPassant,
    // TODO: add MovePromotion and CapturePromotion
    ShortCastle,
    LongCastle,
}

// The starting square is owned by the Board, so we are just borrowing (referencing) it in the ChessMove
// The destination square was just created, so we can take over ownership
#[derive(Debug)]
pub struct ChessMove<'a> {
    piece: &'a Piece,
    origin_square: &'a Square,
    action: Action,
    destination_square: Square,
}
impl ChessMove<'_> {
    pub fn new<'a>(
        piece: &'a Piece,
        origin_square: &'a Square,
        action: Action,
        destination_square: Square,
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

    pub fn action(&self) -> &Action {
        &self.action
    }
}
impl fmt::Display for ChessMove<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Long algebraic notation
        match self.action {
            Action::ShortCastle => {
                // Short castle notation
                write!(f, "0-0")?;
            }
            Action::LongCastle => {
                // Long castle notation
                write!(f, "0-0-0")?;
            }
            _ => {
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
                // TODO: remove when not need and keep when ambiguous (short algebraic)
                write!(f, "{}", self.origin_square)?;

                // Captures
                match self.action {
                    Action::Capture | Action::EnPassant => {
                        write!(f, "x")?;
                    }
                    _ => {}
                }

                // Destination square
                write!(f, "{}", self.destination_square)?;
            }
        }

        // Extra notation for en passant (optional)
        match self.action {
            Action::EnPassant => {
                write!(f, " e.p.")?;
            }
            _ => {}
        }

        // TODO: add check(mate)
        // TODO: add promotions

        // All okay
        Ok(())
    }
}

// Custom type alias for vector of moves
pub type Moves<'a> = Vec<ChessMove<'a>>;
