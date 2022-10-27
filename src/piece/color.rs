// Absolute imports within crate
use crate::board::Rank;

#[derive(Debug, PartialEq)]
pub enum Color {
    Black,
    White,
}

impl Color {
    pub fn get_pawn_starting_rank(&self) -> Rank {
        match self {
            Color::White => 2,
            Color::Black => 7,
        }
    }

    // TODO: pub fn get_pawn_promotion_rank(&self) -> Rank
}
