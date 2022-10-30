// Absolute imports within crate
use crate::board::Rank;

#[derive(Debug, PartialEq, Clone)]
pub enum Color {
    Black,
    White,
}

impl Color {
    pub fn get_first_rank(&self) -> Rank {
        match self {
            Color::White => 1,
            Color::Black => 8,
        }
    }

    pub fn get_second_rank(&self) -> Rank {
        match self {
            Color::White => 2,
            Color::Black => 7,
        }
    }

    pub fn get_eight_rank(&self) -> Rank {
        match self {
            Color::White => 8,
            Color::Black => 1,
        }
    }
}
