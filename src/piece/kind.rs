// External imports
use std::fmt;

const PROMOTABLE_KINDS: [Kind; 4] = [Kind::Bishop, Kind::Knight, Kind::Rook, Kind::Queen];

#[derive(Debug, PartialEq, Clone)]
pub enum Kind {
    Bishop,
    King,
    Knight,
    Pawn,
    Queen,
    Rook,
}

impl Kind {
    pub fn get_promotable_kinds() -> [Kind; 4] {
        PROMOTABLE_KINDS
    }

    // Note: Relative value is arbitrary/subjective and is not part of the rule set of chess
    // Note: The standard valuation is 1,3,3,5,9 however I will use the valuation of AlphaZero
    // Source: https://arxiv.org/pdf/2009.04374.pdf
    pub fn relative_value(&self) -> f32 {
        match self {
            Kind::Pawn => 1.0,
            Kind::Knight => 3.05,
            Kind::Bishop => 3.33,
            Kind::Rook => 5.63,
            Kind::Queen => 9.5,
            Kind::King => f32::INFINITY,
        }
    }
}
impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Kind::Bishop => {
                write!(f, "B")
            }
            Kind::King => {
                write!(f, "K")
            }
            Kind::Knight => {
                write!(f, "N")
            }
            Kind::Pawn => {
                // Implicit
                Ok(())
            }
            Kind::Queen => {
                write!(f, "Q")
            }
            Kind::Rook => {
                write!(f, "R")
            }
        }
    }
}
