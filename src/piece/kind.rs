#[derive(Debug, PartialEq)]
pub enum Kind {
    Bishop,
    King,
    Knight,
    Pawn,
    Queen,
    Rook,
}

impl Kind {
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
